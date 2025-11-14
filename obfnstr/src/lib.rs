const OBFUSCATION_KEY: [u8; 8] = [0x5C, 0x1A, 0x3F, 0x82, 0x9D, 0x6E, 0xB7, 0x4B];

#[inline(always)]
pub fn bytes_xor(bytes: &[u8]) -> Vec<u8> {
    let mut new_bytes = Vec::with_capacity(bytes.len());
    for (i, byte) in bytes.iter().enumerate() {
        let key_byte = OBFUSCATION_KEY[i % OBFUSCATION_KEY.len()];
        new_bytes.push(byte ^ key_byte);
    }
    new_bytes
}

#[inline(always)]
pub fn obfuscate(bytes: &[u8]) -> String {
    let deobfuscated_bytes = bytes_xor(bytes);
    unsafe { String::from_utf8_unchecked(deobfuscated_bytes) }
}