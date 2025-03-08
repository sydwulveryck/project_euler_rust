fn main() {
    let mut i : i32 = 2521;
    let mut found : bool = false;

    while !found {
        let mut count : i32 = 1;
        for q in 1..20 {
            if !(i % q == 0) {
                break;
            } else {
                count = count + 1;
            }
        }
        if count != 20 {
            i = i + 1;
        } else {
            found = true;
        }   
    }
    
    println!("{}", i);
}
