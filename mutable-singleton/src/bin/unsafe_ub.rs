use mutable_singleton::Singleton;
use std::time::Instant;

static SINGLETON: Singleton = Singleton { state: 2 };

#[inline(never)]
fn get_singleton() -> u64 {
    SINGLETON.get_unsafe()
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
