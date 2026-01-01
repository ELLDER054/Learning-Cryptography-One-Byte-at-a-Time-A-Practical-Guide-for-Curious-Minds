// This is an implementation of the RSA encryption algorithm using example values from 'In Code' by Sarah Flannery

// The two prime factors of the public modulus
// These are to be KEPT HIDDEN or THROWN AWAY once phi_n is calcuated
const P: u128 = 281;
const Q: u128 = 167;

// Public exponent and modulus
// Everybody has access to these
// They make up the public key in 'Public Key Cryptography'
const PUB_EXP: u128 = 39423;
const PUB_MOD: u128 = P * Q;

// Converts a group of characters into base 10 from base 26
fn to_base10(chars: &[char]) -> u128 {
    let mut n = 0;

    // Loops through the characters BACKWARDS and multiplies each by an increasing power of 26
    // For example, for the text "yes"
    // y is the 25th letter in the alphabet
    // e is the 5th letter in the alphabet
    // s is the 19th letter in the alphabet
    // But these all need to be -1 because the alphabet is represented as 0-25 instead of 1-26
    // Therefore...
    // 18 * (26 ** 0) + 4 * (26 ** 1) + 24 * (26 ** 2)
    for (i, c) in chars.iter().rev().enumerate() {
        n += (*c as u8 - b'a') as u32 * (u32::pow(26, i as u32));
    }
    n.into()
}

// Undoes the above function using the modulus operator and division
fn to_base26(n: u128, chunks: usize) -> String {
    let mut m = String::new();
    let mut i = 0;
    while i < chunks {
        let power1 = modpow(26, i as u128 + 1, u128::MAX);
        let divisor = modpow(26, i as u128, u128::MAX);
        let c: u8 = (modpow(n, 1, power1) / divisor).try_into().unwrap();

        // Converts the number back into a character
        m.push((b'a' + c) as char);
        i += 1;
    }
    m.chars().rev().collect()
}

// For some reason, I couldn't figure out how to combine this function and the one directly below this one into a single function
// This one splits a string into chunks of 3 characters and adds 'x's as padding at the end
fn split3(s: &str) -> Vec<[char; 3]> {
    let mut chars: Vec<char> = s.chars().collect();

    // Pad so length is a multiple of 3
    let remainder = chars.len() % 3;
    if remainder != 0 {
        // Extend and pad
        chars.extend(std::iter::repeat('x').take(3 - remainder));
    }

    chars
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect()
}

// This one splits a string into chunks of 4 characters and adds 'x's as padding at the end
fn split4(s: &str) -> Vec<[char; 4]> {
    let mut chars: Vec<char> = s.chars().collect();

    // Pad so length is a multiple of 4
    let remainder = chars.len() % 4;
    if remainder != 0 {
        chars.extend(std::iter::repeat('x').take(4 - remainder));
    }

    chars
        .chunks(4)
        .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
        .collect()
}

// This function is essential to making sure powers of large numbers don't go too high and give an overflow
// It mods the multiplication at every step instead of just at the end
fn modpow(a: u128, b: u128, c: u128) -> u128 {
    let mut r: u128 = 1;
    for _i in 0..b {
        // Using rem_euclid here just because I generally think it makes more sense as a modulus operator with no negative remainders
        r = (r * a).rem_euclid(c);
    }
    r
}

// The Extended Euclidean Algorithm
// Finds the modular multiplicative inverse of e, (mod n)
// Solves e * _ ≡ 1 (mod n)
fn multiplicative_inverse(e: u128, n: u128) -> u128 {
    // 'r' is the remainder
    let mut prev_r = n as i128;
    let mut r = e as i128;

    // 'prev_s' is the previous 's'
    // 's' is the coefficient for n
    let mut prev_s = 1;
    let mut s = 0;

    // 'prev_t' is the previous 't'
    // 't' is the coefficient for e
    let mut prev_t = 0;
    let mut t = 1;

    // Initialize the quotient variable
    let mut q;

    // Initialize the temporary variables
    let mut temp_r;
    let mut temp_s;
    let mut temp_t;

    // Repeat until the remainder == 0
    while r != 0 {
        // Find the quotient 'q' of 'prev_r' and 'r' - note this is floor division, so no decimal points
        q = prev_r / r;

        // Take temporary values of the previous 'r', 's', and 't'
        temp_r = r;
        temp_s = s;
        temp_t = t;

        // Update the current 'r', 's', and 't'
        r = prev_r - (q * r);
        s = prev_s - (q * s);
        t = prev_t - (q * t);

        // Correctly reset the 'prev_' variables to the 'temp_' values
        prev_r = temp_r;
        prev_s = temp_s;
        prev_t = temp_t;
    }

    if prev_r == 1 {
        prev_t.rem_euclid(n as i128).try_into().unwrap() // Take the modulo to ensure that it is positive
    } else {
        println!("Error in Extended Euclidean Algorithm - no modular multiplicative inverse exists for {}, (mod {})", e, n);
        0 // No inverse exists
    }
}

// Euler's totient function (aka phi function) gives the total number of numbers below a number that are co-prime to that number
// However, if that number is a product of two primes, it can be more simply calculated as (p - 1) * (q - 1) where p and q are the two prime factors
// The reason for this is that phi(p * q) = phi(p) * phi(q)
// phi(p) is all of the numbers below p, because every number below a prime is co-prime to that prime
// phi(q) is all of the numbers below q
// Thus, phi(n) = (p - 1) * (q - 1)
fn phi_n(p: u128, q: u128) -> u128 {
    (p - 1) * (q - 1)
}

// Getting the private exponent (decryption key) requires finding a d where e * d ≡ 1 (mod phi(n))
// This is called finding the 'multiplicative inverse of e, (mod n)' and it requires the Extended Euclidean Algorithm
fn get_private_exp() -> u128 {
    multiplicative_inverse(PUB_EXP, phi_n(P, Q))
}

// Encrypts a given string with the public encryption key and public modulus
fn encrypt(plaintext: &str) -> String {
    let mut ciphertext = String::new();
    let mut base10;
    let mut cipher_trigraph;

    for plain_trigraph in split3(plaintext) {
        base10 = to_base10(&plain_trigraph);
        cipher_trigraph = to_base26(modpow(base10, PUB_EXP, PUB_MOD), 4);
        ciphertext.push_str(&cipher_trigraph);
    }
    ciphertext
}

// Decrypts a given string with the private decryption key and public modulus
fn decrypt(ciphertext: &str, d: u128) -> String {
    let mut plaintext = String::new();
    let mut base10;
    let mut plain_trigraph;

    for cipher_trigraph in split4(ciphertext) {
        base10 = to_base10(&cipher_trigraph);
        plain_trigraph = to_base26(modpow(base10, d, PUB_MOD), 3);
        plaintext.push_str(&plain_trigraph);
    }
    plaintext
}

fn main() {
    let plaintext = "helloworld";
    println!("{}", plaintext);

    let ciphertext = encrypt(&plaintext);
    println!("{}", ciphertext);

    let private_exp = get_private_exp();
    let decryption = decrypt(&ciphertext, private_exp);
    println!("{}", decryption); // 'helloworldxx' because of padding
}
