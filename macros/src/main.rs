macro_rules! custom_add {
    ($x: expr, $y: ident) => {
        let res = $x.$y(1);
        println!("{} + 1 by {} returns {:#?}", $x, stringify!($y), res);
    };
}

macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

macro_rules! func {
    ($a: expr, $b: expr) => {
        func($a, $b, None)
    };
    ($a: expr, $b: expr, $k: expr) => {
        func($a, $b, Some($k))
    };
}

fn func(a: i32, b: i32, k: Option<i32>) -> i32 {
    let k = k.unwrap_or(1);
    a + b * k
}

fn main() {
    custom_add!(i32::MAX, checked_add);
    custom_add!(i32::MAX, wrapping_add);
    custom_add!(i32::MAX, saturating_add);

    println!(
        "{}{}Hello, color!{}",
        csi!("38;5;", 2, "m"),
        csi!("48;5;", 0, "m"),
        csi!("0m")
    );

    println!("{} {}", func!(1, 2, 3), func!(1, 2));
}
