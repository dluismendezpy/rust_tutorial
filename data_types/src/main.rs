use std::io;

fn main() {
    // Primitive Types
    // Signed types
    let guess_x: i8 = 127;
    let guess_y: i16 = 32767;
    let guess_z: i32 = "2147483647".parse().expect("Not a number!");
    let guess_w: i64 = 9223372036854775807;
    let guess_u: i128 = 170141183460469231731687303715884105727;
    let guess_v: isize = 2147483647; // 32-bit architecture
    let guess_h: isize = 9223372036854775807; // 64-bit architecture

    // Unsigned types
    let guess_a: u8 = 255;
    let guess_b: u16 = 65535;
    let guess_c: u32 = "4294967295".parse().expect("Not a number!");
    let guess_d: u64 = 18446744073709551615;
    let guess_e: u128 = 340282366920938463463374607431768211455;
    let guess_f: usize = 4294967295; // 32-bit architecture
    let guess_j: usize = 9223372036854775807; // 64-bit architecture

    println!("Signed types:");
    println!("{guess_x}, {guess_y}, {guess_z}, {guess_w}, {guess_u}, {guess_v}, {guess_h}");
    println!("\nUnsigned types:");
    println!("{guess_a}, {guess_b}, {guess_c}, {guess_d}, {guess_e}, {guess_f}, {guess_j}");

    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x} - {y}");

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t} - {f}");

    // The Character Type
    let c = 'c';
    let z: char = 'ℤ'; // with explicit type annotation
    let cap = '🧢';
    println!("{c} - {z} - {cap}");

    // ---------------------------------------------------------------- //
    // Compound Types
    // Turple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred: i32 = tup.0;
    let six_point_four: f64 = tup.1;
    let one: u8 = tup.2;
    println!("{five_hundred} - {six_point_four} - {one}");

    //Array
    let days: [&str; 7] = [
        "Monaday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[2]);

    println!("\n\nPlease enter an array index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: &str = days[index];
    println!("The value of the element at index {index} is: {element}");
}
