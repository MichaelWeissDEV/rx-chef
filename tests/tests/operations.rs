// Per-operation integration tests.
//
// Each submodule corresponds to one operation.
// Run all:        cargo test -p cyberchef-rust-tests --test operations
// Run one op:     cargo test -p cyberchef-rust-tests --test operations to_hex

#[path = "operations/a1z26_cipher_decode.rs"]
mod a1z26_cipher_decode;

#[path = "operations/a1z26_cipher_encode.rs"]
mod a1z26_cipher_encode;

#[path = "operations/add.rs"]
mod add;

#[path = "operations/add_line_numbers.rs"]
mod add_line_numbers;

#[path = "operations/add_text_to_image.rs"]
mod add_text_to_image;

#[path = "operations/adler32_checksum.rs"]
mod adler32_checksum;

#[path = "operations/aes_decrypt.rs"]
mod aes_decrypt;

#[path = "operations/aes_encrypt.rs"]
mod aes_encrypt;

#[path = "operations/aes_key_unwrap.rs"]
mod aes_key_unwrap;

#[path = "operations/aes_key_wrap.rs"]
mod aes_key_wrap;

#[path = "operations/affine_cipher_decode.rs"]
mod affine_cipher_decode;

#[path = "operations/affine_cipher_encode.rs"]
mod affine_cipher_encode;

#[path = "operations/alternating_caps.rs"]
mod alternating_caps;

#[path = "operations/amf_decode.rs"]
mod amf_decode;

#[path = "operations/amf_encode.rs"]
mod amf_encode;

#[path = "operations/analyse_hash.rs"]
mod analyse_hash;

#[path = "operations/analyse_uuid.rs"]
mod analyse_uuid;

#[path = "operations/and.rs"]
mod and;

#[path = "operations/argon2.rs"]
mod argon2;

#[path = "operations/argon2_compare.rs"]
mod argon2_compare;

#[path = "operations/atbash_cipher.rs"]
mod atbash_cipher;

#[path = "operations/avro_to_json.rs"]
mod avro_to_json;

#[path = "operations/bacon_cipher_decode.rs"]
mod bacon_cipher_decode;

#[path = "operations/bacon_cipher_encode.rs"]
mod bacon_cipher_encode;

#[path = "operations/bcrypt.rs"]
mod bcrypt;

#[path = "operations/bcrypt_compare.rs"]
mod bcrypt_compare;

#[path = "operations/bcrypt_parse.rs"]
mod bcrypt_parse;

#[path = "operations/bifid_cipher_decode.rs"]
mod bifid_cipher_decode;

#[path = "operations/bifid_cipher_encode.rs"]
mod bifid_cipher_encode;

#[path = "operations/bit_shift_left.rs"]
mod bit_shift_left;

#[path = "operations/bit_shift_right.rs"]
mod bit_shift_right;

#[path = "operations/blake2b.rs"]
mod blake2b;

#[path = "operations/blake2s.rs"]
mod blake2s;

#[path = "operations/blake3.rs"]
mod blake3;

#[path = "operations/blowfish_decrypt.rs"]
mod blowfish_decrypt;

#[path = "operations/blowfish_encrypt.rs"]
mod blowfish_encrypt;

#[path = "operations/blur_image.rs"]
mod blur_image;

#[path = "operations/bombe.rs"]
mod bombe;

#[path = "operations/bson_deserialise.rs"]
mod bson_deserialise;

#[path = "operations/bson_serialise.rs"]
mod bson_serialise;

#[path = "operations/bzip2_compress.rs"]
mod bzip2_compress;

#[path = "operations/bzip2_decompress.rs"]
mod bzip2_decompress;

#[path = "operations/caesar_box_cipher.rs"]
mod caesar_box_cipher;

#[path = "operations/caret_mdecode.rs"]
mod caret_mdecode;

#[path = "operations/cartesian_product.rs"]
mod cartesian_product;

#[path = "operations/cbor_decode.rs"]
mod cbor_decode;

#[path = "operations/cbor_encode.rs"]
mod cbor_encode;

#[path = "operations/cetacean_cipher_decode.rs"]
mod cetacean_cipher_decode;

#[path = "operations/cetacean_cipher_encode.rs"]
mod cetacean_cipher_encode;

#[path = "operations/chacha.rs"]
mod chacha;

#[path = "operations/change_ip_format.rs"]
mod change_ip_format;

#[path = "operations/chi_square.rs"]
mod chi_square;

#[path = "operations/cipher_saber2_decrypt.rs"]
mod cipher_saber2_decrypt;

#[path = "operations/cipher_saber2_encrypt.rs"]
mod cipher_saber2_encrypt;

#[path = "operations/citrix_ctx1_decode.rs"]
mod citrix_ctx1_decode;

#[path = "operations/citrix_ctx1_encode.rs"]
mod citrix_ctx1_encode;

#[path = "operations/cmac.rs"]
mod cmac;

#[path = "operations/colossus.rs"]
mod colossus;

#[path = "operations/comment.rs"]
mod comment;

#[path = "operations/compare_ctph_hashes.rs"]
mod compare_ctph_hashes;

#[path = "operations/compare_ssdeep_hashes.rs"]
mod compare_ssdeep_hashes;

#[path = "operations/conditional_jump.rs"]
mod conditional_jump;

#[path = "operations/contain_image.rs"]
mod contain_image;

#[path = "operations/convert_area.rs"]
mod convert_area;

#[path = "operations/convert_coordinate_format.rs"]
mod convert_coordinate_format;

#[path = "operations/convert_data_units.rs"]
mod convert_data_units;

#[path = "operations/convert_distance.rs"]
mod convert_distance;

#[path = "operations/convert_image_format.rs"]
mod convert_image_format;

#[path = "operations/convert_leet_speak.rs"]
mod convert_leet_speak;

#[path = "operations/convert_mass.rs"]
mod convert_mass;

#[path = "operations/convert_speed.rs"]
mod convert_speed;

#[path = "operations/convert_to_nato_alphabet.rs"]
mod convert_to_nato_alphabet;

#[path = "operations/count_occurrences.rs"]
mod count_occurrences;

#[path = "operations/cover_image.rs"]
mod cover_image;

#[path = "operations/crc32.rs"]
mod crc32;

#[path = "operations/crc_checksum.rs"]
mod crc_checksum;

#[path = "operations/crop_image.rs"]
mod crop_image;

#[path = "operations/css_beautify.rs"]
mod css_beautify;

#[path = "operations/css_minify.rs"]
mod css_minify;

#[path = "operations/css_selector.rs"]
mod css_selector;

#[path = "operations/csv_to_json.rs"]
mod csv_to_json;

#[path = "operations/ctph.rs"]
mod ctph;

#[path = "operations/date_time_delta.rs"]
mod date_time_delta;

#[path = "operations/dechunk_http_response.rs"]
mod dechunk_http_response;

#[path = "operations/decode_netbios_name.rs"]
mod decode_netbios_name;

#[path = "operations/decode_text.rs"]
mod decode_text;

#[path = "operations/defang_ip_addresses.rs"]
mod defang_ip_addresses;

#[path = "operations/defang_url.rs"]
mod defang_url;

#[path = "operations/derive_evp_key.rs"]
mod derive_evp_key;

#[path = "operations/derive_hkdf_key.rs"]
mod derive_hkdf_key;

#[path = "operations/derive_pbkdf2_key.rs"]
mod derive_pbkdf2_key;

#[path = "operations/des_decrypt.rs"]
mod des_decrypt;

#[path = "operations/des_encrypt.rs"]
mod des_encrypt;

#[path = "operations/detect_file_type.rs"]
mod detect_file_type;

#[path = "operations/diff.rs"]
mod diff;

#[path = "operations/disassemble_arm.rs"]
mod disassemble_arm;

#[path = "operations/disassemble_x86.rs"]
mod disassemble_x86;

#[path = "operations/dither_image.rs"]
mod dither_image;

#[path = "operations/divide.rs"]
mod divide;

#[path = "operations/dns_over_https.rs"]
mod dns_over_https;

#[path = "operations/drop_bytes.rs"]
mod drop_bytes;

#[path = "operations/drop_nth_bytes.rs"]
mod drop_nth_bytes;

#[path = "operations/ecdsa_sign.rs"]
mod ecdsa_sign;

#[path = "operations/ecdsa_signature_conversion.rs"]
mod ecdsa_signature_conversion;

#[path = "operations/ecdsa_verify.rs"]
mod ecdsa_verify;

#[path = "operations/elf_info.rs"]
mod elf_info;

#[path = "operations/encode_netbios_name.rs"]
mod encode_netbios_name;

#[path = "operations/encode_text.rs"]
mod encode_text;

#[path = "operations/enigma.rs"]
mod enigma;

#[path = "operations/entropy.rs"]
mod entropy;

#[path = "operations/escape_string.rs"]
mod escape_string;

#[path = "operations/escape_unicode_characters.rs"]
mod escape_unicode_characters;

#[path = "operations/expand_alphabet_range.rs"]
mod expand_alphabet_range;

#[path = "operations/extract_audio_metadata.rs"]
mod extract_audio_metadata;

#[path = "operations/extract_dates.rs"]
mod extract_dates;

#[path = "operations/extract_domains.rs"]
mod extract_domains;

#[path = "operations/extract_email_addresses.rs"]
mod extract_email_addresses;

#[path = "operations/extract_exif.rs"]
mod extract_exif;

#[path = "operations/extract_file_paths.rs"]
mod extract_file_paths;

#[path = "operations/extract_files.rs"]
mod extract_files;

#[path = "operations/extract_hashes.rs"]
mod extract_hashes;

#[path = "operations/extract_id3.rs"]
mod extract_id3;

#[path = "operations/extract_ip_addresses.rs"]
mod extract_ip_addresses;

#[path = "operations/extract_lsb.rs"]
mod extract_lsb;

#[path = "operations/extract_mac_addresses.rs"]
mod extract_mac_addresses;

#[path = "operations/extract_rgba.rs"]
mod extract_rgba;

#[path = "operations/extract_urls.rs"]
mod extract_urls;

#[path = "operations/fang_url.rs"]
mod fang_url;

#[path = "operations/fernet_decrypt.rs"]
mod fernet_decrypt;

#[path = "operations/fernet_encrypt.rs"]
mod fernet_encrypt;

#[path = "operations/file_tree.rs"]
mod file_tree;

#[path = "operations/filter.rs"]
mod filter;

#[path = "operations/find_replace.rs"]
mod find_replace;

#[path = "operations/flask_session_decode.rs"]
mod flask_session_decode;

#[path = "operations/flask_session_sign.rs"]
mod flask_session_sign;

#[path = "operations/flask_session_verify.rs"]
mod flask_session_verify;

#[path = "operations/fletcher16_checksum.rs"]
mod fletcher16_checksum;

#[path = "operations/fletcher32_checksum.rs"]
mod fletcher32_checksum;

#[path = "operations/fletcher64_checksum.rs"]
mod fletcher64_checksum;

#[path = "operations/fletcher8_checksum.rs"]
mod fletcher8_checksum;

#[path = "operations/flip_image.rs"]
mod flip_image;

#[path = "operations/fork.rs"]
mod fork;

#[path = "operations/format_mac_addresses.rs"]
mod format_mac_addresses;

#[path = "operations/frequency_distribution.rs"]
mod frequency_distribution;

#[path = "operations/from_base.rs"]
mod from_base;

#[path = "operations/from_base32.rs"]
mod from_base32;

#[path = "operations/from_base45.rs"]
mod from_base45;

#[path = "operations/from_base58.rs"]
mod from_base58;

#[path = "operations/from_base62.rs"]
mod from_base62;

#[path = "operations/from_base64.rs"]
mod from_base64;

#[path = "operations/from_base85.rs"]
mod from_base85;

#[path = "operations/from_base92.rs"]
mod from_base92;

#[path = "operations/from_bcd.rs"]
mod from_bcd;

#[path = "operations/from_bech32.rs"]
mod from_bech32;

#[path = "operations/from_binary.rs"]
mod from_binary;

#[path = "operations/from_braille.rs"]
mod from_braille;

#[path = "operations/from_case_insensitive_regex.rs"]
mod from_case_insensitive_regex;

#[path = "operations/from_charcode.rs"]
mod from_charcode;

#[path = "operations/from_decimal.rs"]
mod from_decimal;

#[path = "operations/from_float.rs"]
mod from_float;

#[path = "operations/from_hex.rs"]
mod from_hex;

#[path = "operations/from_hex_content.rs"]
mod from_hex_content;

#[path = "operations/from_hexdump.rs"]
mod from_hexdump;

#[path = "operations/from_html_entity.rs"]
mod from_html_entity;

#[path = "operations/from_message_pack.rs"]
mod from_message_pack;

#[path = "operations/from_modhex.rs"]
mod from_modhex;

#[path = "operations/from_morse_code.rs"]
mod from_morse_code;

#[path = "operations/from_octal.rs"]
mod from_octal;

#[path = "operations/from_punycode.rs"]
mod from_punycode;

#[path = "operations/from_quoted_printable.rs"]
mod from_quoted_printable;

#[path = "operations/from_unix_timestamp.rs"]
mod from_unix_timestamp;

#[path = "operations/fuzzy_match.rs"]
mod fuzzy_match;

#[path = "operations/generate_all_checksums.rs"]
mod generate_all_checksums;

#[path = "operations/generate_all_hashes.rs"]
mod generate_all_hashes;

#[path = "operations/generate_de_bruijn_sequence.rs"]
mod generate_de_bruijn_sequence;

#[path = "operations/generate_ecdsa_key_pair.rs"]
mod generate_ecdsa_key_pair;

#[path = "operations/generate_hotp.rs"]
mod generate_hotp;

#[path = "operations/generate_image.rs"]
mod generate_image;

#[path = "operations/generate_lorem_ipsum.rs"]
mod generate_lorem_ipsum;

#[path = "operations/generate_pgp_key_pair.rs"]
mod generate_pgp_key_pair;

#[path = "operations/generate_qr_code.rs"]
mod generate_qr_code;

#[path = "operations/generate_rsa_key_pair.rs"]
mod generate_rsa_key_pair;

#[path = "operations/generate_totp.rs"]
mod generate_totp;

#[path = "operations/generate_uuid.rs"]
mod generate_uuid;

#[path = "operations/generic_code_beautify.rs"]
mod generic_code_beautify;

#[path = "operations/get_all_casings.rs"]
mod get_all_casings;

#[path = "operations/get_time.rs"]
mod get_time;

#[path = "operations/gost_decrypt.rs"]
mod gost_decrypt;

#[path = "operations/gost_encrypt.rs"]
mod gost_encrypt;

#[path = "operations/gost_hash.rs"]
mod gost_hash;

#[path = "operations/gost_key_unwrap.rs"]
mod gost_key_unwrap;

#[path = "operations/gost_key_wrap.rs"]
mod gost_key_wrap;

#[path = "operations/gost_sign.rs"]
mod gost_sign;

#[path = "operations/gost_verify.rs"]
mod gost_verify;

#[path = "operations/group_ip_addresses.rs"]
mod group_ip_addresses;

#[path = "operations/gunzip.rs"]
mod gunzip;

#[path = "operations/gzip.rs"]
mod gzip;

#[path = "operations/hamming_distance.rs"]
mod hamming_distance;

#[path = "operations/has160.rs"]
mod has160;

#[path = "operations/hassh_client_fingerprint.rs"]
mod hassh_client_fingerprint;

#[path = "operations/hassh_server_fingerprint.rs"]
mod hassh_server_fingerprint;

#[path = "operations/haversine_distance.rs"]
mod haversine_distance;

#[path = "operations/head.rs"]
mod head;

#[path = "operations/heatmap_chart.rs"]
mod heatmap_chart;

#[path = "operations/hex_density_chart.rs"]
mod hex_density_chart;

#[path = "operations/hex_to_object_identifier.rs"]
mod hex_to_object_identifier;

#[path = "operations/hex_to_pem.rs"]
mod hex_to_pem;

#[path = "operations/hmac.rs"]
mod hmac;

#[path = "operations/html_to_text.rs"]
mod html_to_text;

#[path = "operations/http_request.rs"]
mod http_request;

#[path = "operations/image_brightness_contrast.rs"]
mod image_brightness_contrast;

#[path = "operations/image_filter.rs"]
mod image_filter;

#[path = "operations/image_hue_saturation_lightness.rs"]
mod image_hue_saturation_lightness;

#[path = "operations/image_opacity.rs"]
mod image_opacity;

#[path = "operations/index_of_coincidence.rs"]
mod index_of_coincidence;

#[path = "operations/invert_image.rs"]
mod invert_image;

#[path = "operations/ipv6_transition_addresses.rs"]
mod ipv6_transition_addresses;

#[path = "operations/ja3_fingerprint.rs"]
mod ja3_fingerprint;

#[path = "operations/ja3s_fingerprint.rs"]
mod ja3s_fingerprint;

#[path = "operations/ja4_fingerprint.rs"]
mod ja4_fingerprint;

#[path = "operations/ja4_server_fingerprint.rs"]
mod ja4_server_fingerprint;

#[path = "operations/java_script_beautify.rs"]
mod java_script_beautify;

#[path = "operations/java_script_minify.rs"]
mod java_script_minify;

#[path = "operations/java_script_parser.rs"]
mod java_script_parser;

#[path = "operations/j_path_expression.rs"]
mod j_path_expression;

#[path = "operations/jq.rs"]
mod jq;

#[path = "operations/jsonata.rs"]
mod jsonata;

#[path = "operations/json_beautify.rs"]
mod json_beautify;

#[path = "operations/json_minify.rs"]
mod json_minify;

#[path = "operations/json_to_csv.rs"]
mod json_to_csv;

#[path = "operations/json_to_yaml.rs"]
mod json_to_yaml;

#[path = "operations/jump.rs"]
mod jump;

#[path = "operations/jwk_to_pem.rs"]
mod jwk_to_pem;

#[path = "operations/jwt_decode.rs"]
mod jwt_decode;

#[path = "operations/jwt_sign.rs"]
mod jwt_sign;

#[path = "operations/jwt_verify.rs"]
mod jwt_verify;

#[path = "operations/keccak.rs"]
mod keccak;

#[path = "operations/label.rs"]
mod label;

#[path = "operations/levenshtein_distance.rs"]
mod levenshtein_distance;

#[path = "operations/lm_hash.rs"]
mod lm_hash;

#[path = "operations/lorenz.rs"]
mod lorenz;

#[path = "operations/ls47_decrypt.rs"]
mod ls47_decrypt;

#[path = "operations/ls47_encrypt.rs"]
mod ls47_encrypt;

#[path = "operations/luhn_checksum.rs"]
mod luhn_checksum;

#[path = "operations/lz4_compress.rs"]
mod lz4_compress;

#[path = "operations/lz4_decompress.rs"]
mod lz4_decompress;

#[path = "operations/lzma_compress.rs"]
mod lzma_compress;

#[path = "operations/lzma_decompress.rs"]
mod lzma_decompress;

#[path = "operations/lznt1_decompress.rs"]
mod lznt1_decompress;

#[path = "operations/lz_string_compress.rs"]
mod lz_string_compress;

#[path = "operations/lz_string_decompress.rs"]
mod lz_string_decompress;

#[path = "operations/magic.rs"]
mod magic;

#[path = "operations/md2.rs"]
mod md2;

#[path = "operations/md4.rs"]
mod md4;

#[path = "operations/md5.rs"]
mod md5;

#[path = "operations/md6.rs"]
mod md6;

#[path = "operations/mean.rs"]
mod mean;

#[path = "operations/median.rs"]
mod median;

#[path = "operations/merge.rs"]
mod merge;

#[path = "operations/microsoft_script_decoder.rs"]
mod microsoft_script_decoder;

#[path = "operations/mime_decoding.rs"]
mod mime_decoding;

#[path = "operations/multiple_bombe.rs"]
mod multiple_bombe;

#[path = "operations/multiply.rs"]
mod multiply;

#[path = "operations/murmur_hash3.rs"]
mod murmur_hash3;

#[path = "operations/normalise_image.rs"]
mod normalise_image;

#[path = "operations/normalise_unicode.rs"]
mod normalise_unicode;

#[path = "operations/not.rs"]
mod not;

#[path = "operations/nt_hash.rs"]
mod nt_hash;

#[path = "operations/numberwang.rs"]
mod numberwang;

#[path = "operations/object_identifier_to_hex.rs"]
mod object_identifier_to_hex;

#[path = "operations/offset_checker.rs"]
mod offset_checker;

#[path = "operations/optical_character_recognition.rs"]
mod optical_character_recognition;

#[path = "operations/or.rs"]
mod or;

#[path = "operations/pad_lines.rs"]
mod pad_lines;

#[path = "operations/parity_bit.rs"]
mod parity_bit;

#[path = "operations/parse_asn1_hex_string.rs"]
mod parse_asn1_hex_string;

#[path = "operations/parse_colour_code.rs"]
mod parse_colour_code;

#[path = "operations/parse_csr.rs"]
mod parse_csr;

#[path = "operations/parse_date_time.rs"]
mod parse_date_time;

#[path = "operations/parse_ethernet_frame.rs"]
mod parse_ethernet_frame;

#[path = "operations/parse_ip_range.rs"]
mod parse_ip_range;

#[path = "operations/parse_ipv4_header.rs"]
mod parse_ipv4_header;

#[path = "operations/parse_ipv6_address.rs"]
mod parse_ipv6_address;

#[path = "operations/parse_object_id_timestamp.rs"]
mod parse_object_id_timestamp;

#[path = "operations/parse_qr_code.rs"]
mod parse_qr_code;

#[path = "operations/parse_ssh_host_key.rs"]
mod parse_ssh_host_key;

#[path = "operations/parse_tcp.rs"]
mod parse_tcp;

#[path = "operations/parse_tls_record.rs"]
mod parse_tls_record;

#[path = "operations/parse_tlv.rs"]
mod parse_tlv;

#[path = "operations/parse_udp.rs"]
mod parse_udp;

#[path = "operations/parse_unix_file_permissions.rs"]
mod parse_unix_file_permissions;

#[path = "operations/parse_uri.rs"]
mod parse_uri;

#[path = "operations/parse_user_agent.rs"]
mod parse_user_agent;

#[path = "operations/parse_x509_certificate.rs"]
mod parse_x509_certificate;

#[path = "operations/parse_x509_crl.rs"]
mod parse_x509_crl;

#[path = "operations/pem_to_hex.rs"]
mod pem_to_hex;

#[path = "operations/pem_to_jwk.rs"]
mod pem_to_jwk;

#[path = "operations/pgp_decrypt.rs"]
mod pgp_decrypt;

#[path = "operations/pgp_decrypt_and_verify.rs"]
mod pgp_decrypt_and_verify;

#[path = "operations/pgp_encrypt.rs"]
mod pgp_encrypt;

#[path = "operations/pgp_encrypt_and_sign.rs"]
mod pgp_encrypt_and_sign;

#[path = "operations/pgp_verify.rs"]
mod pgp_verify;

#[path = "operations/php_deserialize.rs"]
mod php_deserialize;

#[path = "operations/php_serialize.rs"]
mod php_serialize;

#[path = "operations/play_media.rs"]
mod play_media;

#[path = "operations/plist_viewer.rs"]
mod plist_viewer;

#[path = "operations/power_set.rs"]
mod power_set;

#[path = "operations/protobuf_decode.rs"]
mod protobuf_decode;

#[path = "operations/protobuf_encode.rs"]
mod protobuf_encode;

#[path = "operations/pseudo_random_integer_generator.rs"]
mod pseudo_random_integer_generator;

#[path = "operations/pseudo_random_number_generator.rs"]
mod pseudo_random_number_generator;

#[path = "operations/pub_key_from_cert.rs"]
mod pub_key_from_cert;

#[path = "operations/pub_key_from_priv_key.rs"]
mod pub_key_from_priv_key;

#[path = "operations/rabbit.rs"]
mod rabbit;

#[path = "operations/rail_fence_cipher_decode.rs"]
mod rail_fence_cipher_decode;

#[path = "operations/rail_fence_cipher_encode.rs"]
mod rail_fence_cipher_encode;

#[path = "operations/rake.rs"]
mod rake;

#[path = "operations/randomize_colour_palette.rs"]
mod randomize_colour_palette;

#[path = "operations/raw_deflate.rs"]
mod raw_deflate;

#[path = "operations/raw_inflate.rs"]
mod raw_inflate;

#[path = "operations/rc2_decrypt.rs"]
mod rc2_decrypt;

#[path = "operations/rc2_encrypt.rs"]
mod rc2_encrypt;

#[path = "operations/rc4.rs"]
mod rc4;

#[path = "operations/rc4_drop.rs"]
mod rc4_drop;

#[path = "operations/rc6_decrypt.rs"]
mod rc6_decrypt;

#[path = "operations/rc6_encrypt.rs"]
mod rc6_encrypt;

#[path = "operations/register.rs"]
mod register;

#[path = "operations/regular_expression.rs"]
mod regular_expression;

#[path = "operations/remove_diacritics.rs"]
mod remove_diacritics;

#[path = "operations/remove_exif.rs"]
mod remove_exif;

#[path = "operations/remove_line_numbers.rs"]
mod remove_line_numbers;

#[path = "operations/remove_null_bytes.rs"]
mod remove_null_bytes;

#[path = "operations/remove_whitespace.rs"]
mod remove_whitespace;

#[path = "operations/render_image.rs"]
mod render_image;

#[path = "operations/render_markdown.rs"]
mod render_markdown;

#[path = "operations/resize_image.rs"]
mod resize_image;

#[path = "operations/return_op.rs"]
mod return_op;

#[path = "operations/reverse.rs"]
mod reverse;

#[path = "operations/ripemd.rs"]
mod ripemd;

#[path = "operations/rison_decode.rs"]
mod rison_decode;

#[path = "operations/rison_encode.rs"]
mod rison_encode;

#[path = "operations/rot13.rs"]
mod rot13;

#[path = "operations/rot13_brute_force.rs"]
mod rot13_brute_force;

#[path = "operations/rot47.rs"]
mod rot47;

#[path = "operations/rot47_brute_force.rs"]
mod rot47_brute_force;

#[path = "operations/rot8000.rs"]
mod rot8000;

#[path = "operations/rotate_image.rs"]
mod rotate_image;

#[path = "operations/rotate_left.rs"]
mod rotate_left;

#[path = "operations/rotate_right.rs"]
mod rotate_right;

#[path = "operations/rsa_decrypt.rs"]
mod rsa_decrypt;

#[path = "operations/rsa_encrypt.rs"]
mod rsa_encrypt;

#[path = "operations/rsa_sign.rs"]
mod rsa_sign;

#[path = "operations/rsa_verify.rs"]
mod rsa_verify;

#[path = "operations/salsa20.rs"]
mod salsa20;

#[path = "operations/scan_for_embedded_files.rs"]
mod scan_for_embedded_files;

#[path = "operations/scatter_chart.rs"]
mod scatter_chart;

#[path = "operations/scrypt.rs"]
mod scrypt;

#[path = "operations/series_chart.rs"]
mod series_chart;

#[path = "operations/set_difference.rs"]
mod set_difference;

#[path = "operations/set_intersection.rs"]
mod set_intersection;

#[path = "operations/set_union.rs"]
mod set_union;

#[path = "operations/sha0.rs"]
mod sha0;

#[path = "operations/sha1.rs"]
mod sha1;

#[path = "operations/sha2.rs"]
mod sha2;

#[path = "operations/sha3.rs"]
mod sha3;

#[path = "operations/shake.rs"]
mod shake;

#[path = "operations/sharpen_image.rs"]
mod sharpen_image;

#[path = "operations/show_base64_offsets.rs"]
mod show_base64_offsets;

#[path = "operations/show_on_map.rs"]
mod show_on_map;

#[path = "operations/shuffle.rs"]
mod shuffle;

#[path = "operations/sigaba.rs"]
mod sigaba;

#[path = "operations/sleep.rs"]
mod sleep;

#[path = "operations/sm2_decrypt.rs"]
mod sm2_decrypt;

#[path = "operations/sm2_encrypt.rs"]
mod sm2_encrypt;

#[path = "operations/sm3.rs"]
mod sm3;

#[path = "operations/sm4_decrypt.rs"]
mod sm4_decrypt;

#[path = "operations/sm4_encrypt.rs"]
mod sm4_encrypt;

#[path = "operations/snefru.rs"]
mod snefru;

#[path = "operations/sort.rs"]
mod sort;

#[path = "operations/split.rs"]
mod split;

#[path = "operations/split_colour_channels.rs"]
mod split_colour_channels;

#[path = "operations/sql_beautify.rs"]
mod sql_beautify;

#[path = "operations/sql_minify.rs"]
mod sql_minify;

#[path = "operations/ssdeep.rs"]
mod ssdeep;

#[path = "operations/standard_deviation.rs"]
mod standard_deviation;

#[path = "operations/streebog.rs"]
mod streebog;

#[path = "operations/strings.rs"]
mod strings;

#[path = "operations/strip_html_tags.rs"]
mod strip_html_tags;

#[path = "operations/strip_http_headers.rs"]
mod strip_http_headers;

#[path = "operations/strip_ipv4_header.rs"]
mod strip_ipv4_header;

#[path = "operations/strip_tcp_header.rs"]
mod strip_tcp_header;

#[path = "operations/strip_udp_header.rs"]
mod strip_udp_header;

#[path = "operations/sub.rs"]
mod sub;

#[path = "operations/subsection.rs"]
mod subsection;

#[path = "operations/substitute.rs"]
mod substitute;

#[path = "operations/subtract.rs"]
mod subtract;

#[path = "operations/sum.rs"]
mod sum;

#[path = "operations/swap_case.rs"]
mod swap_case;

#[path = "operations/swap_endianness.rs"]
mod swap_endianness;

#[path = "operations/symmetric_difference.rs"]
mod symmetric_difference;

#[path = "operations/syntax_highlighter.rs"]
mod syntax_highlighter;

#[path = "operations/tail.rs"]
mod tail;

#[path = "operations/take_bytes.rs"]
mod take_bytes;

#[path = "operations/take_nth_bytes.rs"]
mod take_nth_bytes;

#[path = "operations/tar.rs"]
mod tar;

#[path = "operations/tcpip_checksum.rs"]
mod tcpip_checksum;

#[path = "operations/template.rs"]
mod template;

#[path = "operations/test_x509.rs"]
mod test_x509;

#[path = "operations/text_encoding_brute_force.rs"]
mod text_encoding_brute_force;

#[path = "operations/text_integer_converter.rs"]
mod text_integer_converter;

#[path = "operations/to_base.rs"]
mod to_base;

#[path = "operations/to_base32.rs"]
mod to_base32;

#[path = "operations/to_base45.rs"]
mod to_base45;

#[path = "operations/to_base58.rs"]
mod to_base58;

#[path = "operations/to_base62.rs"]
mod to_base62;

#[path = "operations/to_base64.rs"]
mod to_base64;

#[path = "operations/to_base85.rs"]
mod to_base85;

#[path = "operations/to_base92.rs"]
mod to_base92;

#[path = "operations/to_bcd.rs"]
mod to_bcd;

#[path = "operations/to_bech32.rs"]
mod to_bech32;

#[path = "operations/to_binary.rs"]
mod to_binary;

#[path = "operations/to_braille.rs"]
mod to_braille;

#[path = "operations/to_camel_case.rs"]
mod to_camel_case;

#[path = "operations/to_case_insensitive_regex.rs"]
mod to_case_insensitive_regex;

#[path = "operations/to_charcode.rs"]
mod to_charcode;

#[path = "operations/to_decimal.rs"]
mod to_decimal;

#[path = "operations/to_float.rs"]
mod to_float;

#[path = "operations/to_hex.rs"]
mod to_hex;

#[path = "operations/to_hex_content.rs"]
mod to_hex_content;

#[path = "operations/to_hexdump.rs"]
mod to_hexdump;

#[path = "operations/to_html_entity.rs"]
mod to_html_entity;

#[path = "operations/to_kebab_case.rs"]
mod to_kebab_case;

#[path = "operations/to_lower_case.rs"]
mod to_lower_case;

#[path = "operations/to_message_pack.rs"]
mod to_message_pack;

#[path = "operations/to_modhex.rs"]
mod to_modhex;

#[path = "operations/to_morse_code.rs"]
mod to_morse_code;

#[path = "operations/to_octal.rs"]
mod to_octal;

#[path = "operations/to_punycode.rs"]
mod to_punycode;

#[path = "operations/to_quoted_printable.rs"]
mod to_quoted_printable;

#[path = "operations/to_snake_case.rs"]
mod to_snake_case;

#[path = "operations/to_table.rs"]
mod to_table;

#[path = "operations/to_unix_timestamp.rs"]
mod to_unix_timestamp;

#[path = "operations/to_upper_case.rs"]
mod to_upper_case;

#[path = "operations/translate_date_time_format.rs"]
mod translate_date_time_format;

#[path = "operations/triple_des_decrypt.rs"]
mod triple_des_decrypt;

#[path = "operations/triple_des_encrypt.rs"]
mod triple_des_encrypt;

#[path = "operations/typex.rs"]
mod typex;

#[path = "operations/unescape_string.rs"]
mod unescape_string;

#[path = "operations/unescape_unicode_characters.rs"]
mod unescape_unicode_characters;

#[path = "operations/unicode_text_format.rs"]
mod unicode_text_format;

#[path = "operations/unique.rs"]
mod unique;

#[path = "operations/unix_timestamp_to_windows_filetime.rs"]
mod unix_timestamp_to_windows_filetime;

#[path = "operations/untar.rs"]
mod untar;

#[path = "operations/unzip.rs"]
mod unzip;

#[path = "operations/url_decode.rs"]
mod url_decode;

#[path = "operations/url_encode.rs"]
mod url_encode;

#[path = "operations/varint_decode.rs"]
mod varint_decode;

#[path = "operations/varint_encode.rs"]
mod varint_encode;

#[path = "operations/view_bit_plane.rs"]
mod view_bit_plane;

#[path = "operations/vigenere_decode.rs"]
mod vigenere_decode;

#[path = "operations/vigenere_encode.rs"]
mod vigenere_encode;

#[path = "operations/whirlpool.rs"]
mod whirlpool;

#[path = "operations/windows_filetime_to_unix_timestamp.rs"]
mod windows_filetime_to_unix_timestamp;

#[path = "operations/wrap.rs"]
mod wrap;

#[path = "operations/xkcd_random_number.rs"]
mod xkcd_random_number;

#[path = "operations/xml_beautify.rs"]
mod xml_beautify;

#[path = "operations/xml_minify.rs"]
mod xml_minify;

#[path = "operations/xor.rs"]
mod xor;

#[path = "operations/xor_brute_force.rs"]
mod xor_brute_force;

#[path = "operations/xor_checksum.rs"]
mod xor_checksum;

#[path = "operations/x_path_expression.rs"]
mod x_path_expression;

#[path = "operations/x_salsa20.rs"]
mod x_salsa20;

#[path = "operations/xxtea_decrypt.rs"]
mod xxtea_decrypt;

#[path = "operations/xxtea_encrypt.rs"]
mod xxtea_encrypt;

#[path = "operations/yaml_to_json.rs"]
mod yaml_to_json;

#[path = "operations/yara_rules.rs"]
mod yara_rules;

#[path = "operations/zip.rs"]
mod zip;

#[path = "operations/zlib_deflate.rs"]
mod zlib_deflate;

#[path = "operations/zlib_inflate.rs"]
mod zlib_inflate;
