fn main() {
    const LARGE_N : usize = 100000;
    let mut l : [bool; LARGE_N + 1] = [true; LARGE_N + 1];
    
    l[0] = false;
    l[1] = false;

    for i in 2..LARGE_N {
        if l[i] {
            for j in 2*i..LARGE_N.step_by(i) {
                l[j] = false;
            }
        }
    }
}
