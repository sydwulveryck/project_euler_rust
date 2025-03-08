fn is_palindrom(s : String) -> bool {
    let len : usize = s.len();
    for i in 0..len/2 {
        if s[i] != s[len() - i] {
            return false;
        }     
    }
    true
}

fn main() {
    let max : i32 = 999 * 999;
    let mut a : i32 = 999;
    let mut b : i32 = 999;

    let n = a * b;

    let s = n.to_string();

    // check if s is palindrome
    
}
