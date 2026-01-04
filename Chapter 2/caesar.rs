// Shifts each character in the string by the 'shift' variable
fn caesar_shift(plaintext: &str, shift: i8) -> String {
    let ciphertext: String = plaintext.chars().map(|c| match c {
        'a'..='z' => {
            let base = b'a' as i8;

            // a.rem_euclid(b) is like the modulus operator (a mod b) but it gives the least NONNEGATIVE remainder
            // It may not be necessary here, but I think it makes more sense to use it in general
            let offset = (c as i8 - base + shift).rem_euclid(26);

            // Returns the base, plus the shift that we created above as a 'char' type
            ((base + offset) as u8) as char
        }
        'A'..='Z' => { // Same as above, but for capital A-Z
            let base = b'A' as i8;
            let offset = (c as i8 - base + shift).rem_euclid(26);
            ((base + offset) as u8) as char
        }
        _ => c // Any other character just becomes itself - this is generally a bad idea because spaces are retained, making it easier to attack
    }).collect();
    ciphertext
}

fn main() {
    // Example plaintext
    let plaintext = "The lazy dog jumped over the river";
    println!("{}", plaintext);

    // Shifts the letters right by 2
    let ciphertext = caesar_shift(&plaintext, 2);
    println!("{}", ciphertext);

    // Shifts the letter LEFT by 2 (which is why the 2 is negative)
    let decryption = caesar_shift(&ciphertext, -2);
    println!("{}", decryption);
}
