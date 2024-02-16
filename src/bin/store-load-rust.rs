#![no_main]
fn store_load() -> isize {
    let mut stuff = vec![0_isize; 42];
    stuff[1] = 2;
    stuff[1]
}

#[no_mangle]
pub fn main() -> isize {
    for _ in 0..9_999_999 {
        core::hint::black_box(store_load());
    }
    0
}
