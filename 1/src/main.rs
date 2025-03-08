fn main() {
    // Calculer la somme des multiples de 3 ou de 5 entre 0 et 1000

    let mut total : i32 = 0;
    
    for n in 0..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            total += n;
        }
    }
    println!("{}", total);
}
