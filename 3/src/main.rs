fn main() {
    let mut n : i64 = 600851475143;
    let mut d : i64 = 2;
    let mut q : i64 = 1;

    let small_primes : [i64; 8] = [2,3,5,7,11,13,17,21];

    while !small_primes.contains(&n){
        while n % d != 0 {
            d = d + 1;
        }
        println!("{} % {}, q={}", n, d, q);
        
        n = n / d;
        q = q * d;

        d = 2;
    }
}

