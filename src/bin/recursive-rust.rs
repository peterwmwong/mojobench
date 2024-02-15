fn recursive(x: isize) {
    // Mojo's Blog uses Vec::with_capacity(), which feels a bit odd/cherry-picked-to-make-rust-look-bad.
    // Using `vec!` seems more common/idiomatic:
    // - https://doc.rust-lang.org/rust-by-example/std/vec.html
    // - https://doc.rust-lang.org/book/ch08-01-vectors.html
    // - https://doc.rust-lang.org/std/vec/struct.Vec.html
    let _stuff = vec![0_isize; 42];
    if x == 0 {
        return;
    }
    recursive(x - 1);
}

fn main() {
    recursive(999_999_999);
}
