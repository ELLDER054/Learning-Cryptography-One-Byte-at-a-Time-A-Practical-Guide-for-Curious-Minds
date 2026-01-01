// Implementation of the Sieve of Eratosthenes
// Finds all of the primes between 0 and n
// For educational purposes only
// Extremely slow for large values of n
fn sieve(n: usize) -> Vec<usize> {
    // 'is_prime' is a list of all of the values and whether or not they are prime
    // Note that they all start out as 'true' because the sieve "kills" values that aren't prime
    // We'll be left with only the values that ARE prime
    let mut is_prime = vec![true; n];

    // Both 0 and 1 are not considered prime
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while (i * i) < n {
        if is_prime[i] {
            // Kills all of the multiples of i
            let mut j = i * i;
            while j < n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p { Some(i) } else { None })
        .collect()
}

fn main() {
    println!("{:?}", sieve(30)); // There are 10 primes between 0 and 30
}
