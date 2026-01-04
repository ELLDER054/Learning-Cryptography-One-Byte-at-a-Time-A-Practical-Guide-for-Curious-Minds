/*
A Caesar's Cipher shifts each numerical value of the plaintext by a key number
Example:

key = 3
h  e  l  l  o  w  o  r  l  d
7  4  11 11 14 22 14 17 11 3
+3 ...
10 7  14 14 17 25 17 20 14 6
k  h  o  o  r  z  r  u  o  g

so

"helloworld" -> "khoorzruog"
with a key of 3

In order to decrypt the ciphertext, you repeat the above operation, but subtracting 3 instead of adding

k  h  o  o  r  z  r  u  o  g
10 7  14 14 17 25 17 20 14 6
-3 ...
7  4  11 11 14 22 14 17 11 3
h  e  l  l  o  w  o  r  l  d

to get "helloworld" again
*/

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
    let plaintext = "hello world";
    println!("{}", plaintext);

    // Shifts the letters right by 3
    let ciphertext = caesar_shift(&plaintext, 3);
    println!("{}", ciphertext);

    // Shifts the letter LEFT by 3 (which is why the 3 is negative)
    let decryption = caesar_shift(&ciphertext, -3);
    println!("{}", decryption);
}
