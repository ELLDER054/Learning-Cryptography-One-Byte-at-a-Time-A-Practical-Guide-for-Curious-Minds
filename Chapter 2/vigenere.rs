// Shifts each character in the string by the numerical value of each character in the 'key' string
// Note: The 'encrypting' variable should be either 1 or -1
fn vigenere_shift(plaintext: &str, key: &str, encrypting: i8) -> String {
    let mut ciphertext = String::new();
    let shift: Vec<i8> = key.chars().map(|k| k as i8 - b'a' as i8).collect();

    for (i, c) in plaintext.chars().enumerate() {
        ciphertext.push(match c {
            'a'..='z' => {
                let base = b'a' as i8;

                // a.rem_euclid(b) is like the modulus operator (a mod b) but it gives the least NONNEGATIVE remainder
                // It may not be necessary here, but I think it makes more sense to use it in general
                let offset = (c as i8 - base + (encrypting * shift[i.rem_euclid(key.len())])).rem_euclid(26);

                // Returns the base, plus the shift that we created above as a 'char' type
                ((base + offset) as u8) as char
            }
            'A'..='Z' => {
                let base = b'A' as i8;

                // a.rem_euclid(b) is like the modulus operator (a mod b) but it gives the least NONNEGATIVE remainder
                // It may not be necessary here, but I think it makes more sense to use it in general
                let offset = (c as i8 - base + (encrypting * shift[i.rem_euclid(key.len())])).rem_euclid(26);

                // Returns the base, plus the shift that we created above as a 'char' type
                ((base + offset) as u8) as char
            }
            _ => c // Any other character just becomes itself - this is generally a bad idea because spaces are retained, making it easier to attack
        });
    }
    ciphertext
}

fn main() {
    // Example plaintext
    let plaintext = "The lazy dog jumped over the river";
    println!("{}", plaintext);

    // Shifts the letters
    let ciphertext = vigenere_shift(&plaintext, "vigenere", 1);
    println!("{}", ciphertext);

    // Shifts the letters back
    let decryption = vigenere_shift(&ciphertext, "vigenere", -1);
    println!("{}", decryption);
