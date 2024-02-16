fn recursive(x: isize) {
    let _stuff = vec![0_isize; 42];
    if x == 0 {
        return;
    }
    recursive(x - 1);
}

fn main() {
    recursive(999_999_999);
}
