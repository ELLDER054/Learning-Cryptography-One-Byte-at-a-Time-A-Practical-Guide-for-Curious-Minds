use std::fs::File;
use std::io::{Read, Write};
use rand::RngCore;
use rand::rngs::OsRng;

fn generate_key_file(filename: &str, len: usize) -> std::io::Result<()> {
    let mut key = vec![0u8; len];
    OsRng.fill_bytes(&mut key);
    let mut file = File::create(filename)?;
    file.write_all(&key)?;
    Ok(())
}

fn read(file_name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_name)?;
    let mut text = Vec::new();
    file.read_to_end(&mut text)?;
    Ok(text)
}

fn xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext.iter()
        .zip(key.iter())
        .map(|(&m, &k)| m ^ k)
        .collect()
}

fn main() {
    let plaintext = b"My name is bob!";

    generate_key_file("key.bin", plaintext.len()).unwrap();
    let key = read("key.bin").unwrap_or_default();

    let encrypted = xor(plaintext, &key);
    let decrypted = xor(&encrypted, &key);

    println!("{:?}", String::from_utf8_lossy(&encrypted));
    println!("{}", String::from_utf8(decrypted).unwrap());
}
