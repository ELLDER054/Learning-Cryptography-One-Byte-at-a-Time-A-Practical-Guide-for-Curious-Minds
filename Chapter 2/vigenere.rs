/*
A Vigenere's Cipher shifts each numerical value of the plaintext by a numerical value of the key
Example:

key = "mykey" = (12, 24, 10, 4, 24)
h   e   l   l   o   w   o   r   l   d
7   4   11  11  14  22  14  17  11  3
+12 +24 +10 +4  +24 +12 +24 +10 +4  +24
19  28  21  15  38  34  38  27  15  27

mod by 26

19  2   21  15  12  8   12  1   15  1
t   c   v   p   m   i   m   b   p   b

so

"helloworld" -> "tcvpmimbpb"
with a key of "mykey"

In order to decrypt the ciphertext, you repeat the above operation, but subtracting each key number instead of adding

t   c   v   p   m   i   m   b   p   b
19  2   21  15  12  8   12  1   15  1
-12 -24 -10 -4  -24 -12 -24 -10 -4  -24
7   -22 11  11  -12 -4  -12 -9  11  -23

mod by 26

7   4   11  11  14  22  14  17  11  3
h   e   l   l   o   w   o   r   l   d

to get "helloworld" again
*/

// Shifts each character in the string by the numerical value of each character in the 'key' string
// Note: The 'encrypting' variable should be either 1 or -1
fn vigenere_shift(plaintext: &str, key: &str, encrypting: i8) -> String {
    let mut ciphertext = String::new();
    let shift: Vec<i8> = key.chars().map(|k| k as i8 - b'a' as i8).collect();
    println!("{:?}", shift);

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
    let plaintext = "helloworld";
    println!("{}", plaintext);

    // Shifts the letters
    let ciphertext = vigenere_shift(&plaintext, "mykey", 1);
    println!("{}", ciphertext);

    // Shifts the letters back
    let decryption = vigenere_shift(&ciphertext, "mykey", -1);
    println!("{}", decryption);
}
