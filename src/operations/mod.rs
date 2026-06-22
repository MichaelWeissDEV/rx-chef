/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Generated operations registry for rxchef
 * License:     Apache-2.0
 * Description: Auto-generated registry of rxchef operations.
 * -----------------------------------------------------------------------------
 */

#[allow(dead_code)]

use crate::operation::Operation;

pub mod a1z26_cipher_decode;
pub mod a1z26_cipher_encode;
pub mod add;
pub mod add_line_numbers;
pub mod add_text_to_image;
pub mod adler32_checksum;
pub mod aes_decrypt;
pub mod aes_encrypt;
pub mod aes_key_unwrap;
pub mod aes_key_wrap;
pub mod affine_cipher_decode;
pub mod affine_cipher_encode;
pub mod alternating_caps;
pub mod amf_decode;
pub mod amf_encode;
pub mod analyse_hash;
pub mod analyse_uuid;
pub mod and;
pub mod argon2;
pub mod argon2_compare;
pub mod atbash_cipher;
pub mod avro_to_json;
pub mod bacon_cipher_decode;
pub mod bacon_cipher_encode;
pub mod bcrypt;
pub mod bcrypt_compare;
pub mod bcrypt_parse;
pub mod bifid_cipher_decode;
pub mod bifid_cipher_encode;
pub mod bit_shift_left;
pub mod bit_shift_right;
pub mod blake2b;
pub mod blake2s;
pub mod blake3;
pub mod blowfish_decrypt;
pub mod blowfish_encrypt;
pub mod blur_image;
pub mod bombe;
pub mod bson_deserialise;
pub mod bson_serialise;
pub mod bzip2_compress;
pub mod bzip2_decompress;
pub mod caesar_box_cipher;
pub mod caret_mdecode;
pub mod cartesian_product;
pub mod cbor_decode;
pub mod cbor_encode;
pub mod cetacean_cipher_decode;
pub mod cetacean_cipher_encode;
pub mod chacha;
pub mod change_ip_format;
pub mod chi_square;
pub mod cipher_saber2_decrypt;
pub mod cipher_saber2_encrypt;
pub mod citrix_ctx1_decode;
pub mod citrix_ctx1_encode;
pub mod cmac;
pub mod colossus;
pub mod comment;
pub mod compare_ctph_hashes;
pub mod compare_ssdeep_hashes;
pub mod conditional_jump;
pub mod contain_image;
pub mod convert_area;
pub mod convert_coordinate_format;
pub mod convert_data_units;
pub mod convert_distance;
pub mod convert_image_format;
pub mod convert_leet_speak;
pub mod convert_mass;
pub mod convert_speed;
pub mod convert_to_nato_alphabet;
pub mod count_occurrences;
pub mod cover_image;
pub mod crc32;
pub mod crc_checksum;
pub mod crop_image;
pub mod css_beautify;
pub mod css_minify;
pub mod css_selector;
pub mod csv_to_json;
pub mod ctph;
pub mod date_time_delta;
pub mod dechunk_http_response;
pub mod decode_netbios_name;
pub mod decode_text;
pub mod defang_ip_addresses;
pub mod defang_url;
pub mod derive_evp_key;
pub mod derive_hkdf_key;
pub mod derive_pbkdf2_key;
pub mod des_decrypt;
pub mod des_encrypt;
pub mod detect_file_type;
pub mod diff;
pub mod disassemble_arm;
pub mod disassemble_x86;
pub mod dither_image;
pub mod divide;
pub mod dns_over_https;
pub mod drop_bytes;
pub mod drop_nth_bytes;
pub mod ecdsa_sign;
pub mod ecdsa_signature_conversion;
pub mod ecdsa_verify;
pub mod elf_info;
pub mod encode_netbios_name;
pub mod encode_text;
pub mod enigma;
pub mod entropy;
pub mod escape_string;
pub mod escape_unicode_characters;
pub mod expand_alphabet_range;
pub mod extract_audio_metadata;
pub mod extract_dates;
pub mod extract_domains;
pub mod extract_email_addresses;
pub mod extract_exif;
pub mod extract_file_paths;
pub mod extract_files;
pub mod extract_hashes;
pub mod extract_id3;
pub mod extract_ip_addresses;
pub mod extract_lsb;
pub mod extract_mac_addresses;
pub mod extract_rgba;
pub mod extract_urls;
pub mod fang_url;
pub mod fernet_decrypt;
pub mod fernet_encrypt;
pub mod file_tree;
pub mod filter;
pub mod find_replace;
pub mod flask_session_decode;
pub mod flask_session_sign;
pub mod flask_session_verify;
pub mod fletcher16_checksum;
pub mod fletcher32_checksum;
pub mod fletcher64_checksum;
pub mod fletcher8_checksum;
pub mod flip_image;
pub mod fork;
pub mod format_mac_addresses;
pub mod frequency_distribution;
pub mod from_base;
pub mod from_base32;
pub mod from_base45;
pub mod from_base58;
pub mod from_base62;
pub mod from_base64;
pub mod from_base85;
pub mod from_base92;
pub mod from_bcd;
pub mod from_bech32;
pub mod from_binary;
pub mod from_braille;
pub mod from_case_insensitive_regex;
pub mod from_charcode;
pub mod from_decimal;
pub mod from_float;
pub mod from_hex;
pub mod from_hex_content;
pub mod from_hexdump;
pub mod from_html_entity;
pub mod from_message_pack;
pub mod from_modhex;
pub mod from_morse_code;
pub mod from_octal;
pub mod from_punycode;
pub mod from_quoted_printable;
pub mod from_unix_timestamp;
pub mod fuzzy_match;
pub mod generate_all_checksums;
pub mod generate_all_hashes;
pub mod generate_de_bruijn_sequence;
pub mod generate_ecdsa_key_pair;
pub mod generate_hotp;
pub mod generate_image;
pub mod generate_lorem_ipsum;
pub mod generate_pgp_key_pair;
pub mod generate_qr_code;
pub mod generate_rsa_key_pair;
pub mod generate_totp;
pub mod generate_uuid;
pub mod generic_code_beautify;
pub mod get_all_casings;
pub mod get_time;
pub mod gost_decrypt;
pub mod gost_encrypt;
pub mod gost_hash;
pub mod gost_key_unwrap;
pub mod gost_key_wrap;
pub mod gost_sign;
pub mod gost_verify;
pub mod group_ip_addresses;
pub mod gunzip;
pub mod gzip;
pub mod hamming_distance;
pub mod has160;
pub mod hassh_client_fingerprint;
pub mod hassh_server_fingerprint;
pub mod haversine_distance;
pub mod head;
pub mod heatmap_chart;
pub mod hex_density_chart;
pub mod hex_to_object_identifier;
pub mod hex_to_pem;
pub mod hmac;
pub mod html_to_text;
pub mod http_request;
pub mod image_brightness_contrast;
pub mod image_filter;
pub mod image_hue_saturation_lightness;
pub mod image_opacity;
pub mod index_of_coincidence;
pub mod invert_image;
pub mod ipv6_transition_addresses;
pub mod j_path_expression;
pub mod ja3_fingerprint;
pub mod ja3s_fingerprint;
pub mod ja4_fingerprint;
pub mod ja4_server_fingerprint;
pub mod java_script_beautify;
pub mod java_script_minify;
pub mod java_script_parser;
pub mod jq;
pub mod json_beautify;
pub mod json_minify;
pub mod json_to_csv;
pub mod json_to_yaml;
pub mod jsonata;
pub mod jump;
pub mod jwk_to_pem;
pub mod jwt_decode;
pub mod jwt_sign;
pub mod jwt_verify;
pub mod keccak;
pub mod label;
pub mod levenshtein_distance;
pub mod lm_hash;
pub mod lorenz;
pub mod ls47_decrypt;
pub mod ls47_encrypt;
pub mod luhn_checksum;
pub mod lz4_compress;
pub mod lz4_decompress;
pub mod lz_string_compress;
pub mod lz_string_decompress;
pub mod lzma_compress;
pub mod lzma_decompress;
pub mod lznt1_decompress;
pub mod magic;
pub mod md2;
pub mod md4;
pub mod md5;
pub mod md6;
pub mod mean;
pub mod median;
pub mod merge;
pub mod microsoft_script_decoder;
pub mod mime_decoding;
pub mod multiple_bombe;
pub mod multiply;
pub mod murmur_hash3;
pub mod normalise_image;
pub mod normalise_unicode;
pub mod not;
pub mod nt_hash;
pub mod numberwang;
pub mod object_identifier_to_hex;
pub mod offset_checker;
pub mod optical_character_recognition;
pub mod or;
pub mod pad_lines;
pub mod parity_bit;
pub mod parse_asn1_hex_string;
pub mod parse_colour_code;
pub mod parse_csr;
pub mod parse_date_time;
pub mod parse_ethernet_frame;
pub mod parse_ip_range;
pub mod parse_ipv4_header;
pub mod parse_ipv6_address;
pub mod parse_object_id_timestamp;
pub mod parse_qr_code;
pub mod parse_ssh_host_key;
pub mod parse_tcp;
pub mod parse_tls_record;
pub mod parse_tlv;
pub mod parse_udp;
pub mod parse_unix_file_permissions;
pub mod parse_uri;
pub mod parse_user_agent;
pub mod parse_x509_certificate;
pub mod parse_x509_crl;
pub mod pem_to_hex;
pub mod pem_to_jwk;
pub mod pgp_decrypt;
pub mod pgp_decrypt_and_verify;
pub mod pgp_encrypt;
pub mod pgp_encrypt_and_sign;
pub mod pgp_verify;
pub mod php_deserialize;
pub mod php_serialize;
pub mod play_media;
pub mod plist_viewer;
pub mod power_set;
pub mod protobuf_decode;
pub mod protobuf_encode;
pub mod pseudo_random_integer_generator;
pub mod pseudo_random_number_generator;
pub mod pub_key_from_cert;
pub mod pub_key_from_priv_key;
pub mod rabbit;
pub mod rail_fence_cipher_decode;
pub mod rail_fence_cipher_encode;
pub mod rake;
pub mod randomize_colour_palette;
pub mod raw_deflate;
pub mod raw_inflate;
pub mod rc2_decrypt;
pub mod rc2_encrypt;
pub mod rc4;
pub mod rc4_drop;
pub mod rc6_decrypt;
pub mod rc6_encrypt;
pub mod register;
pub mod regular_expression;
pub mod remove_diacritics;
pub mod remove_exif;
pub mod remove_line_numbers;
pub mod remove_null_bytes;
pub mod remove_whitespace;
pub mod render_image;
pub mod render_markdown;
pub mod resize_image;
pub mod return_op;
pub mod reverse;
pub mod ripemd;
pub mod rison_decode;
pub mod rison_encode;
pub mod rot13;
pub mod rot13_brute_force;
pub mod rot47;
pub mod rot47_brute_force;
pub mod rot8000;
pub mod rotate_image;
pub mod rotate_left;
pub mod rotate_right;
pub mod rsa_decrypt;
pub mod rsa_encrypt;
pub mod rsa_sign;
pub mod rsa_verify;
pub mod salsa20;
pub mod scan_for_embedded_files;
pub mod scatter_chart;
pub mod scrypt;
pub mod series_chart;
pub mod set_difference;
pub mod set_intersection;
pub mod set_union;
pub mod sha0;
pub mod sha1;
pub mod sha2;
pub mod sha3;
pub mod shake;
pub mod sharpen_image;
pub mod show_base64_offsets;
pub mod show_on_map;
pub mod shuffle;
pub mod sigaba;
pub mod sleep;
pub mod sm2_decrypt;
pub mod sm2_encrypt;
pub mod sm3;
pub mod sm4_decrypt;
pub mod sm4_encrypt;
pub mod snefru;
pub mod sort;
pub mod split;
pub mod split_colour_channels;
pub mod sql_beautify;
pub mod sql_minify;
pub mod ssdeep;
pub mod standard_deviation;
pub mod streebog;
pub mod strings;
pub mod strip_html_tags;
pub mod strip_http_headers;
pub mod strip_ipv4_header;
pub mod strip_tcp_header;
pub mod strip_udp_header;
pub mod sub;
pub mod subsection;
pub mod substitute;
pub mod subtract;
pub mod sum;
pub mod swap_case;
pub mod swap_endianness;
pub mod symmetric_difference;
pub mod syntax_highlighter;
pub mod tail;
pub mod take_bytes;
pub mod take_nth_bytes;
pub mod tar;
pub mod tcpip_checksum;
pub mod template;
pub mod test_x509;
pub mod text_encoding_brute_force;
pub mod text_integer_converter;
pub mod to_base;
pub mod to_base32;
pub mod to_base45;
pub mod to_base58;
pub mod to_base62;
pub mod to_base64;
pub mod to_base85;
pub mod to_base92;
pub mod to_bcd;
pub mod to_bech32;
pub mod to_binary;
pub mod to_braille;
pub mod to_camel_case;
pub mod to_case_insensitive_regex;
pub mod to_charcode;
pub mod to_decimal;
pub mod to_float;
pub mod to_hex;
pub mod to_hex_content;
pub mod to_hexdump;
pub mod to_html_entity;
pub mod to_kebab_case;
pub mod to_lower_case;
pub mod to_message_pack;
pub mod to_modhex;
pub mod to_morse_code;
pub mod to_octal;
pub mod to_punycode;
pub mod to_quoted_printable;
pub mod to_snake_case;
pub mod to_table;
pub mod to_unix_timestamp;
pub mod to_upper_case;
pub mod translate_date_time_format;
pub mod triple_des_decrypt;
pub mod triple_des_encrypt;
pub mod typex;
pub mod unescape_string;
pub mod unescape_unicode_characters;
pub mod unicode_text_format;
pub mod unique;
pub mod unix_timestamp_to_windows_filetime;
pub mod untar;
pub mod unzip;
pub mod url_decode;
pub mod url_encode;
pub mod varint_decode;
pub mod varint_encode;
pub mod view_bit_plane;
pub mod vigenere_decode;
pub mod vigenere_encode;
pub mod whirlpool;
pub mod windows_filetime_to_unix_timestamp;
pub mod wrap;
pub mod x_path_expression;
pub mod x_salsa20;
pub mod xkcd_random_number;
pub mod xml_beautify;
pub mod xml_minify;
pub mod xor;
pub mod xor_brute_force;
pub mod xor_checksum;
pub mod xxtea_decrypt;
pub mod xxtea_encrypt;
pub mod yaml_to_json;
pub mod yara_rules;
pub mod zip;
pub mod zlib_deflate;
pub mod zlib_inflate;

pub fn operation_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    names.push(a1z26_cipher_decode::A1Z26CipherDecode.name().to_string());
    names.push(a1z26_cipher_encode::A1Z26CipherEncode.name().to_string());
    names.push(add::AddOp.name().to_string());
    names.push(add_line_numbers::AddLineNumbers.name().to_string());
    names.push(add_text_to_image::AddTextToImage.name().to_string());
    names.push(adler32_checksum::Adler32Checksum.name().to_string());
    names.push(aes_decrypt::AesDecrypt.name().to_string());
    names.push(aes_encrypt::AesEncrypt.name().to_string());
    names.push(aes_key_unwrap::AesKeyUnwrap.name().to_string());
    names.push(aes_key_wrap::AesKeyWrap.name().to_string());
    names.push(affine_cipher_decode::AffineCipherDecode.name().to_string());
    names.push(affine_cipher_encode::AffineCipherEncode.name().to_string());
    names.push(alternating_caps::AlternatingCaps.name().to_string());
    names.push(amf_decode::AmfDecode.name().to_string());
    names.push(amf_encode::AmfEncode.name().to_string());
    names.push(analyse_hash::AnalyseHash.name().to_string());
    names.push(analyse_uuid::AnalyseUUID.name().to_string());
    names.push(and::AndOp.name().to_string());
    names.push(argon2::Argon2.name().to_string());
    names.push(argon2_compare::Argon2Compare.name().to_string());
    names.push(atbash_cipher::AtbashCipher.name().to_string());
    names.push(avro_to_json::AvroToJSON.name().to_string());
    names.push(bacon_cipher_decode::BaconCipherDecode.name().to_string());
    names.push(bacon_cipher_encode::BaconCipherEncode.name().to_string());
    names.push(bcrypt::Bcrypt.name().to_string());
    names.push(bcrypt_compare::BcryptCompare.name().to_string());
    names.push(bcrypt_parse::BcryptParse.name().to_string());
    names.push(bifid_cipher_decode::BifidCipherDecode.name().to_string());
    names.push(bifid_cipher_encode::BifidCipherEncode.name().to_string());
    names.push(bit_shift_left::BitShiftLeft.name().to_string());
    names.push(bit_shift_right::BitShiftRight.name().to_string());
    names.push(blake2b::BLAKE2b.name().to_string());
    names.push(blake2s::BLAKE2s.name().to_string());
    names.push(blake3::BLAKE3.name().to_string());
    names.push(blowfish_decrypt::BlowfishDecrypt.name().to_string());
    names.push(blowfish_encrypt::BlowfishEncrypt.name().to_string());
    names.push(blur_image::BlurImage.name().to_string());
    names.push(bombe::Bombe.name().to_string());
    names.push(bson_deserialise::BsonDeserialise.name().to_string());
    names.push(bson_serialise::BsonSerialise.name().to_string());
    names.push(bzip2_compress::Bzip2Compress.name().to_string());
    names.push(bzip2_decompress::Bzip2Decompress.name().to_string());
    names.push(caesar_box_cipher::CaesarBoxCipher.name().to_string());
    names.push(caret_mdecode::CaretMdecode.name().to_string());
    names.push(cartesian_product::CartesianProduct.name().to_string());
    names.push(cbor_decode::CBORDecode.name().to_string());
    names.push(cbor_encode::CBOREncode.name().to_string());
    names.push(cetacean_cipher_decode::CetaceanCipherDecode.name().to_string());
    names.push(cetacean_cipher_encode::CetaceanCipherEncode.name().to_string());
    names.push(chacha::ChaCha.name().to_string());
    names.push(change_ip_format::ChangeIPFormat.name().to_string());
    names.push(chi_square::ChiSquare.name().to_string());
    names.push(cipher_saber2_decrypt::CipherSaber2Decrypt.name().to_string());
    names.push(cipher_saber2_encrypt::CipherSaber2Encrypt.name().to_string());
    names.push(citrix_ctx1_decode::CitrixCtx1Decode.name().to_string());
    names.push(citrix_ctx1_encode::CitrixCtx1Encode.name().to_string());
    names.push(cmac::Cmac.name().to_string());
    names.push(colossus::Colossus.name().to_string());
    names.push(comment::CommentOp.name().to_string());
    names.push(compare_ctph_hashes::CompareCTPHHashes.name().to_string());
    names.push(compare_ssdeep_hashes::CompareSSDEEPHashes.name().to_string());
    names.push(conditional_jump::ConditionalJump.name().to_string());
    names.push(contain_image::ContainImage.name().to_string());
    names.push(convert_area::ConvertArea.name().to_string());
    names.push(convert_coordinate_format::ConvertCoordinateFormat.name().to_string());
    names.push(convert_data_units::ConvertDataUnits.name().to_string());
    names.push(convert_distance::ConvertDistance.name().to_string());
    names.push(convert_image_format::ConvertImageFormat.name().to_string());
    names.push(convert_leet_speak::ConvertLeetSpeak.name().to_string());
    names.push(convert_mass::ConvertMass.name().to_string());
    names.push(convert_speed::ConvertSpeed.name().to_string());
    names.push(convert_to_nato_alphabet::ConvertToNATOAlphabet.name().to_string());
    names.push(count_occurrences::CountOccurrences.name().to_string());
    names.push(cover_image::CoverImage.name().to_string());
    names.push(crc32::CRC32.name().to_string());
    names.push(crc_checksum::CrcChecksum.name().to_string());
    names.push(crop_image::CropImage.name().to_string());
    names.push(css_beautify::CssBeautify.name().to_string());
    names.push(css_minify::CssMinify.name().to_string());
    names.push(css_selector::CssSelector.name().to_string());
    names.push(csv_to_json::CsvToJson.name().to_string());
    names.push(ctph::CTPH.name().to_string());
    names.push(date_time_delta::DateTimeDelta.name().to_string());
    names.push(dechunk_http_response::DechunkHttpResponse.name().to_string());
    names.push(decode_netbios_name::DecodeNetBIOSName.name().to_string());
    names.push(decode_text::DecodeText.name().to_string());
    names.push(defang_ip_addresses::DefangIPAddresses.name().to_string());
    names.push(defang_url::DefangURL.name().to_string());
    names.push(derive_evp_key::DeriveEvpKey.name().to_string());
    names.push(derive_hkdf_key::DeriveHKDFKey.name().to_string());
    names.push(derive_pbkdf2_key::DerivePBKDF2Key.name().to_string());
    names.push(des_decrypt::DesDecrypt.name().to_string());
    names.push(des_encrypt::DesEncrypt.name().to_string());
    names.push(detect_file_type::DetectFileType.name().to_string());
    names.push(diff::Diff.name().to_string());
    names.push(disassemble_arm::DisassembleArm.name().to_string());
    names.push(disassemble_x86::DisassembleX86.name().to_string());
    names.push(dither_image::DitherImage.name().to_string());
    names.push(divide::Divide.name().to_string());
    names.push(dns_over_https::DnsOverHttps.name().to_string());
    names.push(drop_bytes::DropBytes.name().to_string());
    names.push(drop_nth_bytes::DropNthBytes.name().to_string());
    names.push(ecdsa_sign::ECDSASign.name().to_string());
    names.push(ecdsa_signature_conversion::ECDSASignatureConversion.name().to_string());
    names.push(ecdsa_verify::ECDSAVerify.name().to_string());
    names.push(elf_info::ELFInfo.name().to_string());
    names.push(encode_netbios_name::EncodeNetBIOSName.name().to_string());
    names.push(encode_text::EncodeText.name().to_string());
    names.push(enigma::Enigma.name().to_string());
    names.push(entropy::Entropy.name().to_string());
    names.push(escape_string::EscapeString.name().to_string());
    names.push(escape_unicode_characters::EscapeUnicodeCharacters.name().to_string());
    names.push(expand_alphabet_range::ExpandAlphabetRange.name().to_string());
    names.push(extract_audio_metadata::ExtractAudioMetadata.name().to_string());
    names.push(extract_dates::ExtractDates.name().to_string());
    names.push(extract_domains::ExtractDomains.name().to_string());
    names.push(extract_email_addresses::ExtractEmailAddresses.name().to_string());
    names.push(extract_exif::ExtractEXIF.name().to_string());
    names.push(extract_file_paths::ExtractFilePaths.name().to_string());
    names.push(extract_files::ExtractFiles.name().to_string());
    names.push(extract_hashes::ExtractHashes.name().to_string());
    names.push(extract_id3::ExtractID3.name().to_string());
    names.push(extract_ip_addresses::ExtractIPAddresses.name().to_string());
    names.push(extract_lsb::ExtractLSB.name().to_string());
    names.push(extract_mac_addresses::ExtractMACAddresses.name().to_string());
    names.push(extract_rgba::ExtractRGBA.name().to_string());
    names.push(extract_urls::ExtractURLs.name().to_string());
    names.push(fang_url::FangURL.name().to_string());
    names.push(fernet_decrypt::FernetDecrypt.name().to_string());
    names.push(fernet_encrypt::FernetEncrypt.name().to_string());
    names.push(file_tree::FileTree.name().to_string());
    names.push(filter::Filter.name().to_string());
    names.push(find_replace::FindReplace.name().to_string());
    names.push(flask_session_decode::FlaskSessionDecode.name().to_string());
    names.push(flask_session_sign::FlaskSessionSign.name().to_string());
    names.push(flask_session_verify::FlaskSessionVerify.name().to_string());
    names.push(fletcher16_checksum::Fletcher16Checksum.name().to_string());
    names.push(fletcher32_checksum::Fletcher32Checksum.name().to_string());
    names.push(fletcher64_checksum::Fletcher64Checksum.name().to_string());
    names.push(fletcher8_checksum::Fletcher8Checksum.name().to_string());
    names.push(flip_image::FlipImage.name().to_string());
    names.push(fork::Fork.name().to_string());
    names.push(format_mac_addresses::FormatMACAddresses.name().to_string());
    names.push(frequency_distribution::FrequencyDistribution.name().to_string());
    names.push(from_base::FromBase.name().to_string());
    names.push(from_base32::FromBase32.name().to_string());
    names.push(from_base45::FromBase45.name().to_string());
    names.push(from_base58::FromBase58.name().to_string());
    names.push(from_base62::FromBase62.name().to_string());
    names.push(from_base64::FromBase64.name().to_string());
    names.push(from_base85::FromBase85.name().to_string());
    names.push(from_base92::FromBase92.name().to_string());
    names.push(from_bcd::FromBCD.name().to_string());
    names.push(from_bech32::FromBech32.name().to_string());
    names.push(from_binary::FromBinary.name().to_string());
    names.push(from_braille::FromBraille.name().to_string());
    names.push(from_case_insensitive_regex::FromCaseInsensitiveRegex.name().to_string());
    names.push(from_charcode::FromCharcode.name().to_string());
    names.push(from_decimal::FromDecimal.name().to_string());
    names.push(from_float::FromFloat.name().to_string());
    names.push(from_hex::FromHex.name().to_string());
    names.push(from_hex_content::FromHexContent.name().to_string());
    names.push(from_hexdump::FromHexdump.name().to_string());
    names.push(from_html_entity::FromHTMLEntity.name().to_string());
    names.push(from_message_pack::FromMessagePack.name().to_string());
    names.push(from_modhex::FromModhex.name().to_string());
    names.push(from_morse_code::FromMorseCode.name().to_string());
    names.push(from_octal::FromOctal.name().to_string());
    names.push(from_punycode::FromPunycode.name().to_string());
    names.push(from_quoted_printable::FromQuotedPrintable.name().to_string());
    names.push(from_unix_timestamp::FromUNIXTimestamp.name().to_string());
    names.push(fuzzy_match::FuzzyMatch.name().to_string());
    names.push(generate_all_checksums::GenerateAllChecksums.name().to_string());
    names.push(generate_all_hashes::GenerateAllHashes.name().to_string());
    names.push(generate_de_bruijn_sequence::GenerateDeBruijnSequence.name().to_string());
    names.push(generate_ecdsa_key_pair::GenerateECDSAKeyPairOp.name().to_string());
    names.push(generate_hotp::GenerateHOTPOp.name().to_string());
    names.push(generate_image::GenerateImageOp.name().to_string());
    names.push(generate_lorem_ipsum::GenerateLoremIpsum.name().to_string());
    names.push(generate_pgp_key_pair::GeneratePGPKeyPair.name().to_string());
    names.push(generate_qr_code::GenerateQRCodeOp.name().to_string());
    names.push(generate_rsa_key_pair::GenerateRSAKeyPair.name().to_string());
    names.push(generate_totp::GenerateTOTP.name().to_string());
    names.push(generate_uuid::GenerateUUID.name().to_string());
    names.push(generic_code_beautify::GenericCodeBeautify.name().to_string());
    names.push(get_all_casings::GetAllCasings.name().to_string());
    names.push(get_time::GetTime.name().to_string());
    names.push(gost_decrypt::GOSTDecryptOp.name().to_string());
    names.push(gost_encrypt::GostEncrypt.name().to_string());
    names.push(gost_hash::GostHash.name().to_string());
    names.push(gost_key_unwrap::GOSTKeyUnwrapOp.name().to_string());
    names.push(gost_key_wrap::GostKeyWrap.name().to_string());
    names.push(gost_sign::GostSign.name().to_string());
    names.push(gost_verify::GOSTVerifyOp.name().to_string());
    names.push(group_ip_addresses::GroupIPAddresses.name().to_string());
    names.push(gunzip::Gunzip.name().to_string());
    names.push(gzip::Gzip.name().to_string());
    names.push(hamming_distance::HammingDistance.name().to_string());
    names.push(has160::HAS160Op.name().to_string());
    names.push(hassh_client_fingerprint::HASSHClientFingerprint.name().to_string());
    names.push(hassh_server_fingerprint::HASSHServerFingerprint.name().to_string());
    names.push(haversine_distance::HaversineDistance.name().to_string());
    names.push(head::Head.name().to_string());
    names.push(heatmap_chart::HeatmapChart.name().to_string());
    names.push(hex_density_chart::HexDensityChartOp.name().to_string());
    names.push(hex_to_object_identifier::HexToObjectIdentifier.name().to_string());
    names.push(hex_to_pem::HexToPEM.name().to_string());
    names.push(hmac::HMAC.name().to_string());
    names.push(html_to_text::HTMLToText.name().to_string());
    names.push(http_request::HTTPRequest.name().to_string());
    names.push(image_brightness_contrast::ImageBrightnessContrast.name().to_string());
    names.push(image_filter::ImageFilter.name().to_string());
    names.push(image_hue_saturation_lightness::ImageHueSaturationLightness.name().to_string());
    names.push(image_opacity::ImageOpacity.name().to_string());
    names.push(index_of_coincidence::IndexOfCoincidence.name().to_string());
    names.push(invert_image::InvertImage.name().to_string());
    names.push(ipv6_transition_addresses::IPv6TransitionAddresses.name().to_string());
    names.push(j_path_expression::JPathExpression.name().to_string());
    names.push(ja3_fingerprint::JA3Fingerprint.name().to_string());
    names.push(ja3s_fingerprint::JA3SFingerprint.name().to_string());
    names.push(ja4_fingerprint::JA4Fingerprint.name().to_string());
    names.push(ja4_server_fingerprint::JA4ServerFingerprint.name().to_string());
    names.push(java_script_beautify::JavaScriptBeautify.name().to_string());
    names.push(java_script_minify::JavaScriptMinify.name().to_string());
    names.push(java_script_parser::JavaScriptParser.name().to_string());
    names.push(jq::Jq.name().to_string());
    names.push(json_beautify::JSONBeautify.name().to_string());
    names.push(json_minify::JSONMinify.name().to_string());
    names.push(json_to_csv::JSONToCSV.name().to_string());
    names.push(json_to_yaml::JSONToYAML.name().to_string());
    names.push(jsonata::Jsonata.name().to_string());
    names.push(jump::Jump.name().to_string());
    names.push(jwk_to_pem::JWKToPem.name().to_string());
    names.push(jwt_decode::JWTDecode.name().to_string());
    names.push(jwt_sign::JWTSign.name().to_string());
    names.push(jwt_verify::JWTVerify.name().to_string());
    names.push(keccak::Keccak.name().to_string());
    names.push(label::Label.name().to_string());
    names.push(levenshtein_distance::LevenshteinDistance.name().to_string());
    names.push(lm_hash::LMHash.name().to_string());
    names.push(lorenz::Lorenz.name().to_string());
    names.push(ls47_decrypt::LS47Decrypt.name().to_string());
    names.push(ls47_encrypt::LS47Encrypt.name().to_string());
    names.push(luhn_checksum::LuhnChecksum.name().to_string());
    names.push(lz4_compress::LZ4Compress.name().to_string());
    names.push(lz4_decompress::LZ4Decompress.name().to_string());
    names.push(lz_string_compress::LZStringCompress.name().to_string());
    names.push(lz_string_decompress::LZStringDecompress.name().to_string());
    names.push(lzma_compress::LZMACompress.name().to_string());
    names.push(lzma_decompress::LZMADecompress.name().to_string());
    names.push(lznt1_decompress::LZNT1Decompress.name().to_string());
    names.push(magic::Magic.name().to_string());
    names.push(md2::MD2.name().to_string());
    names.push(md4::MD4.name().to_string());
    names.push(md5::MD5.name().to_string());
    names.push(md6::MD6.name().to_string());
    names.push(mean::Mean.name().to_string());
    names.push(median::Median.name().to_string());
    names.push(merge::Merge.name().to_string());
    names.push(microsoft_script_decoder::MicrosoftScriptDecoder.name().to_string());
    names.push(mime_decoding::MIMEDecoding.name().to_string());
    names.push(multiple_bombe::MultipleBombe.name().to_string());
    names.push(multiply::Multiply.name().to_string());
    names.push(murmur_hash3::MurmurHash3.name().to_string());
    names.push(normalise_image::NormaliseImage.name().to_string());
    names.push(normalise_unicode::NormaliseUnicode.name().to_string());
    names.push(not::NOT.name().to_string());
    names.push(nt_hash::NTHash.name().to_string());
    names.push(numberwang::Numberwang.name().to_string());
    names.push(object_identifier_to_hex::ObjectIdentifierToHex.name().to_string());
    names.push(offset_checker::OffsetChecker.name().to_string());
    names.push(optical_character_recognition::OpticalCharacterRecognition.name().to_string());
    names.push(or::OrOp.name().to_string());
    names.push(pad_lines::PadLines.name().to_string());
    names.push(parity_bit::ParityBit.name().to_string());
    names.push(parse_asn1_hex_string::ParseASN1HexString.name().to_string());
    names.push(parse_colour_code::ParseColourCode.name().to_string());
    names.push(parse_csr::ParseCSR.name().to_string());
    names.push(parse_date_time::ParseDateTime.name().to_string());
    names.push(parse_ethernet_frame::ParseEthernetFrame.name().to_string());
    names.push(parse_ip_range::ParseIPRange.name().to_string());
    names.push(parse_ipv4_header::ParseIPv4Header.name().to_string());
    names.push(parse_ipv6_address::ParseIPv6Address.name().to_string());
    names.push(parse_object_id_timestamp::ParseObjectIDTimestamp.name().to_string());
    names.push(parse_qr_code::ParseQRCode.name().to_string());
    names.push(parse_ssh_host_key::ParseSshHostKey.name().to_string());
    names.push(parse_tcp::ParseTcp.name().to_string());
    names.push(parse_tls_record::ParseTLSRecord.name().to_string());
    names.push(parse_tlv::ParseTLV.name().to_string());
    names.push(parse_udp::ParseUDP.name().to_string());
    names.push(parse_unix_file_permissions::ParseUNIXFilePermissions.name().to_string());
    names.push(parse_uri::ParseURI.name().to_string());
    names.push(parse_user_agent::ParseUserAgent.name().to_string());
    names.push(parse_x509_certificate::ParseX509Certificate.name().to_string());
    names.push(parse_x509_crl::ParseX509CRL.name().to_string());
    names.push(pem_to_hex::PEMToHex.name().to_string());
    names.push(pem_to_jwk::PEMToJWK.name().to_string());
    names.push(pgp_decrypt::PGPDecrypt.name().to_string());
    names.push(pgp_decrypt_and_verify::PGPDecryptAndVerify.name().to_string());
    names.push(pgp_encrypt::PGPEncrypt.name().to_string());
    names.push(pgp_encrypt_and_sign::PGPEncryptAndSign.name().to_string());
    names.push(pgp_verify::PGPVerify.name().to_string());
    names.push(php_deserialize::PHPDeserialize.name().to_string());
    names.push(php_serialize::PHPSerialize.name().to_string());
    names.push(play_media::PlayMedia.name().to_string());
    names.push(plist_viewer::PLISTViewer.name().to_string());
    names.push(power_set::PowerSet.name().to_string());
    names.push(protobuf_decode::ProtobufDecode.name().to_string());
    names.push(protobuf_encode::ProtobufEncode.name().to_string());
    names.push(pseudo_random_integer_generator::PseudoRandomIntegerGenerator.name().to_string());
    names.push(pseudo_random_number_generator::PseudoRandomNumberGenerator.name().to_string());
    names.push(pub_key_from_cert::PubKeyFromCert.name().to_string());
    names.push(pub_key_from_priv_key::PubKeyFromPrivKeyOp.name().to_string());
    names.push(rabbit::RabbitOp.name().to_string());
    names.push(rail_fence_cipher_decode::RailFenceCipherDecode.name().to_string());
    names.push(rail_fence_cipher_encode::RailFenceCipherEncode.name().to_string());
    names.push(rake::RAKE.name().to_string());
    names.push(randomize_colour_palette::RandomizeColourPalette.name().to_string());
    names.push(raw_deflate::RawDeflate.name().to_string());
    names.push(raw_inflate::RawInflate.name().to_string());
    names.push(rc2_decrypt::RC2Decrypt.name().to_string());
    names.push(rc2_encrypt::RC2Encrypt.name().to_string());
    names.push(rc4::RC4.name().to_string());
    names.push(rc4_drop::RC4Drop.name().to_string());
    names.push(rc6_decrypt::RC6Decrypt.name().to_string());
    names.push(rc6_encrypt::RC6Encrypt.name().to_string());
    names.push(register::Register.name().to_string());
    names.push(regular_expression::RegularExpressionOp.name().to_string());
    names.push(remove_diacritics::RemoveDiacritics.name().to_string());
    names.push(remove_exif::RemoveEXIF.name().to_string());
    names.push(remove_line_numbers::RemoveLineNumbers.name().to_string());
    names.push(remove_null_bytes::RemoveNullBytes.name().to_string());
    names.push(remove_whitespace::RemoveWhitespace.name().to_string());
    names.push(render_image::RenderImageOp.name().to_string());
    names.push(render_markdown::RenderMarkdown.name().to_string());
    names.push(resize_image::ResizeImage.name().to_string());
    names.push(return_op::ReturnOp.name().to_string());
    names.push(reverse::Reverse.name().to_string());
    names.push(ripemd::RIPEMD.name().to_string());
    names.push(rison_decode::RisonDecode.name().to_string());
    names.push(rison_encode::RisonEncode.name().to_string());
    names.push(rot13::ROT13.name().to_string());
    names.push(rot13_brute_force::ROT13BruteForce.name().to_string());
    names.push(rot47::ROT47.name().to_string());
    names.push(rot47_brute_force::ROT47BruteForce.name().to_string());
    names.push(rot8000::ROT8000.name().to_string());
    names.push(rotate_image::RotateImage.name().to_string());
    names.push(rotate_left::RotateLeft.name().to_string());
    names.push(rotate_right::RotateRight.name().to_string());
    names.push(rsa_decrypt::RSADecrypt.name().to_string());
    names.push(rsa_encrypt::RSAEncrypt.name().to_string());
    names.push(rsa_sign::RSASign.name().to_string());
    names.push(rsa_verify::RSAVerify.name().to_string());
    names.push(salsa20::Salsa20Op.name().to_string());
    names.push(scan_for_embedded_files::ScanForEmbeddedFiles.name().to_string());
    names.push(scatter_chart::ScatterChart.name().to_string());
    names.push(scrypt::ScryptOp.name().to_string());
    names.push(series_chart::SeriesChart.name().to_string());
    names.push(set_difference::SetDifference.name().to_string());
    names.push(set_intersection::SetIntersection.name().to_string());
    names.push(set_union::SetUnion.name().to_string());
    names.push(sha0::SHA0.name().to_string());
    names.push(sha1::SHA1.name().to_string());
    names.push(sha2::SHA2.name().to_string());
    names.push(sha3::SHA3.name().to_string());
    names.push(shake::SHAKE.name().to_string());
    names.push(sharpen_image::SharpenImage.name().to_string());
    names.push(show_base64_offsets::ShowBase64Offsets.name().to_string());
    names.push(show_on_map::ShowOnMap.name().to_string());
    names.push(shuffle::Shuffle.name().to_string());
    names.push(sigaba::SigabaOp.name().to_string());
    names.push(sleep::Sleep.name().to_string());
    names.push(sm2_decrypt::Sm2Decrypt.name().to_string());
    names.push(sm2_encrypt::Sm2Encrypt.name().to_string());
    names.push(sm3::SM3.name().to_string());
    names.push(sm4_decrypt::Sm4Decrypt.name().to_string());
    names.push(sm4_encrypt::Sm4Encrypt.name().to_string());
    names.push(snefru::SNEFRU.name().to_string());
    names.push(sort::Sort.name().to_string());
    names.push(split::Split.name().to_string());
    names.push(split_colour_channels::SplitColourChannels.name().to_string());
    names.push(sql_beautify::SQLBeautify.name().to_string());
    names.push(sql_minify::SQLMinify.name().to_string());
    names.push(ssdeep::SSDEEP.name().to_string());
    names.push(standard_deviation::StandardDeviation.name().to_string());
    names.push(streebog::Streebog.name().to_string());
    names.push(strings::Strings.name().to_string());
    names.push(strip_html_tags::StripHTMLTags.name().to_string());
    names.push(strip_http_headers::StripHTTPHeaders.name().to_string());
    names.push(strip_ipv4_header::StripIPv4Header.name().to_string());
    names.push(strip_tcp_header::StripTCPHeader.name().to_string());
    names.push(strip_udp_header::StripUDPHeader.name().to_string());
    names.push(sub::SUB.name().to_string());
    names.push(subsection::Subsection.name().to_string());
    names.push(substitute::Substitute.name().to_string());
    names.push(subtract::Subtract.name().to_string());
    names.push(sum::Sum.name().to_string());
    names.push(swap_case::SwapCase.name().to_string());
    names.push(swap_endianness::SwapEndianness.name().to_string());
    names.push(symmetric_difference::SymmetricDifference.name().to_string());
    names.push(syntax_highlighter::SyntaxHighlighter.name().to_string());
    names.push(tail::Tail.name().to_string());
    names.push(take_bytes::TakeBytes.name().to_string());
    names.push(take_nth_bytes::TakeNthBytes.name().to_string());
    names.push(tar::Tar.name().to_string());
    names.push(tcpip_checksum::TCPIPChecksum.name().to_string());
    names.push(template::Template.name().to_string());
    names.push(text_encoding_brute_force::TextEncodingBruteForce.name().to_string());
    names.push(text_integer_converter::TextIntegerConverter.name().to_string());
    names.push(to_base::ToBase.name().to_string());
    names.push(to_base32::ToBase32.name().to_string());
    names.push(to_base45::ToBase45.name().to_string());
    names.push(to_base58::ToBase58.name().to_string());
    names.push(to_base62::ToBase62.name().to_string());
    names.push(to_base64::ToBase64.name().to_string());
    names.push(to_base85::ToBase85.name().to_string());
    names.push(to_base92::ToBase92.name().to_string());
    names.push(to_bcd::ToBCD.name().to_string());
    names.push(to_bech32::ToBech32.name().to_string());
    names.push(to_binary::ToBinary.name().to_string());
    names.push(to_braille::ToBraille.name().to_string());
    names.push(to_camel_case::ToCamelCase.name().to_string());
    names.push(to_case_insensitive_regex::ToCaseInsensitiveRegex.name().to_string());
    names.push(to_charcode::ToCharcode.name().to_string());
    names.push(to_decimal::ToDecimal.name().to_string());
    names.push(to_float::ToFloat.name().to_string());
    names.push(to_hex::ToHex.name().to_string());
    names.push(to_hex_content::ToHexContent.name().to_string());
    names.push(to_hexdump::ToHexdump.name().to_string());
    names.push(to_html_entity::ToHTMLEntity.name().to_string());
    names.push(to_kebab_case::ToKebabCase.name().to_string());
    names.push(to_lower_case::ToLowerCase.name().to_string());
    names.push(to_message_pack::ToMessagePack.name().to_string());
    names.push(to_modhex::ToModhex.name().to_string());
    names.push(to_morse_code::ToMorseCode.name().to_string());
    names.push(to_octal::ToOctal.name().to_string());
    names.push(to_punycode::ToPunycode.name().to_string());
    names.push(to_quoted_printable::ToQuotedPrintable.name().to_string());
    names.push(to_snake_case::ToSnakeCase.name().to_string());
    names.push(to_table::ToTable.name().to_string());
    names.push(to_unix_timestamp::ToUNIXTimestamp.name().to_string());
    names.push(to_upper_case::ToUpperCase.name().to_string());
    names.push(translate_date_time_format::TranslateDateTimeFormat.name().to_string());
    names.push(triple_des_decrypt::TripleDESDecrypt.name().to_string());
    names.push(triple_des_encrypt::TripleDESEncrypt.name().to_string());
    names.push(typex::Typex.name().to_string());
    names.push(unescape_string::UnescapeString.name().to_string());
    names.push(unescape_unicode_characters::UnescapeUnicodeCharacters.name().to_string());
    names.push(unicode_text_format::UnicodeTextFormat.name().to_string());
    names.push(unique::Unique.name().to_string());
    names.push(unix_timestamp_to_windows_filetime::UNIXTimestampToWindowsFiletime.name().to_string());
    names.push(untar::Untar.name().to_string());
    names.push(unzip::Unzip.name().to_string());
    names.push(url_decode::URLDecode.name().to_string());
    names.push(url_encode::URLEncode.name().to_string());
    names.push(varint_decode::VarIntDecode.name().to_string());
    names.push(varint_encode::VarIntEncode.name().to_string());
    names.push(view_bit_plane::ViewBitPlane.name().to_string());
    names.push(vigenere_decode::VigenereDecodeOp.name().to_string());
    names.push(vigenere_encode::VigenereEncodeOp.name().to_string());
    names.push(whirlpool::WHIRLPOOL.name().to_string());
    names.push(windows_filetime_to_unix_timestamp::WindowsFiletimeToUnixTimestampOp.name().to_string());
    names.push(wrap::WrapOp.name().to_string());
    names.push(x_path_expression::XPathExpression.name().to_string());
    names.push(x_salsa20::XSalsa20Op.name().to_string());
    names.push(xkcd_random_number::XkcdRandomNumberOp.name().to_string());
    names.push(xml_beautify::XMLBeautify.name().to_string());
    names.push(xml_minify::XMLMinify.name().to_string());
    names.push(xor::XorOp.name().to_string());
    names.push(xor_brute_force::XORBruteForce.name().to_string());
    names.push(xor_checksum::XORChecksum.name().to_string());
    names.push(xxtea_decrypt::XxteaDecryptOp.name().to_string());
    names.push(xxtea_encrypt::XxteaEncryptOp.name().to_string());
    names.push(yaml_to_json::YAMLToJSON.name().to_string());
    names.push(yara_rules::YARARules.name().to_string());
    names.push(zip::ZipOp.name().to_string());
    names.push(zlib_deflate::ZlibDeflate.name().to_string());
    names.push(zlib_inflate::ZlibInflate.name().to_string());
    names.sort();
    names
}

/// Returns an operation by its name.
pub fn get_operation(name: &str) -> Option<Box<dyn Operation>> {
    let lowered = name.to_lowercase();
    { let op = a1z26_cipher_decode::A1Z26CipherDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(a1z26_cipher_decode::A1Z26CipherDecode)); } }
    { let op = a1z26_cipher_encode::A1Z26CipherEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(a1z26_cipher_encode::A1Z26CipherEncode)); } }
    { let op = add::AddOp; if op.name().to_lowercase() == lowered { return Some(Box::new(add::AddOp)); } }
    { let op = add_line_numbers::AddLineNumbers; if op.name().to_lowercase() == lowered { return Some(Box::new(add_line_numbers::AddLineNumbers)); } }
    { let op = add_text_to_image::AddTextToImage; if op.name().to_lowercase() == lowered { return Some(Box::new(add_text_to_image::AddTextToImage)); } }
    { let op = adler32_checksum::Adler32Checksum; if op.name().to_lowercase() == lowered { return Some(Box::new(adler32_checksum::Adler32Checksum)); } }
    { let op = aes_decrypt::AesDecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(aes_decrypt::AesDecrypt)); } }
    { let op = aes_encrypt::AesEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(aes_encrypt::AesEncrypt)); } }
    { let op = aes_key_unwrap::AesKeyUnwrap; if op.name().to_lowercase() == lowered { return Some(Box::new(aes_key_unwrap::AesKeyUnwrap)); } }
    { let op = aes_key_wrap::AesKeyWrap; if op.name().to_lowercase() == lowered { return Some(Box::new(aes_key_wrap::AesKeyWrap)); } }
    { let op = affine_cipher_decode::AffineCipherDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(affine_cipher_decode::AffineCipherDecode)); } }
    { let op = affine_cipher_encode::AffineCipherEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(affine_cipher_encode::AffineCipherEncode)); } }
    { let op = alternating_caps::AlternatingCaps; if op.name().to_lowercase() == lowered { return Some(Box::new(alternating_caps::AlternatingCaps)); } }
    { let op = amf_decode::AmfDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(amf_decode::AmfDecode)); } }
    { let op = amf_encode::AmfEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(amf_encode::AmfEncode)); } }
    { let op = analyse_hash::AnalyseHash; if op.name().to_lowercase() == lowered { return Some(Box::new(analyse_hash::AnalyseHash)); } }
    { let op = analyse_uuid::AnalyseUUID; if op.name().to_lowercase() == lowered { return Some(Box::new(analyse_uuid::AnalyseUUID)); } }
    { let op = and::AndOp; if op.name().to_lowercase() == lowered { return Some(Box::new(and::AndOp)); } }
    { let op = argon2::Argon2; if op.name().to_lowercase() == lowered { return Some(Box::new(argon2::Argon2)); } }
    { let op = argon2_compare::Argon2Compare; if op.name().to_lowercase() == lowered { return Some(Box::new(argon2_compare::Argon2Compare)); } }
    { let op = atbash_cipher::AtbashCipher; if op.name().to_lowercase() == lowered { return Some(Box::new(atbash_cipher::AtbashCipher)); } }
    { let op = avro_to_json::AvroToJSON; if op.name().to_lowercase() == lowered { return Some(Box::new(avro_to_json::AvroToJSON)); } }
    { let op = bacon_cipher_decode::BaconCipherDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(bacon_cipher_decode::BaconCipherDecode)); } }
    { let op = bacon_cipher_encode::BaconCipherEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(bacon_cipher_encode::BaconCipherEncode)); } }
    { let op = bcrypt::Bcrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(bcrypt::Bcrypt)); } }
    { let op = bcrypt_compare::BcryptCompare; if op.name().to_lowercase() == lowered { return Some(Box::new(bcrypt_compare::BcryptCompare)); } }
    { let op = bcrypt_parse::BcryptParse; if op.name().to_lowercase() == lowered { return Some(Box::new(bcrypt_parse::BcryptParse)); } }
    { let op = bifid_cipher_decode::BifidCipherDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(bifid_cipher_decode::BifidCipherDecode)); } }
    { let op = bifid_cipher_encode::BifidCipherEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(bifid_cipher_encode::BifidCipherEncode)); } }
    { let op = bit_shift_left::BitShiftLeft; if op.name().to_lowercase() == lowered { return Some(Box::new(bit_shift_left::BitShiftLeft)); } }
    { let op = bit_shift_right::BitShiftRight; if op.name().to_lowercase() == lowered { return Some(Box::new(bit_shift_right::BitShiftRight)); } }
    { let op = blake2b::BLAKE2b; if op.name().to_lowercase() == lowered { return Some(Box::new(blake2b::BLAKE2b)); } }
    { let op = blake2s::BLAKE2s; if op.name().to_lowercase() == lowered { return Some(Box::new(blake2s::BLAKE2s)); } }
    { let op = blake3::BLAKE3; if op.name().to_lowercase() == lowered { return Some(Box::new(blake3::BLAKE3)); } }
    { let op = blowfish_decrypt::BlowfishDecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(blowfish_decrypt::BlowfishDecrypt)); } }
    { let op = blowfish_encrypt::BlowfishEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(blowfish_encrypt::BlowfishEncrypt)); } }
    { let op = blur_image::BlurImage; if op.name().to_lowercase() == lowered { return Some(Box::new(blur_image::BlurImage)); } }
    { let op = bombe::Bombe; if op.name().to_lowercase() == lowered { return Some(Box::new(bombe::Bombe)); } }
    { let op = bson_deserialise::BsonDeserialise; if op.name().to_lowercase() == lowered { return Some(Box::new(bson_deserialise::BsonDeserialise)); } }
    { let op = bson_serialise::BsonSerialise; if op.name().to_lowercase() == lowered { return Some(Box::new(bson_serialise::BsonSerialise)); } }
    { let op = bzip2_compress::Bzip2Compress; if op.name().to_lowercase() == lowered { return Some(Box::new(bzip2_compress::Bzip2Compress)); } }
    { let op = bzip2_decompress::Bzip2Decompress; if op.name().to_lowercase() == lowered { return Some(Box::new(bzip2_decompress::Bzip2Decompress)); } }
    { let op = caesar_box_cipher::CaesarBoxCipher; if op.name().to_lowercase() == lowered { return Some(Box::new(caesar_box_cipher::CaesarBoxCipher)); } }
    { let op = caret_mdecode::CaretMdecode; if op.name().to_lowercase() == lowered { return Some(Box::new(caret_mdecode::CaretMdecode)); } }
    { let op = cartesian_product::CartesianProduct; if op.name().to_lowercase() == lowered { return Some(Box::new(cartesian_product::CartesianProduct)); } }
    { let op = cbor_decode::CBORDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(cbor_decode::CBORDecode)); } }
    { let op = cbor_encode::CBOREncode; if op.name().to_lowercase() == lowered { return Some(Box::new(cbor_encode::CBOREncode)); } }
    { let op = cetacean_cipher_decode::CetaceanCipherDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(cetacean_cipher_decode::CetaceanCipherDecode)); } }
    { let op = cetacean_cipher_encode::CetaceanCipherEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(cetacean_cipher_encode::CetaceanCipherEncode)); } }
    { let op = chacha::ChaCha; if op.name().to_lowercase() == lowered { return Some(Box::new(chacha::ChaCha)); } }
    { let op = change_ip_format::ChangeIPFormat; if op.name().to_lowercase() == lowered { return Some(Box::new(change_ip_format::ChangeIPFormat)); } }
    { let op = chi_square::ChiSquare; if op.name().to_lowercase() == lowered { return Some(Box::new(chi_square::ChiSquare)); } }
    { let op = cipher_saber2_decrypt::CipherSaber2Decrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(cipher_saber2_decrypt::CipherSaber2Decrypt)); } }
    { let op = cipher_saber2_encrypt::CipherSaber2Encrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(cipher_saber2_encrypt::CipherSaber2Encrypt)); } }
    { let op = citrix_ctx1_decode::CitrixCtx1Decode; if op.name().to_lowercase() == lowered { return Some(Box::new(citrix_ctx1_decode::CitrixCtx1Decode)); } }
    { let op = citrix_ctx1_encode::CitrixCtx1Encode; if op.name().to_lowercase() == lowered { return Some(Box::new(citrix_ctx1_encode::CitrixCtx1Encode)); } }
    { let op = cmac::Cmac; if op.name().to_lowercase() == lowered { return Some(Box::new(cmac::Cmac)); } }
    { let op = colossus::Colossus; if op.name().to_lowercase() == lowered { return Some(Box::new(colossus::Colossus)); } }
    { let op = comment::CommentOp; if op.name().to_lowercase() == lowered { return Some(Box::new(comment::CommentOp)); } }
    { let op = compare_ctph_hashes::CompareCTPHHashes; if op.name().to_lowercase() == lowered { return Some(Box::new(compare_ctph_hashes::CompareCTPHHashes)); } }
    { let op = compare_ssdeep_hashes::CompareSSDEEPHashes; if op.name().to_lowercase() == lowered { return Some(Box::new(compare_ssdeep_hashes::CompareSSDEEPHashes)); } }
    { let op = conditional_jump::ConditionalJump; if op.name().to_lowercase() == lowered { return Some(Box::new(conditional_jump::ConditionalJump)); } }
    { let op = contain_image::ContainImage; if op.name().to_lowercase() == lowered { return Some(Box::new(contain_image::ContainImage)); } }
    { let op = convert_area::ConvertArea; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_area::ConvertArea)); } }
    { let op = convert_coordinate_format::ConvertCoordinateFormat; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_coordinate_format::ConvertCoordinateFormat)); } }
    { let op = convert_data_units::ConvertDataUnits; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_data_units::ConvertDataUnits)); } }
    { let op = convert_distance::ConvertDistance; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_distance::ConvertDistance)); } }
    { let op = convert_image_format::ConvertImageFormat; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_image_format::ConvertImageFormat)); } }
    { let op = convert_leet_speak::ConvertLeetSpeak; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_leet_speak::ConvertLeetSpeak)); } }
    { let op = convert_mass::ConvertMass; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_mass::ConvertMass)); } }
    { let op = convert_speed::ConvertSpeed; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_speed::ConvertSpeed)); } }
    { let op = convert_to_nato_alphabet::ConvertToNATOAlphabet; if op.name().to_lowercase() == lowered { return Some(Box::new(convert_to_nato_alphabet::ConvertToNATOAlphabet)); } }
    { let op = count_occurrences::CountOccurrences; if op.name().to_lowercase() == lowered { return Some(Box::new(count_occurrences::CountOccurrences)); } }
    { let op = cover_image::CoverImage; if op.name().to_lowercase() == lowered { return Some(Box::new(cover_image::CoverImage)); } }
    { let op = crc32::CRC32; if op.name().to_lowercase() == lowered { return Some(Box::new(crc32::CRC32)); } }
    { let op = crc_checksum::CrcChecksum; if op.name().to_lowercase() == lowered { return Some(Box::new(crc_checksum::CrcChecksum)); } }
    { let op = crop_image::CropImage; if op.name().to_lowercase() == lowered { return Some(Box::new(crop_image::CropImage)); } }
    { let op = css_beautify::CssBeautify; if op.name().to_lowercase() == lowered { return Some(Box::new(css_beautify::CssBeautify)); } }
    { let op = css_minify::CssMinify; if op.name().to_lowercase() == lowered { return Some(Box::new(css_minify::CssMinify)); } }
    { let op = css_selector::CssSelector; if op.name().to_lowercase() == lowered { return Some(Box::new(css_selector::CssSelector)); } }
    { let op = csv_to_json::CsvToJson; if op.name().to_lowercase() == lowered { return Some(Box::new(csv_to_json::CsvToJson)); } }
    { let op = ctph::CTPH; if op.name().to_lowercase() == lowered { return Some(Box::new(ctph::CTPH)); } }
    { let op = date_time_delta::DateTimeDelta; if op.name().to_lowercase() == lowered { return Some(Box::new(date_time_delta::DateTimeDelta)); } }
    { let op = dechunk_http_response::DechunkHttpResponse; if op.name().to_lowercase() == lowered { return Some(Box::new(dechunk_http_response::DechunkHttpResponse)); } }
    { let op = decode_netbios_name::DecodeNetBIOSName; if op.name().to_lowercase() == lowered { return Some(Box::new(decode_netbios_name::DecodeNetBIOSName)); } }
    { let op = decode_text::DecodeText; if op.name().to_lowercase() == lowered { return Some(Box::new(decode_text::DecodeText)); } }
    { let op = defang_ip_addresses::DefangIPAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(defang_ip_addresses::DefangIPAddresses)); } }
    { let op = defang_url::DefangURL; if op.name().to_lowercase() == lowered { return Some(Box::new(defang_url::DefangURL)); } }
    { let op = derive_evp_key::DeriveEvpKey; if op.name().to_lowercase() == lowered { return Some(Box::new(derive_evp_key::DeriveEvpKey)); } }
    { let op = derive_hkdf_key::DeriveHKDFKey; if op.name().to_lowercase() == lowered { return Some(Box::new(derive_hkdf_key::DeriveHKDFKey)); } }
    { let op = derive_pbkdf2_key::DerivePBKDF2Key; if op.name().to_lowercase() == lowered { return Some(Box::new(derive_pbkdf2_key::DerivePBKDF2Key)); } }
    { let op = des_decrypt::DesDecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(des_decrypt::DesDecrypt)); } }
    { let op = des_encrypt::DesEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(des_encrypt::DesEncrypt)); } }
    { let op = detect_file_type::DetectFileType; if op.name().to_lowercase() == lowered { return Some(Box::new(detect_file_type::DetectFileType)); } }
    { let op = diff::Diff; if op.name().to_lowercase() == lowered { return Some(Box::new(diff::Diff)); } }
    { let op = disassemble_arm::DisassembleArm; if op.name().to_lowercase() == lowered { return Some(Box::new(disassemble_arm::DisassembleArm)); } }
    { let op = disassemble_x86::DisassembleX86; if op.name().to_lowercase() == lowered { return Some(Box::new(disassemble_x86::DisassembleX86)); } }
    { let op = dither_image::DitherImage; if op.name().to_lowercase() == lowered { return Some(Box::new(dither_image::DitherImage)); } }
    { let op = divide::Divide; if op.name().to_lowercase() == lowered { return Some(Box::new(divide::Divide)); } }
    { let op = dns_over_https::DnsOverHttps; if op.name().to_lowercase() == lowered { return Some(Box::new(dns_over_https::DnsOverHttps)); } }
    { let op = drop_bytes::DropBytes; if op.name().to_lowercase() == lowered { return Some(Box::new(drop_bytes::DropBytes)); } }
    { let op = drop_nth_bytes::DropNthBytes; if op.name().to_lowercase() == lowered { return Some(Box::new(drop_nth_bytes::DropNthBytes)); } }
    { let op = ecdsa_sign::ECDSASign; if op.name().to_lowercase() == lowered { return Some(Box::new(ecdsa_sign::ECDSASign)); } }
    { let op = ecdsa_signature_conversion::ECDSASignatureConversion; if op.name().to_lowercase() == lowered { return Some(Box::new(ecdsa_signature_conversion::ECDSASignatureConversion)); } }
    { let op = ecdsa_verify::ECDSAVerify; if op.name().to_lowercase() == lowered { return Some(Box::new(ecdsa_verify::ECDSAVerify)); } }
    { let op = elf_info::ELFInfo; if op.name().to_lowercase() == lowered { return Some(Box::new(elf_info::ELFInfo)); } }
    { let op = encode_netbios_name::EncodeNetBIOSName; if op.name().to_lowercase() == lowered { return Some(Box::new(encode_netbios_name::EncodeNetBIOSName)); } }
    { let op = encode_text::EncodeText; if op.name().to_lowercase() == lowered { return Some(Box::new(encode_text::EncodeText)); } }
    { let op = enigma::Enigma; if op.name().to_lowercase() == lowered { return Some(Box::new(enigma::Enigma)); } }
    { let op = entropy::Entropy; if op.name().to_lowercase() == lowered { return Some(Box::new(entropy::Entropy)); } }
    { let op = escape_string::EscapeString; if op.name().to_lowercase() == lowered { return Some(Box::new(escape_string::EscapeString)); } }
    { let op = escape_unicode_characters::EscapeUnicodeCharacters; if op.name().to_lowercase() == lowered { return Some(Box::new(escape_unicode_characters::EscapeUnicodeCharacters)); } }
    { let op = expand_alphabet_range::ExpandAlphabetRange; if op.name().to_lowercase() == lowered { return Some(Box::new(expand_alphabet_range::ExpandAlphabetRange)); } }
    { let op = extract_audio_metadata::ExtractAudioMetadata; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_audio_metadata::ExtractAudioMetadata)); } }
    { let op = extract_dates::ExtractDates; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_dates::ExtractDates)); } }
    { let op = extract_domains::ExtractDomains; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_domains::ExtractDomains)); } }
    { let op = extract_email_addresses::ExtractEmailAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_email_addresses::ExtractEmailAddresses)); } }
    { let op = extract_exif::ExtractEXIF; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_exif::ExtractEXIF)); } }
    { let op = extract_file_paths::ExtractFilePaths; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_file_paths::ExtractFilePaths)); } }
    { let op = extract_files::ExtractFiles; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_files::ExtractFiles)); } }
    { let op = extract_hashes::ExtractHashes; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_hashes::ExtractHashes)); } }
    { let op = extract_id3::ExtractID3; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_id3::ExtractID3)); } }
    { let op = extract_ip_addresses::ExtractIPAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_ip_addresses::ExtractIPAddresses)); } }
    { let op = extract_lsb::ExtractLSB; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_lsb::ExtractLSB)); } }
    { let op = extract_mac_addresses::ExtractMACAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_mac_addresses::ExtractMACAddresses)); } }
    { let op = extract_rgba::ExtractRGBA; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_rgba::ExtractRGBA)); } }
    { let op = extract_urls::ExtractURLs; if op.name().to_lowercase() == lowered { return Some(Box::new(extract_urls::ExtractURLs)); } }
    { let op = fang_url::FangURL; if op.name().to_lowercase() == lowered { return Some(Box::new(fang_url::FangURL)); } }
    { let op = fernet_decrypt::FernetDecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(fernet_decrypt::FernetDecrypt)); } }
    { let op = fernet_encrypt::FernetEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(fernet_encrypt::FernetEncrypt)); } }
    { let op = file_tree::FileTree; if op.name().to_lowercase() == lowered { return Some(Box::new(file_tree::FileTree)); } }
    { let op = filter::Filter; if op.name().to_lowercase() == lowered { return Some(Box::new(filter::Filter)); } }
    { let op = find_replace::FindReplace; if op.name().to_lowercase() == lowered { return Some(Box::new(find_replace::FindReplace)); } }
    { let op = flask_session_decode::FlaskSessionDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(flask_session_decode::FlaskSessionDecode)); } }
    { let op = flask_session_sign::FlaskSessionSign; if op.name().to_lowercase() == lowered { return Some(Box::new(flask_session_sign::FlaskSessionSign)); } }
    { let op = flask_session_verify::FlaskSessionVerify; if op.name().to_lowercase() == lowered { return Some(Box::new(flask_session_verify::FlaskSessionVerify)); } }
    { let op = fletcher16_checksum::Fletcher16Checksum; if op.name().to_lowercase() == lowered { return Some(Box::new(fletcher16_checksum::Fletcher16Checksum)); } }
    { let op = fletcher32_checksum::Fletcher32Checksum; if op.name().to_lowercase() == lowered { return Some(Box::new(fletcher32_checksum::Fletcher32Checksum)); } }
    { let op = fletcher64_checksum::Fletcher64Checksum; if op.name().to_lowercase() == lowered { return Some(Box::new(fletcher64_checksum::Fletcher64Checksum)); } }
    { let op = fletcher8_checksum::Fletcher8Checksum; if op.name().to_lowercase() == lowered { return Some(Box::new(fletcher8_checksum::Fletcher8Checksum)); } }
    { let op = flip_image::FlipImage; if op.name().to_lowercase() == lowered { return Some(Box::new(flip_image::FlipImage)); } }
    { let op = fork::Fork; if op.name().to_lowercase() == lowered { return Some(Box::new(fork::Fork)); } }
    { let op = format_mac_addresses::FormatMACAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(format_mac_addresses::FormatMACAddresses)); } }
    { let op = frequency_distribution::FrequencyDistribution; if op.name().to_lowercase() == lowered { return Some(Box::new(frequency_distribution::FrequencyDistribution)); } }
    { let op = from_base::FromBase; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base::FromBase)); } }
    { let op = from_base32::FromBase32; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base32::FromBase32)); } }
    { let op = from_base45::FromBase45; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base45::FromBase45)); } }
    { let op = from_base58::FromBase58; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base58::FromBase58)); } }
    { let op = from_base62::FromBase62; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base62::FromBase62)); } }
    { let op = from_base64::FromBase64; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base64::FromBase64)); } }
    { let op = from_base85::FromBase85; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base85::FromBase85)); } }
    { let op = from_base92::FromBase92; if op.name().to_lowercase() == lowered { return Some(Box::new(from_base92::FromBase92)); } }
    { let op = from_bcd::FromBCD; if op.name().to_lowercase() == lowered { return Some(Box::new(from_bcd::FromBCD)); } }
    { let op = from_bech32::FromBech32; if op.name().to_lowercase() == lowered { return Some(Box::new(from_bech32::FromBech32)); } }
    { let op = from_binary::FromBinary; if op.name().to_lowercase() == lowered { return Some(Box::new(from_binary::FromBinary)); } }
    { let op = from_braille::FromBraille; if op.name().to_lowercase() == lowered { return Some(Box::new(from_braille::FromBraille)); } }
    { let op = from_case_insensitive_regex::FromCaseInsensitiveRegex; if op.name().to_lowercase() == lowered { return Some(Box::new(from_case_insensitive_regex::FromCaseInsensitiveRegex)); } }
    { let op = from_charcode::FromCharcode; if op.name().to_lowercase() == lowered { return Some(Box::new(from_charcode::FromCharcode)); } }
    { let op = from_decimal::FromDecimal; if op.name().to_lowercase() == lowered { return Some(Box::new(from_decimal::FromDecimal)); } }
    { let op = from_float::FromFloat; if op.name().to_lowercase() == lowered { return Some(Box::new(from_float::FromFloat)); } }
    { let op = from_hex::FromHex; if op.name().to_lowercase() == lowered { return Some(Box::new(from_hex::FromHex)); } }
    { let op = from_hex_content::FromHexContent; if op.name().to_lowercase() == lowered { return Some(Box::new(from_hex_content::FromHexContent)); } }
    { let op = from_hexdump::FromHexdump; if op.name().to_lowercase() == lowered { return Some(Box::new(from_hexdump::FromHexdump)); } }
    { let op = from_html_entity::FromHTMLEntity; if op.name().to_lowercase() == lowered { return Some(Box::new(from_html_entity::FromHTMLEntity)); } }
    { let op = from_message_pack::FromMessagePack; if op.name().to_lowercase() == lowered { return Some(Box::new(from_message_pack::FromMessagePack)); } }
    { let op = from_modhex::FromModhex; if op.name().to_lowercase() == lowered { return Some(Box::new(from_modhex::FromModhex)); } }
    { let op = from_morse_code::FromMorseCode; if op.name().to_lowercase() == lowered { return Some(Box::new(from_morse_code::FromMorseCode)); } }
    { let op = from_octal::FromOctal; if op.name().to_lowercase() == lowered { return Some(Box::new(from_octal::FromOctal)); } }
    { let op = from_punycode::FromPunycode; if op.name().to_lowercase() == lowered { return Some(Box::new(from_punycode::FromPunycode)); } }
    { let op = from_quoted_printable::FromQuotedPrintable; if op.name().to_lowercase() == lowered { return Some(Box::new(from_quoted_printable::FromQuotedPrintable)); } }
    { let op = from_unix_timestamp::FromUNIXTimestamp; if op.name().to_lowercase() == lowered { return Some(Box::new(from_unix_timestamp::FromUNIXTimestamp)); } }
    { let op = fuzzy_match::FuzzyMatch; if op.name().to_lowercase() == lowered { return Some(Box::new(fuzzy_match::FuzzyMatch)); } }
    { let op = generate_all_checksums::GenerateAllChecksums; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_all_checksums::GenerateAllChecksums)); } }
    { let op = generate_all_hashes::GenerateAllHashes; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_all_hashes::GenerateAllHashes)); } }
    { let op = generate_de_bruijn_sequence::GenerateDeBruijnSequence; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_de_bruijn_sequence::GenerateDeBruijnSequence)); } }
    { let op = generate_ecdsa_key_pair::GenerateECDSAKeyPairOp; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_ecdsa_key_pair::GenerateECDSAKeyPairOp)); } }
    { let op = generate_hotp::GenerateHOTPOp; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_hotp::GenerateHOTPOp)); } }
    { let op = generate_image::GenerateImageOp; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_image::GenerateImageOp)); } }
    { let op = generate_lorem_ipsum::GenerateLoremIpsum; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_lorem_ipsum::GenerateLoremIpsum)); } }
    { let op = generate_pgp_key_pair::GeneratePGPKeyPair; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_pgp_key_pair::GeneratePGPKeyPair)); } }
    { let op = generate_qr_code::GenerateQRCodeOp; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_qr_code::GenerateQRCodeOp)); } }
    { let op = generate_rsa_key_pair::GenerateRSAKeyPair; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_rsa_key_pair::GenerateRSAKeyPair)); } }
    { let op = generate_totp::GenerateTOTP; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_totp::GenerateTOTP)); } }
    { let op = generate_uuid::GenerateUUID; if op.name().to_lowercase() == lowered { return Some(Box::new(generate_uuid::GenerateUUID)); } }
    { let op = generic_code_beautify::GenericCodeBeautify; if op.name().to_lowercase() == lowered { return Some(Box::new(generic_code_beautify::GenericCodeBeautify)); } }
    { let op = get_all_casings::GetAllCasings; if op.name().to_lowercase() == lowered { return Some(Box::new(get_all_casings::GetAllCasings)); } }
    { let op = get_time::GetTime; if op.name().to_lowercase() == lowered { return Some(Box::new(get_time::GetTime)); } }
    { let op = gost_decrypt::GOSTDecryptOp; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_decrypt::GOSTDecryptOp)); } }
    { let op = gost_encrypt::GostEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_encrypt::GostEncrypt)); } }
    { let op = gost_hash::GostHash; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_hash::GostHash)); } }
    { let op = gost_key_unwrap::GOSTKeyUnwrapOp; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_key_unwrap::GOSTKeyUnwrapOp)); } }
    { let op = gost_key_wrap::GostKeyWrap; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_key_wrap::GostKeyWrap)); } }
    { let op = gost_sign::GostSign; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_sign::GostSign)); } }
    { let op = gost_verify::GOSTVerifyOp; if op.name().to_lowercase() == lowered { return Some(Box::new(gost_verify::GOSTVerifyOp)); } }
    { let op = group_ip_addresses::GroupIPAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(group_ip_addresses::GroupIPAddresses)); } }
    { let op = gunzip::Gunzip; if op.name().to_lowercase() == lowered { return Some(Box::new(gunzip::Gunzip)); } }
    { let op = gzip::Gzip; if op.name().to_lowercase() == lowered { return Some(Box::new(gzip::Gzip)); } }
    { let op = hamming_distance::HammingDistance; if op.name().to_lowercase() == lowered { return Some(Box::new(hamming_distance::HammingDistance)); } }
    { let op = has160::HAS160Op; if op.name().to_lowercase() == lowered { return Some(Box::new(has160::HAS160Op)); } }
    { let op = hassh_client_fingerprint::HASSHClientFingerprint; if op.name().to_lowercase() == lowered { return Some(Box::new(hassh_client_fingerprint::HASSHClientFingerprint)); } }
    { let op = hassh_server_fingerprint::HASSHServerFingerprint; if op.name().to_lowercase() == lowered { return Some(Box::new(hassh_server_fingerprint::HASSHServerFingerprint)); } }
    { let op = haversine_distance::HaversineDistance; if op.name().to_lowercase() == lowered { return Some(Box::new(haversine_distance::HaversineDistance)); } }
    { let op = head::Head; if op.name().to_lowercase() == lowered { return Some(Box::new(head::Head)); } }
    { let op = heatmap_chart::HeatmapChart; if op.name().to_lowercase() == lowered { return Some(Box::new(heatmap_chart::HeatmapChart)); } }
    { let op = hex_density_chart::HexDensityChartOp; if op.name().to_lowercase() == lowered { return Some(Box::new(hex_density_chart::HexDensityChartOp)); } }
    { let op = hex_to_object_identifier::HexToObjectIdentifier; if op.name().to_lowercase() == lowered { return Some(Box::new(hex_to_object_identifier::HexToObjectIdentifier)); } }
    { let op = hex_to_pem::HexToPEM; if op.name().to_lowercase() == lowered { return Some(Box::new(hex_to_pem::HexToPEM)); } }
    { let op = hmac::HMAC; if op.name().to_lowercase() == lowered { return Some(Box::new(hmac::HMAC)); } }
    { let op = html_to_text::HTMLToText; if op.name().to_lowercase() == lowered { return Some(Box::new(html_to_text::HTMLToText)); } }
    { let op = http_request::HTTPRequest; if op.name().to_lowercase() == lowered { return Some(Box::new(http_request::HTTPRequest)); } }
    { let op = image_brightness_contrast::ImageBrightnessContrast; if op.name().to_lowercase() == lowered { return Some(Box::new(image_brightness_contrast::ImageBrightnessContrast)); } }
    { let op = image_filter::ImageFilter; if op.name().to_lowercase() == lowered { return Some(Box::new(image_filter::ImageFilter)); } }
    { let op = image_hue_saturation_lightness::ImageHueSaturationLightness; if op.name().to_lowercase() == lowered { return Some(Box::new(image_hue_saturation_lightness::ImageHueSaturationLightness)); } }
    { let op = image_opacity::ImageOpacity; if op.name().to_lowercase() == lowered { return Some(Box::new(image_opacity::ImageOpacity)); } }
    { let op = index_of_coincidence::IndexOfCoincidence; if op.name().to_lowercase() == lowered { return Some(Box::new(index_of_coincidence::IndexOfCoincidence)); } }
    { let op = invert_image::InvertImage; if op.name().to_lowercase() == lowered { return Some(Box::new(invert_image::InvertImage)); } }
    { let op = ipv6_transition_addresses::IPv6TransitionAddresses; if op.name().to_lowercase() == lowered { return Some(Box::new(ipv6_transition_addresses::IPv6TransitionAddresses)); } }
    { let op = j_path_expression::JPathExpression; if op.name().to_lowercase() == lowered { return Some(Box::new(j_path_expression::JPathExpression)); } }
    { let op = ja3_fingerprint::JA3Fingerprint; if op.name().to_lowercase() == lowered { return Some(Box::new(ja3_fingerprint::JA3Fingerprint)); } }
    { let op = ja3s_fingerprint::JA3SFingerprint; if op.name().to_lowercase() == lowered { return Some(Box::new(ja3s_fingerprint::JA3SFingerprint)); } }
    { let op = ja4_fingerprint::JA4Fingerprint; if op.name().to_lowercase() == lowered { return Some(Box::new(ja4_fingerprint::JA4Fingerprint)); } }
    { let op = ja4_server_fingerprint::JA4ServerFingerprint; if op.name().to_lowercase() == lowered { return Some(Box::new(ja4_server_fingerprint::JA4ServerFingerprint)); } }
    { let op = java_script_beautify::JavaScriptBeautify; if op.name().to_lowercase() == lowered { return Some(Box::new(java_script_beautify::JavaScriptBeautify)); } }
    { let op = java_script_minify::JavaScriptMinify; if op.name().to_lowercase() == lowered { return Some(Box::new(java_script_minify::JavaScriptMinify)); } }
    { let op = java_script_parser::JavaScriptParser; if op.name().to_lowercase() == lowered { return Some(Box::new(java_script_parser::JavaScriptParser)); } }
    { let op = jq::Jq; if op.name().to_lowercase() == lowered { return Some(Box::new(jq::Jq)); } }
    { let op = json_beautify::JSONBeautify; if op.name().to_lowercase() == lowered { return Some(Box::new(json_beautify::JSONBeautify)); } }
    { let op = json_minify::JSONMinify; if op.name().to_lowercase() == lowered { return Some(Box::new(json_minify::JSONMinify)); } }
    { let op = json_to_csv::JSONToCSV; if op.name().to_lowercase() == lowered { return Some(Box::new(json_to_csv::JSONToCSV)); } }
    { let op = json_to_yaml::JSONToYAML; if op.name().to_lowercase() == lowered { return Some(Box::new(json_to_yaml::JSONToYAML)); } }
    { let op = jsonata::Jsonata; if op.name().to_lowercase() == lowered { return Some(Box::new(jsonata::Jsonata)); } }
    { let op = jump::Jump; if op.name().to_lowercase() == lowered { return Some(Box::new(jump::Jump)); } }
    { let op = jwk_to_pem::JWKToPem; if op.name().to_lowercase() == lowered { return Some(Box::new(jwk_to_pem::JWKToPem)); } }
    { let op = jwt_decode::JWTDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(jwt_decode::JWTDecode)); } }
    { let op = jwt_sign::JWTSign; if op.name().to_lowercase() == lowered { return Some(Box::new(jwt_sign::JWTSign)); } }
    { let op = jwt_verify::JWTVerify; if op.name().to_lowercase() == lowered { return Some(Box::new(jwt_verify::JWTVerify)); } }
    { let op = keccak::Keccak; if op.name().to_lowercase() == lowered { return Some(Box::new(keccak::Keccak)); } }
    { let op = label::Label; if op.name().to_lowercase() == lowered { return Some(Box::new(label::Label)); } }
    { let op = levenshtein_distance::LevenshteinDistance; if op.name().to_lowercase() == lowered { return Some(Box::new(levenshtein_distance::LevenshteinDistance)); } }
    { let op = lm_hash::LMHash; if op.name().to_lowercase() == lowered { return Some(Box::new(lm_hash::LMHash)); } }
    { let op = lorenz::Lorenz; if op.name().to_lowercase() == lowered { return Some(Box::new(lorenz::Lorenz)); } }
    { let op = ls47_decrypt::LS47Decrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(ls47_decrypt::LS47Decrypt)); } }
    { let op = ls47_encrypt::LS47Encrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(ls47_encrypt::LS47Encrypt)); } }
    { let op = luhn_checksum::LuhnChecksum; if op.name().to_lowercase() == lowered { return Some(Box::new(luhn_checksum::LuhnChecksum)); } }
    { let op = lz4_compress::LZ4Compress; if op.name().to_lowercase() == lowered { return Some(Box::new(lz4_compress::LZ4Compress)); } }
    { let op = lz4_decompress::LZ4Decompress; if op.name().to_lowercase() == lowered { return Some(Box::new(lz4_decompress::LZ4Decompress)); } }
    { let op = lz_string_compress::LZStringCompress; if op.name().to_lowercase() == lowered { return Some(Box::new(lz_string_compress::LZStringCompress)); } }
    { let op = lz_string_decompress::LZStringDecompress; if op.name().to_lowercase() == lowered { return Some(Box::new(lz_string_decompress::LZStringDecompress)); } }
    { let op = lzma_compress::LZMACompress; if op.name().to_lowercase() == lowered { return Some(Box::new(lzma_compress::LZMACompress)); } }
    { let op = lzma_decompress::LZMADecompress; if op.name().to_lowercase() == lowered { return Some(Box::new(lzma_decompress::LZMADecompress)); } }
    { let op = lznt1_decompress::LZNT1Decompress; if op.name().to_lowercase() == lowered { return Some(Box::new(lznt1_decompress::LZNT1Decompress)); } }
    { let op = magic::Magic; if op.name().to_lowercase() == lowered { return Some(Box::new(magic::Magic)); } }
    { let op = md2::MD2; if op.name().to_lowercase() == lowered { return Some(Box::new(md2::MD2)); } }
    { let op = md4::MD4; if op.name().to_lowercase() == lowered { return Some(Box::new(md4::MD4)); } }
    { let op = md5::MD5; if op.name().to_lowercase() == lowered { return Some(Box::new(md5::MD5)); } }
    { let op = md6::MD6; if op.name().to_lowercase() == lowered { return Some(Box::new(md6::MD6)); } }
    { let op = mean::Mean; if op.name().to_lowercase() == lowered { return Some(Box::new(mean::Mean)); } }
    { let op = median::Median; if op.name().to_lowercase() == lowered { return Some(Box::new(median::Median)); } }
    { let op = merge::Merge; if op.name().to_lowercase() == lowered { return Some(Box::new(merge::Merge)); } }
    { let op = microsoft_script_decoder::MicrosoftScriptDecoder; if op.name().to_lowercase() == lowered { return Some(Box::new(microsoft_script_decoder::MicrosoftScriptDecoder)); } }
    { let op = mime_decoding::MIMEDecoding; if op.name().to_lowercase() == lowered { return Some(Box::new(mime_decoding::MIMEDecoding)); } }
    { let op = multiple_bombe::MultipleBombe; if op.name().to_lowercase() == lowered { return Some(Box::new(multiple_bombe::MultipleBombe)); } }
    { let op = multiply::Multiply; if op.name().to_lowercase() == lowered { return Some(Box::new(multiply::Multiply)); } }
    { let op = murmur_hash3::MurmurHash3; if op.name().to_lowercase() == lowered { return Some(Box::new(murmur_hash3::MurmurHash3)); } }
    { let op = normalise_image::NormaliseImage; if op.name().to_lowercase() == lowered { return Some(Box::new(normalise_image::NormaliseImage)); } }
    { let op = normalise_unicode::NormaliseUnicode; if op.name().to_lowercase() == lowered { return Some(Box::new(normalise_unicode::NormaliseUnicode)); } }
    { let op = not::NOT; if op.name().to_lowercase() == lowered { return Some(Box::new(not::NOT)); } }
    { let op = nt_hash::NTHash; if op.name().to_lowercase() == lowered { return Some(Box::new(nt_hash::NTHash)); } }
    { let op = numberwang::Numberwang; if op.name().to_lowercase() == lowered { return Some(Box::new(numberwang::Numberwang)); } }
    { let op = object_identifier_to_hex::ObjectIdentifierToHex; if op.name().to_lowercase() == lowered { return Some(Box::new(object_identifier_to_hex::ObjectIdentifierToHex)); } }
    { let op = offset_checker::OffsetChecker; if op.name().to_lowercase() == lowered { return Some(Box::new(offset_checker::OffsetChecker)); } }
    { let op = optical_character_recognition::OpticalCharacterRecognition; if op.name().to_lowercase() == lowered { return Some(Box::new(optical_character_recognition::OpticalCharacterRecognition)); } }
    { let op = or::OrOp; if op.name().to_lowercase() == lowered { return Some(Box::new(or::OrOp)); } }
    { let op = pad_lines::PadLines; if op.name().to_lowercase() == lowered { return Some(Box::new(pad_lines::PadLines)); } }
    { let op = parity_bit::ParityBit; if op.name().to_lowercase() == lowered { return Some(Box::new(parity_bit::ParityBit)); } }
    { let op = parse_asn1_hex_string::ParseASN1HexString; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_asn1_hex_string::ParseASN1HexString)); } }
    { let op = parse_colour_code::ParseColourCode; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_colour_code::ParseColourCode)); } }
    { let op = parse_csr::ParseCSR; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_csr::ParseCSR)); } }
    { let op = parse_date_time::ParseDateTime; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_date_time::ParseDateTime)); } }
    { let op = parse_ethernet_frame::ParseEthernetFrame; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_ethernet_frame::ParseEthernetFrame)); } }
    { let op = parse_ip_range::ParseIPRange; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_ip_range::ParseIPRange)); } }
    { let op = parse_ipv4_header::ParseIPv4Header; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_ipv4_header::ParseIPv4Header)); } }
    { let op = parse_ipv6_address::ParseIPv6Address; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_ipv6_address::ParseIPv6Address)); } }
    { let op = parse_object_id_timestamp::ParseObjectIDTimestamp; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_object_id_timestamp::ParseObjectIDTimestamp)); } }
    { let op = parse_qr_code::ParseQRCode; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_qr_code::ParseQRCode)); } }
    { let op = parse_ssh_host_key::ParseSshHostKey; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_ssh_host_key::ParseSshHostKey)); } }
    { let op = parse_tcp::ParseTcp; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_tcp::ParseTcp)); } }
    { let op = parse_tls_record::ParseTLSRecord; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_tls_record::ParseTLSRecord)); } }
    { let op = parse_tlv::ParseTLV; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_tlv::ParseTLV)); } }
    { let op = parse_udp::ParseUDP; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_udp::ParseUDP)); } }
    { let op = parse_unix_file_permissions::ParseUNIXFilePermissions; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_unix_file_permissions::ParseUNIXFilePermissions)); } }
    { let op = parse_uri::ParseURI; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_uri::ParseURI)); } }
    { let op = parse_user_agent::ParseUserAgent; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_user_agent::ParseUserAgent)); } }
    { let op = parse_x509_certificate::ParseX509Certificate; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_x509_certificate::ParseX509Certificate)); } }
    { let op = parse_x509_crl::ParseX509CRL; if op.name().to_lowercase() == lowered { return Some(Box::new(parse_x509_crl::ParseX509CRL)); } }
    { let op = pem_to_hex::PEMToHex; if op.name().to_lowercase() == lowered { return Some(Box::new(pem_to_hex::PEMToHex)); } }
    { let op = pem_to_jwk::PEMToJWK; if op.name().to_lowercase() == lowered { return Some(Box::new(pem_to_jwk::PEMToJWK)); } }
    { let op = pgp_decrypt::PGPDecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(pgp_decrypt::PGPDecrypt)); } }
    { let op = pgp_decrypt_and_verify::PGPDecryptAndVerify; if op.name().to_lowercase() == lowered { return Some(Box::new(pgp_decrypt_and_verify::PGPDecryptAndVerify)); } }
    { let op = pgp_encrypt::PGPEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(pgp_encrypt::PGPEncrypt)); } }
    { let op = pgp_encrypt_and_sign::PGPEncryptAndSign; if op.name().to_lowercase() == lowered { return Some(Box::new(pgp_encrypt_and_sign::PGPEncryptAndSign)); } }
    { let op = pgp_verify::PGPVerify; if op.name().to_lowercase() == lowered { return Some(Box::new(pgp_verify::PGPVerify)); } }
    { let op = php_deserialize::PHPDeserialize; if op.name().to_lowercase() == lowered { return Some(Box::new(php_deserialize::PHPDeserialize)); } }
    { let op = php_serialize::PHPSerialize; if op.name().to_lowercase() == lowered { return Some(Box::new(php_serialize::PHPSerialize)); } }
    { let op = play_media::PlayMedia; if op.name().to_lowercase() == lowered { return Some(Box::new(play_media::PlayMedia)); } }
    { let op = plist_viewer::PLISTViewer; if op.name().to_lowercase() == lowered { return Some(Box::new(plist_viewer::PLISTViewer)); } }
    { let op = power_set::PowerSet; if op.name().to_lowercase() == lowered { return Some(Box::new(power_set::PowerSet)); } }
    { let op = protobuf_decode::ProtobufDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(protobuf_decode::ProtobufDecode)); } }
    { let op = protobuf_encode::ProtobufEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(protobuf_encode::ProtobufEncode)); } }
    { let op = pseudo_random_integer_generator::PseudoRandomIntegerGenerator; if op.name().to_lowercase() == lowered { return Some(Box::new(pseudo_random_integer_generator::PseudoRandomIntegerGenerator)); } }
    { let op = pseudo_random_number_generator::PseudoRandomNumberGenerator; if op.name().to_lowercase() == lowered { return Some(Box::new(pseudo_random_number_generator::PseudoRandomNumberGenerator)); } }
    { let op = pub_key_from_cert::PubKeyFromCert; if op.name().to_lowercase() == lowered { return Some(Box::new(pub_key_from_cert::PubKeyFromCert)); } }
    { let op = pub_key_from_priv_key::PubKeyFromPrivKeyOp; if op.name().to_lowercase() == lowered { return Some(Box::new(pub_key_from_priv_key::PubKeyFromPrivKeyOp)); } }
    { let op = rabbit::RabbitOp; if op.name().to_lowercase() == lowered { return Some(Box::new(rabbit::RabbitOp)); } }
    { let op = rail_fence_cipher_decode::RailFenceCipherDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(rail_fence_cipher_decode::RailFenceCipherDecode)); } }
    { let op = rail_fence_cipher_encode::RailFenceCipherEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(rail_fence_cipher_encode::RailFenceCipherEncode)); } }
    { let op = rake::RAKE; if op.name().to_lowercase() == lowered { return Some(Box::new(rake::RAKE)); } }
    { let op = randomize_colour_palette::RandomizeColourPalette; if op.name().to_lowercase() == lowered { return Some(Box::new(randomize_colour_palette::RandomizeColourPalette)); } }
    { let op = raw_deflate::RawDeflate; if op.name().to_lowercase() == lowered { return Some(Box::new(raw_deflate::RawDeflate)); } }
    { let op = raw_inflate::RawInflate; if op.name().to_lowercase() == lowered { return Some(Box::new(raw_inflate::RawInflate)); } }
    { let op = rc2_decrypt::RC2Decrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(rc2_decrypt::RC2Decrypt)); } }
    { let op = rc2_encrypt::RC2Encrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(rc2_encrypt::RC2Encrypt)); } }
    { let op = rc4::RC4; if op.name().to_lowercase() == lowered { return Some(Box::new(rc4::RC4)); } }
    { let op = rc4_drop::RC4Drop; if op.name().to_lowercase() == lowered { return Some(Box::new(rc4_drop::RC4Drop)); } }
    { let op = rc6_decrypt::RC6Decrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(rc6_decrypt::RC6Decrypt)); } }
    { let op = rc6_encrypt::RC6Encrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(rc6_encrypt::RC6Encrypt)); } }
    { let op = register::Register; if op.name().to_lowercase() == lowered { return Some(Box::new(register::Register)); } }
    { let op = regular_expression::RegularExpressionOp; if op.name().to_lowercase() == lowered { return Some(Box::new(regular_expression::RegularExpressionOp)); } }
    { let op = remove_diacritics::RemoveDiacritics; if op.name().to_lowercase() == lowered { return Some(Box::new(remove_diacritics::RemoveDiacritics)); } }
    { let op = remove_exif::RemoveEXIF; if op.name().to_lowercase() == lowered { return Some(Box::new(remove_exif::RemoveEXIF)); } }
    { let op = remove_line_numbers::RemoveLineNumbers; if op.name().to_lowercase() == lowered { return Some(Box::new(remove_line_numbers::RemoveLineNumbers)); } }
    { let op = remove_null_bytes::RemoveNullBytes; if op.name().to_lowercase() == lowered { return Some(Box::new(remove_null_bytes::RemoveNullBytes)); } }
    { let op = remove_whitespace::RemoveWhitespace; if op.name().to_lowercase() == lowered { return Some(Box::new(remove_whitespace::RemoveWhitespace)); } }
    { let op = render_image::RenderImageOp; if op.name().to_lowercase() == lowered { return Some(Box::new(render_image::RenderImageOp)); } }
    { let op = render_markdown::RenderMarkdown; if op.name().to_lowercase() == lowered { return Some(Box::new(render_markdown::RenderMarkdown)); } }
    { let op = resize_image::ResizeImage; if op.name().to_lowercase() == lowered { return Some(Box::new(resize_image::ResizeImage)); } }
    { let op = return_op::ReturnOp; if op.name().to_lowercase() == lowered { return Some(Box::new(return_op::ReturnOp)); } }
    { let op = reverse::Reverse; if op.name().to_lowercase() == lowered { return Some(Box::new(reverse::Reverse)); } }
    { let op = ripemd::RIPEMD; if op.name().to_lowercase() == lowered { return Some(Box::new(ripemd::RIPEMD)); } }
    { let op = rison_decode::RisonDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(rison_decode::RisonDecode)); } }
    { let op = rison_encode::RisonEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(rison_encode::RisonEncode)); } }
    { let op = rot13::ROT13; if op.name().to_lowercase() == lowered { return Some(Box::new(rot13::ROT13)); } }
    { let op = rot13_brute_force::ROT13BruteForce; if op.name().to_lowercase() == lowered { return Some(Box::new(rot13_brute_force::ROT13BruteForce)); } }
    { let op = rot47::ROT47; if op.name().to_lowercase() == lowered { return Some(Box::new(rot47::ROT47)); } }
    { let op = rot47_brute_force::ROT47BruteForce; if op.name().to_lowercase() == lowered { return Some(Box::new(rot47_brute_force::ROT47BruteForce)); } }
    { let op = rot8000::ROT8000; if op.name().to_lowercase() == lowered { return Some(Box::new(rot8000::ROT8000)); } }
    { let op = rotate_image::RotateImage; if op.name().to_lowercase() == lowered { return Some(Box::new(rotate_image::RotateImage)); } }
    { let op = rotate_left::RotateLeft; if op.name().to_lowercase() == lowered { return Some(Box::new(rotate_left::RotateLeft)); } }
    { let op = rotate_right::RotateRight; if op.name().to_lowercase() == lowered { return Some(Box::new(rotate_right::RotateRight)); } }
    { let op = rsa_decrypt::RSADecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(rsa_decrypt::RSADecrypt)); } }
    { let op = rsa_encrypt::RSAEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(rsa_encrypt::RSAEncrypt)); } }
    { let op = rsa_sign::RSASign; if op.name().to_lowercase() == lowered { return Some(Box::new(rsa_sign::RSASign)); } }
    { let op = rsa_verify::RSAVerify; if op.name().to_lowercase() == lowered { return Some(Box::new(rsa_verify::RSAVerify)); } }
    { let op = salsa20::Salsa20Op; if op.name().to_lowercase() == lowered { return Some(Box::new(salsa20::Salsa20Op)); } }
    { let op = scan_for_embedded_files::ScanForEmbeddedFiles; if op.name().to_lowercase() == lowered { return Some(Box::new(scan_for_embedded_files::ScanForEmbeddedFiles)); } }
    { let op = scatter_chart::ScatterChart; if op.name().to_lowercase() == lowered { return Some(Box::new(scatter_chart::ScatterChart)); } }
    { let op = scrypt::ScryptOp; if op.name().to_lowercase() == lowered { return Some(Box::new(scrypt::ScryptOp)); } }
    { let op = series_chart::SeriesChart; if op.name().to_lowercase() == lowered { return Some(Box::new(series_chart::SeriesChart)); } }
    { let op = set_difference::SetDifference; if op.name().to_lowercase() == lowered { return Some(Box::new(set_difference::SetDifference)); } }
    { let op = set_intersection::SetIntersection; if op.name().to_lowercase() == lowered { return Some(Box::new(set_intersection::SetIntersection)); } }
    { let op = set_union::SetUnion; if op.name().to_lowercase() == lowered { return Some(Box::new(set_union::SetUnion)); } }
    { let op = sha0::SHA0; if op.name().to_lowercase() == lowered { return Some(Box::new(sha0::SHA0)); } }
    { let op = sha1::SHA1; if op.name().to_lowercase() == lowered { return Some(Box::new(sha1::SHA1)); } }
    { let op = sha2::SHA2; if op.name().to_lowercase() == lowered { return Some(Box::new(sha2::SHA2)); } }
    { let op = sha3::SHA3; if op.name().to_lowercase() == lowered { return Some(Box::new(sha3::SHA3)); } }
    { let op = shake::SHAKE; if op.name().to_lowercase() == lowered { return Some(Box::new(shake::SHAKE)); } }
    { let op = sharpen_image::SharpenImage; if op.name().to_lowercase() == lowered { return Some(Box::new(sharpen_image::SharpenImage)); } }
    { let op = show_base64_offsets::ShowBase64Offsets; if op.name().to_lowercase() == lowered { return Some(Box::new(show_base64_offsets::ShowBase64Offsets)); } }
    { let op = show_on_map::ShowOnMap; if op.name().to_lowercase() == lowered { return Some(Box::new(show_on_map::ShowOnMap)); } }
    { let op = shuffle::Shuffle; if op.name().to_lowercase() == lowered { return Some(Box::new(shuffle::Shuffle)); } }
    { let op = sigaba::SigabaOp; if op.name().to_lowercase() == lowered { return Some(Box::new(sigaba::SigabaOp)); } }
    { let op = sleep::Sleep; if op.name().to_lowercase() == lowered { return Some(Box::new(sleep::Sleep)); } }
    { let op = sm2_decrypt::Sm2Decrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(sm2_decrypt::Sm2Decrypt)); } }
    { let op = sm2_encrypt::Sm2Encrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(sm2_encrypt::Sm2Encrypt)); } }
    { let op = sm3::SM3; if op.name().to_lowercase() == lowered { return Some(Box::new(sm3::SM3)); } }
    { let op = sm4_decrypt::Sm4Decrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(sm4_decrypt::Sm4Decrypt)); } }
    { let op = sm4_encrypt::Sm4Encrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(sm4_encrypt::Sm4Encrypt)); } }
    { let op = snefru::SNEFRU; if op.name().to_lowercase() == lowered { return Some(Box::new(snefru::SNEFRU)); } }
    { let op = sort::Sort; if op.name().to_lowercase() == lowered { return Some(Box::new(sort::Sort)); } }
    { let op = split::Split; if op.name().to_lowercase() == lowered { return Some(Box::new(split::Split)); } }
    { let op = split_colour_channels::SplitColourChannels; if op.name().to_lowercase() == lowered { return Some(Box::new(split_colour_channels::SplitColourChannels)); } }
    { let op = sql_beautify::SQLBeautify; if op.name().to_lowercase() == lowered { return Some(Box::new(sql_beautify::SQLBeautify)); } }
    { let op = sql_minify::SQLMinify; if op.name().to_lowercase() == lowered { return Some(Box::new(sql_minify::SQLMinify)); } }
    { let op = ssdeep::SSDEEP; if op.name().to_lowercase() == lowered { return Some(Box::new(ssdeep::SSDEEP)); } }
    { let op = standard_deviation::StandardDeviation; if op.name().to_lowercase() == lowered { return Some(Box::new(standard_deviation::StandardDeviation)); } }
    { let op = streebog::Streebog; if op.name().to_lowercase() == lowered { return Some(Box::new(streebog::Streebog)); } }
    { let op = strings::Strings; if op.name().to_lowercase() == lowered { return Some(Box::new(strings::Strings)); } }
    { let op = strip_html_tags::StripHTMLTags; if op.name().to_lowercase() == lowered { return Some(Box::new(strip_html_tags::StripHTMLTags)); } }
    { let op = strip_http_headers::StripHTTPHeaders; if op.name().to_lowercase() == lowered { return Some(Box::new(strip_http_headers::StripHTTPHeaders)); } }
    { let op = strip_ipv4_header::StripIPv4Header; if op.name().to_lowercase() == lowered { return Some(Box::new(strip_ipv4_header::StripIPv4Header)); } }
    { let op = strip_tcp_header::StripTCPHeader; if op.name().to_lowercase() == lowered { return Some(Box::new(strip_tcp_header::StripTCPHeader)); } }
    { let op = strip_udp_header::StripUDPHeader; if op.name().to_lowercase() == lowered { return Some(Box::new(strip_udp_header::StripUDPHeader)); } }
    { let op = sub::SUB; if op.name().to_lowercase() == lowered { return Some(Box::new(sub::SUB)); } }
    { let op = subsection::Subsection; if op.name().to_lowercase() == lowered { return Some(Box::new(subsection::Subsection)); } }
    { let op = substitute::Substitute; if op.name().to_lowercase() == lowered { return Some(Box::new(substitute::Substitute)); } }
    { let op = subtract::Subtract; if op.name().to_lowercase() == lowered { return Some(Box::new(subtract::Subtract)); } }
    { let op = sum::Sum; if op.name().to_lowercase() == lowered { return Some(Box::new(sum::Sum)); } }
    { let op = swap_case::SwapCase; if op.name().to_lowercase() == lowered { return Some(Box::new(swap_case::SwapCase)); } }
    { let op = swap_endianness::SwapEndianness; if op.name().to_lowercase() == lowered { return Some(Box::new(swap_endianness::SwapEndianness)); } }
    { let op = symmetric_difference::SymmetricDifference; if op.name().to_lowercase() == lowered { return Some(Box::new(symmetric_difference::SymmetricDifference)); } }
    { let op = syntax_highlighter::SyntaxHighlighter; if op.name().to_lowercase() == lowered { return Some(Box::new(syntax_highlighter::SyntaxHighlighter)); } }
    { let op = tail::Tail; if op.name().to_lowercase() == lowered { return Some(Box::new(tail::Tail)); } }
    { let op = take_bytes::TakeBytes; if op.name().to_lowercase() == lowered { return Some(Box::new(take_bytes::TakeBytes)); } }
    { let op = take_nth_bytes::TakeNthBytes; if op.name().to_lowercase() == lowered { return Some(Box::new(take_nth_bytes::TakeNthBytes)); } }
    { let op = tar::Tar; if op.name().to_lowercase() == lowered { return Some(Box::new(tar::Tar)); } }
    { let op = tcpip_checksum::TCPIPChecksum; if op.name().to_lowercase() == lowered { return Some(Box::new(tcpip_checksum::TCPIPChecksum)); } }
    { let op = template::Template; if op.name().to_lowercase() == lowered { return Some(Box::new(template::Template)); } }
    { let op = text_encoding_brute_force::TextEncodingBruteForce; if op.name().to_lowercase() == lowered { return Some(Box::new(text_encoding_brute_force::TextEncodingBruteForce)); } }
    { let op = text_integer_converter::TextIntegerConverter; if op.name().to_lowercase() == lowered { return Some(Box::new(text_integer_converter::TextIntegerConverter)); } }
    { let op = to_base::ToBase; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base::ToBase)); } }
    { let op = to_base32::ToBase32; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base32::ToBase32)); } }
    { let op = to_base45::ToBase45; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base45::ToBase45)); } }
    { let op = to_base58::ToBase58; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base58::ToBase58)); } }
    { let op = to_base62::ToBase62; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base62::ToBase62)); } }
    { let op = to_base64::ToBase64; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base64::ToBase64)); } }
    { let op = to_base85::ToBase85; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base85::ToBase85)); } }
    { let op = to_base92::ToBase92; if op.name().to_lowercase() == lowered { return Some(Box::new(to_base92::ToBase92)); } }
    { let op = to_bcd::ToBCD; if op.name().to_lowercase() == lowered { return Some(Box::new(to_bcd::ToBCD)); } }
    { let op = to_bech32::ToBech32; if op.name().to_lowercase() == lowered { return Some(Box::new(to_bech32::ToBech32)); } }
    { let op = to_binary::ToBinary; if op.name().to_lowercase() == lowered { return Some(Box::new(to_binary::ToBinary)); } }
    { let op = to_braille::ToBraille; if op.name().to_lowercase() == lowered { return Some(Box::new(to_braille::ToBraille)); } }
    { let op = to_camel_case::ToCamelCase; if op.name().to_lowercase() == lowered { return Some(Box::new(to_camel_case::ToCamelCase)); } }
    { let op = to_case_insensitive_regex::ToCaseInsensitiveRegex; if op.name().to_lowercase() == lowered { return Some(Box::new(to_case_insensitive_regex::ToCaseInsensitiveRegex)); } }
    { let op = to_charcode::ToCharcode; if op.name().to_lowercase() == lowered { return Some(Box::new(to_charcode::ToCharcode)); } }
    { let op = to_decimal::ToDecimal; if op.name().to_lowercase() == lowered { return Some(Box::new(to_decimal::ToDecimal)); } }
    { let op = to_float::ToFloat; if op.name().to_lowercase() == lowered { return Some(Box::new(to_float::ToFloat)); } }
    { let op = to_hex::ToHex; if op.name().to_lowercase() == lowered { return Some(Box::new(to_hex::ToHex)); } }
    { let op = to_hex_content::ToHexContent; if op.name().to_lowercase() == lowered { return Some(Box::new(to_hex_content::ToHexContent)); } }
    { let op = to_hexdump::ToHexdump; if op.name().to_lowercase() == lowered { return Some(Box::new(to_hexdump::ToHexdump)); } }
    { let op = to_html_entity::ToHTMLEntity; if op.name().to_lowercase() == lowered { return Some(Box::new(to_html_entity::ToHTMLEntity)); } }
    { let op = to_kebab_case::ToKebabCase; if op.name().to_lowercase() == lowered { return Some(Box::new(to_kebab_case::ToKebabCase)); } }
    { let op = to_lower_case::ToLowerCase; if op.name().to_lowercase() == lowered { return Some(Box::new(to_lower_case::ToLowerCase)); } }
    { let op = to_message_pack::ToMessagePack; if op.name().to_lowercase() == lowered { return Some(Box::new(to_message_pack::ToMessagePack)); } }
    { let op = to_modhex::ToModhex; if op.name().to_lowercase() == lowered { return Some(Box::new(to_modhex::ToModhex)); } }
    { let op = to_morse_code::ToMorseCode; if op.name().to_lowercase() == lowered { return Some(Box::new(to_morse_code::ToMorseCode)); } }
    { let op = to_octal::ToOctal; if op.name().to_lowercase() == lowered { return Some(Box::new(to_octal::ToOctal)); } }
    { let op = to_punycode::ToPunycode; if op.name().to_lowercase() == lowered { return Some(Box::new(to_punycode::ToPunycode)); } }
    { let op = to_quoted_printable::ToQuotedPrintable; if op.name().to_lowercase() == lowered { return Some(Box::new(to_quoted_printable::ToQuotedPrintable)); } }
    { let op = to_snake_case::ToSnakeCase; if op.name().to_lowercase() == lowered { return Some(Box::new(to_snake_case::ToSnakeCase)); } }
    { let op = to_table::ToTable; if op.name().to_lowercase() == lowered { return Some(Box::new(to_table::ToTable)); } }
    { let op = to_unix_timestamp::ToUNIXTimestamp; if op.name().to_lowercase() == lowered { return Some(Box::new(to_unix_timestamp::ToUNIXTimestamp)); } }
    { let op = to_upper_case::ToUpperCase; if op.name().to_lowercase() == lowered { return Some(Box::new(to_upper_case::ToUpperCase)); } }
    { let op = translate_date_time_format::TranslateDateTimeFormat; if op.name().to_lowercase() == lowered { return Some(Box::new(translate_date_time_format::TranslateDateTimeFormat)); } }
    { let op = triple_des_decrypt::TripleDESDecrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(triple_des_decrypt::TripleDESDecrypt)); } }
    { let op = triple_des_encrypt::TripleDESEncrypt; if op.name().to_lowercase() == lowered { return Some(Box::new(triple_des_encrypt::TripleDESEncrypt)); } }
    { let op = typex::Typex; if op.name().to_lowercase() == lowered { return Some(Box::new(typex::Typex)); } }
    { let op = unescape_string::UnescapeString; if op.name().to_lowercase() == lowered { return Some(Box::new(unescape_string::UnescapeString)); } }
    { let op = unescape_unicode_characters::UnescapeUnicodeCharacters; if op.name().to_lowercase() == lowered { return Some(Box::new(unescape_unicode_characters::UnescapeUnicodeCharacters)); } }
    { let op = unicode_text_format::UnicodeTextFormat; if op.name().to_lowercase() == lowered { return Some(Box::new(unicode_text_format::UnicodeTextFormat)); } }
    { let op = unique::Unique; if op.name().to_lowercase() == lowered { return Some(Box::new(unique::Unique)); } }
    { let op = unix_timestamp_to_windows_filetime::UNIXTimestampToWindowsFiletime; if op.name().to_lowercase() == lowered { return Some(Box::new(unix_timestamp_to_windows_filetime::UNIXTimestampToWindowsFiletime)); } }
    { let op = untar::Untar; if op.name().to_lowercase() == lowered { return Some(Box::new(untar::Untar)); } }
    { let op = unzip::Unzip; if op.name().to_lowercase() == lowered { return Some(Box::new(unzip::Unzip)); } }
    { let op = url_decode::URLDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(url_decode::URLDecode)); } }
    { let op = url_encode::URLEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(url_encode::URLEncode)); } }
    { let op = varint_decode::VarIntDecode; if op.name().to_lowercase() == lowered { return Some(Box::new(varint_decode::VarIntDecode)); } }
    { let op = varint_encode::VarIntEncode; if op.name().to_lowercase() == lowered { return Some(Box::new(varint_encode::VarIntEncode)); } }
    { let op = view_bit_plane::ViewBitPlane; if op.name().to_lowercase() == lowered { return Some(Box::new(view_bit_plane::ViewBitPlane)); } }
    { let op = vigenere_decode::VigenereDecodeOp; if op.name().to_lowercase() == lowered { return Some(Box::new(vigenere_decode::VigenereDecodeOp)); } }
    { let op = vigenere_encode::VigenereEncodeOp; if op.name().to_lowercase() == lowered { return Some(Box::new(vigenere_encode::VigenereEncodeOp)); } }
    { let op = whirlpool::WHIRLPOOL; if op.name().to_lowercase() == lowered { return Some(Box::new(whirlpool::WHIRLPOOL)); } }
    { let op = windows_filetime_to_unix_timestamp::WindowsFiletimeToUnixTimestampOp; if op.name().to_lowercase() == lowered { return Some(Box::new(windows_filetime_to_unix_timestamp::WindowsFiletimeToUnixTimestampOp)); } }
    { let op = wrap::WrapOp; if op.name().to_lowercase() == lowered { return Some(Box::new(wrap::WrapOp)); } }
    { let op = x_path_expression::XPathExpression; if op.name().to_lowercase() == lowered { return Some(Box::new(x_path_expression::XPathExpression)); } }
    { let op = x_salsa20::XSalsa20Op; if op.name().to_lowercase() == lowered { return Some(Box::new(x_salsa20::XSalsa20Op)); } }
    { let op = xkcd_random_number::XkcdRandomNumberOp; if op.name().to_lowercase() == lowered { return Some(Box::new(xkcd_random_number::XkcdRandomNumberOp)); } }
    { let op = xml_beautify::XMLBeautify; if op.name().to_lowercase() == lowered { return Some(Box::new(xml_beautify::XMLBeautify)); } }
    { let op = xml_minify::XMLMinify; if op.name().to_lowercase() == lowered { return Some(Box::new(xml_minify::XMLMinify)); } }
    { let op = xor::XorOp; if op.name().to_lowercase() == lowered { return Some(Box::new(xor::XorOp)); } }
    { let op = xor_brute_force::XORBruteForce; if op.name().to_lowercase() == lowered { return Some(Box::new(xor_brute_force::XORBruteForce)); } }
    { let op = xor_checksum::XORChecksum; if op.name().to_lowercase() == lowered { return Some(Box::new(xor_checksum::XORChecksum)); } }
    { let op = xxtea_decrypt::XxteaDecryptOp; if op.name().to_lowercase() == lowered { return Some(Box::new(xxtea_decrypt::XxteaDecryptOp)); } }
    { let op = xxtea_encrypt::XxteaEncryptOp; if op.name().to_lowercase() == lowered { return Some(Box::new(xxtea_encrypt::XxteaEncryptOp)); } }
    { let op = yaml_to_json::YAMLToJSON; if op.name().to_lowercase() == lowered { return Some(Box::new(yaml_to_json::YAMLToJSON)); } }
    { let op = yara_rules::YARARules; if op.name().to_lowercase() == lowered { return Some(Box::new(yara_rules::YARARules)); } }
    { let op = zip::ZipOp; if op.name().to_lowercase() == lowered { return Some(Box::new(zip::ZipOp)); } }
    { let op = zlib_deflate::ZlibDeflate; if op.name().to_lowercase() == lowered { return Some(Box::new(zlib_deflate::ZlibDeflate)); } }
    { let op = zlib_inflate::ZlibInflate; if op.name().to_lowercase() == lowered { return Some(Box::new(zlib_inflate::ZlibInflate)); } }
    None
}
