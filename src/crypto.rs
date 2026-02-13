#![allow(dead_code)]
use aes::Aes256;
use base64::{Engine as _, engine::general_purpose};
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use md5::Md5;
use napi_derive::napi;
use rand::RngCore;
use rand::rngs::OsRng;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256, Sha512};

const ROT13_SHIFT: u8 = 13;
const ALPHABET_SIZE: u8 = 26;
const ASCII_LOWER_A: u8 = b'a';
const ASCII_UPPER_A: u8 = b'A';
const AES_KEY_BYTES: usize = 32;
const AES_BLOCK_BYTES: usize = 16;
const AES_IV_BYTES: usize = 16;
const AES_IV_HEX_CHARS: usize = AES_IV_BYTES * 2;
const DEFAULT_RSA_BITS: u32 = 4096;
const MIN_RSA_BITS: u32 = 2048;
const MAX_RSA_BITS: u32 = 8192;
const RSA_BITS_STEP: u32 = 256;
const BASE32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

const MORSE_TABLE: &[(char, &str)] = &[
    ('a', ".-"),
    ('b', "-..."),
    ('c', "-.-."),
    ('d', "-.."),
    ('e', "."),
    ('f', "..-."),
    ('g', "--."),
    ('h', "...."),
    ('i', ".."),
    ('j', ".---"),
    ('k', "-.-"),
    ('l', ".-.."),
    ('m', "--"),
    ('n', "-."),
    ('o', "---"),
    ('p', ".--."),
    ('q', "--.-"),
    ('r', ".-."),
    ('s', "..."),
    ('t', "-"),
    ('u', "..-"),
    ('v', "...-"),
    ('w', ".--"),
    ('x', "-..-"),
    ('y', "-.--"),
    ('z', "--.."),
    ('0', "-----"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
    ('.', ".-.-.-"),
    (',', "--..--"),
    ('?', "..--.."),
    ('!', "-.-.--"),
    ('\'', ".----."),
    ('/', "-..-."),
    ('(', "-.--."),
    (')', "-.--.-"),
    ('&', ".-..."),
    (':', "---..."),
    (';', "-.-.-."),
    ('=', "-...-"),
    ('+', ".-.-."),
    ('-', "-....-"),
    ('_', "..--.-"),
    ('"', ".-..-."),
    ('$', "...-..-"),
    ('@', ".--.-."),
];

enum BinaryEncoding {
    Hex,
    Base64,
}

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

#[napi(object)]
pub struct RsaPacket {
    pub encoded: String,
    #[napi(js_name = "privateKey")]
    pub private_key: String,
    #[napi(js_name = "publicKey")]
    pub public_key: String,
    pub encoding: String,
    pub bits: u32,
}

#[inline(always)]
fn invalid_input(message: &str) -> napi::Error {
    napi::Error::from_reason(message.to_string())
}

#[inline(always)]
fn binary_encoding_name(encoding: &BinaryEncoding) -> &'static str {
    if matches!(encoding, BinaryEncoding::Hex) {
        return "hex";
    }
    "base64"
}

#[inline(always)]
fn normalize_hash_encoding(encoding: Option<String>) -> napi::Result<BinaryEncoding> {
    let value = encoding.unwrap_or_else(|| "hex".to_string());
    normalize_encoding_value(&value)
}

#[inline(always)]
fn normalize_binary_encoding(encoding: Option<String>) -> napi::Result<BinaryEncoding> {
    let value = encoding.unwrap_or_else(|| "base64".to_string());
    normalize_encoding_value(&value)
}

#[inline(always)]
fn normalize_encoding_value(value: &str) -> napi::Result<BinaryEncoding> {
    let normalized = value.to_ascii_lowercase();
    if normalized == "hex" || normalized == "base16" {
        return Ok(BinaryEncoding::Hex);
    }
    if normalized == "base64" {
        return Ok(BinaryEncoding::Base64);
    }
    Err(invalid_input(
        "Encoding must be one of: hex, base16, base64",
    ))
}

#[inline(always)]
fn is_hex_text(input: &str) -> bool {
    if !input.len().is_multiple_of(2) {
        return false;
    }
    input.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

#[inline(always)]
fn infer_binary_encoding(input: &str) -> BinaryEncoding {
    if is_hex_text(input) {
        return BinaryEncoding::Hex;
    }
    BinaryEncoding::Base64
}

#[inline(always)]
fn encode_bytes(bytes: &[u8], encoding: &BinaryEncoding) -> String {
    if matches!(encoding, BinaryEncoding::Hex) {
        return hex::encode(bytes);
    }
    general_purpose::STANDARD.encode(bytes)
}

#[inline(always)]
fn decode_bytes(input: &str, encoding: &BinaryEncoding) -> napi::Result<Vec<u8>> {
    if matches!(encoding, BinaryEncoding::Hex) {
        return hex::decode(input)
            .map_err(|error| invalid_input(&format!("Invalid hex input: {error}")));
    }
    general_purpose::STANDARD
        .decode(input.as_bytes())
        .map_err(|error| invalid_input(&format!("Invalid base64 input: {error}")))
}

#[inline(always)]
fn decode_bytes_with_optional_encoding(
    input: &str,
    encoding: Option<String>,
) -> napi::Result<Vec<u8>> {
    if let Some(value) = encoding {
        let normalized = normalize_encoding_value(&value)?;
        return decode_bytes(input, &normalized);
    }
    let inferred = infer_binary_encoding(input);
    decode_bytes(input, &inferred)
}

#[inline(always)]
fn digest_array<H, const N: usize>(input: &str) -> [u8; N]
where
    H: Digest + Default,
{
    let mut hasher = H::new();
    hasher.update(input.as_bytes());
    let digest = hasher.finalize();
    let mut output = [0_u8; N];
    output.copy_from_slice(&digest);
    output
}

#[inline(always)]
fn digest_md5(input: &str) -> [u8; 16] {
    digest_array::<Md5, 16>(input)
}

#[inline(always)]
fn digest_sha256(input: &str) -> [u8; 32] {
    digest_array::<Sha256, 32>(input)
}

#[inline(always)]
fn digest_sha512(input: &str) -> [u8; 64] {
    digest_array::<Sha512, 64>(input)
}

#[inline(always)]
fn normalize_rot_shift(shift: Option<u8>) -> u8 {
    let normalized = shift.unwrap_or(ROT13_SHIFT) % ALPHABET_SIZE;
    if normalized == 0 {
        return ROT13_SHIFT;
    }
    normalized
}

#[inline(always)]
fn rotate_letter(byte: u8, base: u8, shift: u8) -> u8 {
    ((byte - base + shift) % ALPHABET_SIZE) + base
}

#[inline(always)]
fn rotate_ascii_char(ch: char, shift: u8) -> char {
    if ch.is_ascii_lowercase() {
        return rotate_letter(ch as u8, ASCII_LOWER_A, shift) as char;
    }
    if ch.is_ascii_uppercase() {
        return rotate_letter(ch as u8, ASCII_UPPER_A, shift) as char;
    }
    ch
}

#[inline(always)]
fn base32_value(ch: u8) -> Option<u8> {
    if ch.is_ascii_uppercase() {
        return Some(ch - b'A');
    }
    if (b'2'..=b'7').contains(&ch) {
        return Some(ch - b'2' + 26);
    }
    None
}

#[inline(always)]
fn base32_encode_bytes(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    let mut buffer: u16 = 0;
    let mut bits: u8 = 0;
    for byte in input {
        buffer = (buffer << 8) | (*byte as u16);
        bits += 8;
        while bits >= 5 {
            let index = ((buffer >> (bits - 5)) & 0x1f) as usize;
            output.push(BASE32_ALPHABET[index] as char);
            bits -= 5;
        }
    }

    if bits > 0 {
        let index = ((buffer << (5 - bits)) & 0x1f) as usize;
        output.push(BASE32_ALPHABET[index] as char);
    }
    while !output.len().is_multiple_of(8) {
        output.push('=');
    }
    output
}

#[inline(always)]
fn base32_decode_bytes(input: &str) -> napi::Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits: u8 = 0;
    for raw in input.bytes() {
        if raw.is_ascii_whitespace() {
            continue;
        }
        if raw == b'=' {
            break;
        }

        let upper = raw.to_ascii_uppercase();
        let Some(value) = base32_value(upper) else {
            return Err(invalid_input("Invalid base32 input"));
        };
        buffer = (buffer << 5) | (value as u32);
        bits += 5;
        while bits >= 8 {
            let byte = ((buffer >> (bits - 8)) & 0xff) as u8;
            output.push(byte);
            bits -= 8;
        }
    }
    Ok(output)
}

#[inline(always)]
fn morse_code_for(ch: char) -> Option<&'static str> {
    let normalized = ch.to_ascii_lowercase();
    MORSE_TABLE
        .iter()
        .find(|(symbol, _)| *symbol == normalized)
        .map(|(_, code)| *code)
}

#[inline(always)]
fn morse_char_for(code: &str) -> Option<char> {
    MORSE_TABLE
        .iter()
        .find(|(_, symbol)| *symbol == code)
        .map(|(ch, _)| *ch)
}

#[inline(always)]
fn should_insert_space(output: &str) -> bool {
    !output.is_empty() && !output.ends_with(' ')
}

#[inline(always)]
fn normalize_aes_iv(iv_hex: &str) -> napi::Result<[u8; AES_IV_BYTES]> {
    if iv_hex.len() != AES_IV_HEX_CHARS {
        return Err(invalid_input(
            "AES IV must be 16 bytes encoded as 32 hex characters",
        ));
    }
    let decoded = hex::decode(iv_hex).map_err(|_| invalid_input("AES IV must be valid hex"))?;
    if decoded.len() != AES_IV_BYTES {
        return Err(invalid_input(
            "AES IV must be 16 bytes encoded as 32 hex characters",
        ));
    }

    let mut iv = [0_u8; AES_IV_BYTES];
    iv.copy_from_slice(&decoded);
    Ok(iv)
}

#[inline(always)]
fn generate_aes_iv_hex() -> String {
    let mut iv = [0_u8; AES_IV_BYTES];
    let mut rng = OsRng;
    rng.fill_bytes(&mut iv);
    hex::encode(iv)
}

#[inline(always)]
fn derive_aes_key(key: &str) -> [u8; AES_KEY_BYTES] {
    digest_sha256(key)
}

#[inline(always)]
fn aes_encrypt(
    input: &[u8],
    key: &[u8; AES_KEY_BYTES],
    iv: &[u8; AES_IV_BYTES],
) -> napi::Result<Vec<u8>> {
    let cipher = Aes256CbcEnc::new_from_slices(key, iv)
        .map_err(|_| invalid_input("AES key or IV is invalid"))?;
    let mut buffer = vec![0_u8; input.len() + AES_BLOCK_BYTES];
    buffer[..input.len()].copy_from_slice(input);
    let encrypted = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, input.len())
        .map_err(|_| invalid_input("AES encryption failed"))?;
    Ok(encrypted.to_vec())
}

#[inline(always)]
fn aes_decrypt(
    input: &[u8],
    key: &[u8; AES_KEY_BYTES],
    iv: &[u8; AES_IV_BYTES],
) -> napi::Result<Vec<u8>> {
    let cipher = Aes256CbcDec::new_from_slices(key, iv)
        .map_err(|_| invalid_input("AES key or IV is invalid"))?;
    let mut buffer = input.to_vec();
    let decrypted = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|_| invalid_input("AES decryption failed"))?;
    Ok(decrypted.to_vec())
}

#[inline(always)]
fn normalize_rsa_bits(bits: Option<u32>) -> napi::Result<u32> {
    let value = bits.unwrap_or(DEFAULT_RSA_BITS);
    if !(MIN_RSA_BITS..=MAX_RSA_BITS).contains(&value) {
        return Err(invalid_input("RSA bits must be in range 2048..=8192"));
    }
    if !value.is_multiple_of(RSA_BITS_STEP) {
        return Err(invalid_input("RSA bits must be a multiple of 256"));
    }
    Ok(value)
}

#[inline(always)]
fn normalize_rsa_encoding(encoding: Option<String>) -> napi::Result<BinaryEncoding> {
    normalize_binary_encoding(encoding)
}

#[inline(always)]
fn rsa_generate_keys(bits: u32) -> napi::Result<(RsaPrivateKey, RsaPublicKey)> {
    let mut rng = OsRng;
    let private = RsaPrivateKey::new(&mut rng, bits as usize)
        .map_err(|error| invalid_input(&format!("RSA key generation failed: {error}")))?;
    let public = RsaPublicKey::from(&private);
    Ok((private, public))
}

#[inline(always)]
fn rsa_private_to_pem(private: &RsaPrivateKey) -> napi::Result<String> {
    private
        .to_pkcs8_pem(LineEnding::LF)
        .map(|pem| pem.to_string())
        .map_err(|error| invalid_input(&format!("RSA private key export failed: {error}")))
}

#[inline(always)]
fn rsa_public_to_pem(public: &RsaPublicKey) -> napi::Result<String> {
    public
        .to_public_key_pem(LineEnding::LF)
        .map(|pem| pem.to_string())
        .map_err(|error| invalid_input(&format!("RSA public key export failed: {error}")))
}

#[inline(always)]
fn rsa_private_from_pem(pem: &str) -> napi::Result<RsaPrivateKey> {
    RsaPrivateKey::from_pkcs8_pem(pem)
        .map_err(|error| invalid_input(&format!("RSA private key is invalid: {error}")))
}

#[inline(always)]
fn rsa_encrypt(public: &RsaPublicKey, input: &[u8]) -> napi::Result<Vec<u8>> {
    let mut rng = OsRng;
    public
        .encrypt(&mut rng, Oaep::new::<Sha256>(), input)
        .map_err(|error| invalid_input(&format!("RSA encryption failed: {error}")))
}

#[inline(always)]
fn rsa_decrypt(private: &RsaPrivateKey, input: &[u8]) -> napi::Result<Vec<u8>> {
    private
        .decrypt(Oaep::new::<Sha256>(), input)
        .map_err(|error| invalid_input(&format!("RSA decryption failed: {error}")))
}

#[napi(namespace = "base64", js_name = "encode")]
pub fn base64_encode(input: String) -> String {
    general_purpose::STANDARD.encode(input.as_bytes())
}

#[napi(namespace = "base64", js_name = "decode")]
pub fn base64_decode(input: String) -> napi::Result<String> {
    let decoded = general_purpose::STANDARD
        .decode(input.as_bytes())
        .map_err(|error| invalid_input(&format!("Invalid base64 input: {error}")))?;
    String::from_utf8(decoded)
        .map_err(|error| invalid_input(&format!("Decoded bytes are not valid UTF-8: {error}")))
}

#[napi(namespace = "base16", js_name = "encode")]
pub fn base16_encode(input: String) -> String {
    hex::encode(input.as_bytes())
}

#[napi(namespace = "base16", js_name = "decode")]
pub fn base16_decode(input: String) -> napi::Result<String> {
    let decoded = hex::decode(input)
        .map_err(|error| invalid_input(&format!("Invalid hex input: {error}")))?;
    String::from_utf8(decoded)
        .map_err(|error| invalid_input(&format!("Decoded bytes are not valid UTF-8: {error}")))
}

#[napi(namespace = "base32", js_name = "encode")]
pub fn base32_encode(input: String) -> String {
    base32_encode_bytes(input.as_bytes())
}

#[napi(namespace = "base32", js_name = "decode")]
pub fn base32_decode(input: String) -> napi::Result<String> {
    let decoded = base32_decode_bytes(&input)?;
    String::from_utf8(decoded)
        .map_err(|error| invalid_input(&format!("Decoded bytes are not valid UTF-8: {error}")))
}

#[napi(namespace = "crypto")]
pub fn md5(input: String, encoding: Option<String>) -> napi::Result<String> {
    let digest = digest_md5(&input);
    let mode = normalize_hash_encoding(encoding)?;
    Ok(encode_bytes(&digest, &mode))
}

#[napi(namespace = "crypto")]
pub fn sha256(input: String, encoding: Option<String>) -> napi::Result<String> {
    let digest = digest_sha256(&input);
    let mode = normalize_hash_encoding(encoding)?;
    Ok(encode_bytes(&digest, &mode))
}

#[napi(namespace = "crypto")]
pub fn sha512(input: String, encoding: Option<String>) -> napi::Result<String> {
    let digest = digest_sha512(&input);
    let mode = normalize_hash_encoding(encoding)?;
    Ok(encode_bytes(&digest, &mode))
}

#[napi(namespace = "AES", js_name = "encode")]
pub fn aes_encode(
    input: String,
    key: String,
    encoding: Option<String>,
) -> napi::Result<Vec<String>> {
    let mode = normalize_binary_encoding(encoding)?;
    let key_bytes = derive_aes_key(&key);
    let iv_hex = generate_aes_iv_hex();
    let iv_bytes = normalize_aes_iv(&iv_hex)?;
    let encrypted = aes_encrypt(input.as_bytes(), &key_bytes, &iv_bytes)?;
    let encoded = encode_bytes(&encrypted, &mode);
    Ok(vec![encoded, iv_hex])
}

#[napi(namespace = "AES", js_name = "decode")]
pub fn aes_decode(
    encoded: String,
    iv_hex: String,
    key: String,
    encoding: Option<String>,
) -> napi::Result<String> {
    let iv = normalize_aes_iv(&iv_hex)?;
    let key_bytes = derive_aes_key(&key);
    let encrypted = decode_bytes_with_optional_encoding(&encoded, encoding)?;
    let decrypted = aes_decrypt(&encrypted, &key_bytes, &iv)?;
    String::from_utf8(decrypted)
        .map_err(|error| invalid_input(&format!("Decrypted bytes are not valid UTF-8: {error}")))
}

#[napi(namespace = "RSA", js_name = "encode")]
pub fn rsa_encode(
    input: String,
    encoding: Option<String>,
    bits: Option<u32>,
) -> napi::Result<RsaPacket> {
    let mode = normalize_rsa_encoding(encoding)?;
    let bits = normalize_rsa_bits(bits)?;
    let (private, public) = rsa_generate_keys(bits)?;
    let encrypted = rsa_encrypt(&public, input.as_bytes())?;
    let private_key = rsa_private_to_pem(&private)?;
    let public_key = rsa_public_to_pem(&public)?;

    Ok(RsaPacket {
        encoded: encode_bytes(&encrypted, &mode),
        private_key,
        public_key,
        encoding: binary_encoding_name(&mode).to_string(),
        bits,
    })
}

#[napi(namespace = "RSA", js_name = "decode")]
pub fn rsa_decode(packet: RsaPacket) -> napi::Result<String> {
    if packet.private_key.trim().is_empty() {
        return Err(invalid_input("RSA packet.privateKey must not be empty"));
    }

    let encoding = if packet.encoding.trim().is_empty() {
        None
    } else {
        Some(packet.encoding.clone())
    };
    let encrypted = decode_bytes_with_optional_encoding(&packet.encoded, encoding)?;
    let private = rsa_private_from_pem(&packet.private_key)?;
    let decrypted = rsa_decrypt(&private, &encrypted)?;
    String::from_utf8(decrypted)
        .map_err(|error| invalid_input(&format!("RSA plaintext is not valid UTF-8: {error}")))
}

#[napi(namespace = "morse", js_name = "encode")]
pub fn morse_encode(input: String) -> String {
    if input.is_empty() {
        return input;
    }

    let mut output = Vec::new();
    for ch in input.chars() {
        if ch.is_whitespace() {
            output.push("/".to_string());
            continue;
        }
        if let Some(code) = morse_code_for(ch) {
            output.push(code.to_string());
        }
    }
    output.join(" ")
}

#[napi(namespace = "morse", js_name = "decode")]
pub fn morse_decode(input: String) -> String {
    if input.trim().is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for token in input.split_whitespace() {
        if token == "/" && should_insert_space(&output) {
            output.push(' ');
            continue;
        }
        if token == "/" {
            continue;
        }
        if let Some(ch) = morse_char_for(token) {
            output.push(ch);
        }
    }
    output
}

#[napi(namespace = "crypto")]
pub fn rot(input: String, shift: Option<u8>) -> String {
    if input.is_empty() {
        return input;
    }
    let normalized_shift = normalize_rot_shift(shift);
    input
        .chars()
        .map(|ch| rotate_ascii_char(ch, normalized_shift))
        .collect()
}
