use mutable_singleton::Singleton;
use std::{cell::RefCell, time::Instant};

thread_local! {
    static SINGLETON: RefCell<Singleton> = RefCell::new(Singleton::new(2));
}

#[inline(never)]
fn get_singleton() -> u64 {
    SINGLETON.with(|s| s.borrow_mut().get())
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
