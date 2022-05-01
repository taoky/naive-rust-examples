use mutable_singleton::SingletonWithUnsafeCell;
use std::{cell::UnsafeCell, time::Instant};

// THIS DOES NOT COMPILE
// static SINGLETON: SingletonWithUnsafeCell = SingletonWithUnsafeCell {
//     state: UnsafeCell::new(2),
// };

static mut SINGLETON: SingletonWithUnsafeCell = SingletonWithUnsafeCell {
    state: UnsafeCell::new(2),
};

#[inline(never)]
fn get_singleton() -> u64 {
    // SINGLETON.get()
    unsafe { SINGLETON.get() }
}

fn main() {
    let mut x = 0;
    const STEPS: i32 = 1000000000;
    let before = Instant::now();
    for _ in 0..STEPS {
        x = get_singleton();
    }
    println!("{}", x);
    println!("Time: {} ms", before.elapsed().as_millis());
}
