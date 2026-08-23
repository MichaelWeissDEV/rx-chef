//! Shared OpenPGP implementation used by the PGP operations.

#[cfg(feature = "pgp")]
mod enabled {
    use sequoia_openpgp as openpgp;
    use std::{collections::HashMap, io::Write};

    use openpgp::{
        armor,
        cert::{prelude::*, CipherSuite},
        crypto::{KeyPair, Password, SessionKey},
        parse::{stream::*, Parse},
        policy::{AsymmetricAlgorithm, StandardPolicy},
        serialize::{stream::*, SerializeInto},
        types::SymmetricAlgorithm,
        Cert, KeyID,
    };

    fn parse_cert(value: &str) -> openpgp::Result<Cert> {
        Cert::from_bytes(value.as_bytes())
    }

    fn compatibility_policy() -> StandardPolicy<'static> {
        let mut policy = StandardPolicy::new();
        // CyberChef supports reading legacy RSA-1024 OpenPGP material.  Keep
        // generation disabled, but permit interoperability with existing keys.
        policy.accept_asymmetric_algo(AsymmetricAlgorithm::RSA1024);
        policy
    }

    fn signing_key(cert: &Cert, password: &str) -> openpgp::Result<KeyPair> {
        let policy = &compatibility_policy();
        let key = cert
            .keys()
            .secret()
            .with_policy(policy, None)
            .supported()
            .alive()
            .revoked(false)
            .for_signing()
            .next()
            .ok_or_else(|| openpgp::anyhow::anyhow!("No signing-capable secret key"))?
            .key()
            .clone();
        let key = if key.secret().is_encrypted() {
            key.decrypt_secret(&Password::from(password))?
        } else {
            key
        };
        key.into_keypair()
    }

    pub fn generate(
        key_type: &str,
        password: &str,
        name: &str,
        email: &str,
    ) -> openpgp::Result<(Vec<u8>, Vec<u8>)> {
        let suite = match key_type {
            "RSA-2048" => CipherSuite::RSA2k,
            "RSA-4096" => CipherSuite::RSA4k,
            "ECC-256" => CipherSuite::P256,
            "ECC-384" => CipherSuite::P384,
            "ECC-521" => CipherSuite::P521,
            "RSA-1024" => {
                return Err(openpgp::anyhow::anyhow!(
                    "RSA-1024 key generation is intentionally disabled because it is insecure"
                ));
            }
            _ => return Err(openpgp::anyhow::anyhow!("Unsupported key type: {key_type}")),
        };
        let userid = match (name.is_empty(), email.is_empty()) {
            (false, false) => format!("{name} <{email}>"),
            (false, true) => name.to_string(),
            (true, false) => email.to_string(),
            (true, true) => "rxchef user".to_string(),
        };
        let builder = CertBuilder::new()
            .set_cipher_suite(suite)
            .add_userid(userid)
            .add_signing_subkey()
            .add_transport_encryption_subkey();
        let builder = if password.is_empty() {
            builder
        } else {
            builder.set_password(Some(Password::from(password)))
        };
        let (cert, _) = builder.generate()?;
        let public = cert.armored().to_vec()?;
        let private = cert.as_tsk().armored().to_vec()?;
        Ok((public, private))
    }

    pub fn encrypt(plaintext: &[u8], recipient: &str) -> openpgp::Result<Vec<u8>> {
        let cert = parse_cert(recipient)?;
        let policy = &compatibility_policy();
        let recipients = cert
            .keys()
            .with_policy(policy, None)
            .supported()
            .alive()
            .revoked(false)
            .for_transport_encryption();
        let mut output = Vec::new();
        let message = Message::new(&mut output);
        let message = Armorer::new(message).kind(armor::Kind::Message).build()?;
        let message = Encryptor::for_recipients(message, recipients).build()?;
        let mut message = LiteralWriter::new(message).build()?;
        message.write_all(plaintext)?;
        message.finalize()?;
        Ok(output)
    }

    pub fn sign(plaintext: &[u8], signer: &str, password: &str) -> openpgp::Result<Vec<u8>> {
        let cert = parse_cert(signer)?;
        let keypair = signing_key(&cert, password)?;
        let mut output = Vec::new();
        let message = Message::new(&mut output);
        let message = Armorer::new(message).kind(armor::Kind::Message).build()?;
        let message = Signer::new(message, keypair)?.build()?;
        let mut message = LiteralWriter::new(message).build()?;
        message.write_all(plaintext)?;
        message.finalize()?;
        Ok(output)
    }

    pub fn encrypt_and_sign(
        plaintext: &[u8],
        signer: &str,
        password: &str,
        recipient: &str,
    ) -> openpgp::Result<Vec<u8>> {
        let signer = parse_cert(signer)?;
        let recipient = parse_cert(recipient)?;
        let policy = &compatibility_policy();
        let recipients = recipient
            .keys()
            .with_policy(policy, None)
            .supported()
            .alive()
            .revoked(false)
            .for_transport_encryption();
        let keypair = signing_key(&signer, password)?;
        let mut output = Vec::new();
        let message = Message::new(&mut output);
        let message = Armorer::new(message).kind(armor::Kind::Message).build()?;
        let message = Encryptor::for_recipients(message, recipients).build()?;
        let message = Signer::new(message, keypair)?.build()?;
        let mut message = LiteralWriter::new(message).build()?;
        message.write_all(plaintext)?;
        message.finalize()?;
        Ok(output)
    }

    struct VerifyHelper {
        certs: Vec<Cert>,
        require_signature: bool,
    }

    impl VerificationHelper for VerifyHelper {
        fn get_certs(&mut self, _ids: &[openpgp::KeyHandle]) -> openpgp::Result<Vec<Cert>> {
            Ok(self.certs.clone())
        }

        fn check(&mut self, structure: MessageStructure) -> openpgp::Result<()> {
            let mut valid = false;
            for layer in structure {
                if let MessageLayer::SignatureGroup { results } = layer {
                    for result in results {
                        result.map_err(openpgp::Error::from)?;
                        valid = true;
                    }
                }
            }
            if self.require_signature && !valid {
                return Err(openpgp::anyhow::anyhow!("No valid OpenPGP signature"));
            }
            Ok(())
        }
    }

    pub fn verify(message: &[u8], signer: &str) -> openpgp::Result<Vec<u8>> {
        let cert = parse_cert(signer)?;
        let helper = VerifyHelper {
            certs: vec![cert],
            require_signature: true,
        };
        let policy = &compatibility_policy();
        let mut verifier =
            VerifierBuilder::from_bytes(message)?.with_policy(policy, None, helper)?;
        let mut output = Vec::new();
        std::io::copy(&mut verifier, &mut output)?;
        Ok(output)
    }

    struct DecryptHelper {
        keys: HashMap<KeyID, (Cert, KeyPair)>,
        verify: VerifyHelper,
    }

    impl DecryptHelper {
        fn new(secret: Cert, password: &str, verifier: Option<Cert>) -> openpgp::Result<Self> {
            let policy = &compatibility_policy();
            let mut keys = HashMap::new();
            for key in secret
                .keys()
                .secret()
                .with_policy(policy, None)
                .supported()
                .alive()
                .revoked(false)
                .for_transport_encryption()
            {
                let key = key.key().clone();
                let key = if key.secret().is_encrypted() {
                    key.decrypt_secret(&Password::from(password))?
                } else {
                    key
                };
                keys.insert(key.keyid(), (secret.clone(), key.into_keypair()?));
            }
            if keys.is_empty() {
                return Err(openpgp::anyhow::anyhow!("No decryption-capable secret key"));
            }
            Ok(Self {
                keys,
                verify: VerifyHelper {
                    certs: verifier.clone().into_iter().collect(),
                    require_signature: verifier.is_some(),
                },
            })
        }
    }

    impl VerificationHelper for DecryptHelper {
        fn get_certs(&mut self, ids: &[openpgp::KeyHandle]) -> openpgp::Result<Vec<Cert>> {
            self.verify.get_certs(ids)
        }

        fn check(&mut self, structure: MessageStructure) -> openpgp::Result<()> {
            self.verify.check(structure)
        }
    }

    impl DecryptionHelper for DecryptHelper {
        fn decrypt(
            &mut self,
            pkesks: &[openpgp::packet::PKESK],
            _skesks: &[openpgp::packet::SKESK],
            symmetric: Option<SymmetricAlgorithm>,
            decrypt: &mut dyn FnMut(Option<SymmetricAlgorithm>, &SessionKey) -> bool,
        ) -> openpgp::Result<Option<Cert>> {
            for pkesk in pkesks {
                if let Some((cert, keypair)) = self.keys.get_mut(&KeyID::from(pkesk.recipient())) {
                    if pkesk
                        .decrypt(keypair, symmetric)
                        .map(|(algorithm, session)| decrypt(algorithm, &session))
                        .unwrap_or(false)
                    {
                        return Ok(Some(cert.clone()));
                    }
                }
            }
            Ok(None)
        }
    }

    pub fn decrypt(
        message: &[u8],
        recipient: &str,
        password: &str,
        signer: Option<&str>,
    ) -> openpgp::Result<Vec<u8>> {
        let secret = parse_cert(recipient)?;
        let signer = signer.map(parse_cert).transpose()?;
        let helper = DecryptHelper::new(secret, password, signer)?;
        let policy = &compatibility_policy();
        let mut decryptor =
            DecryptorBuilder::from_bytes(message)?.with_policy(policy, None, helper)?;
        let mut output = Vec::new();
        std::io::copy(&mut decryptor, &mut output)?;
        Ok(output)
    }
}

#[cfg(feature = "pgp")]
pub use enabled::*;
