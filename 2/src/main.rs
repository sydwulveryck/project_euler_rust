// Find the sum of even-valued fibonacci number under 4.000.000

fn main() {
    let mut f0 = 1;
    let mut f1 = 2;
    let mut fx = f0 + f1;

    let mut total = 2;

    while fx < 4000000 {
        f0 = f1;
        f1 = fx;
        fx = f0 + f1;
        // println!("f0: {}, f1: {}, fx: {}", f0, f1, fx);

        if fx % 2 == 0 {
            total = total + fx;
        }
    }

    println!("{}", total);
}
