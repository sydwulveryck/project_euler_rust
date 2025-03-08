fn main() {
    let mut sumsq : i32 = 0;
    let mut sqsum : i32 = 0;
    
    for i in 1..101 {
        sumsq = sumsq + i32::pow(i, 2);
        sqsum = sqsum + i;
    }
    sqsum = i32::pow(sqsum, 2);

    println!("{}", sqsum - sumsq)
}
