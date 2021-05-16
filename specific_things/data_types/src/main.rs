fn main() {
    // 8-bit i8 u8
    // 16-bit i16 u16
    // 32-bit i32 u32
    // 64-bit i64 u64
    // 128-bit i128 u128
    // arch isize usize

    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;

    let diffirence = 96.5 - 2.4;

    let product = 4 * 5;

    let remainder = 43 % 5;

    println!("{}, {}, {}, {}, {}, {}", x, y, sum, diffirence, product, remainder);

    let t = true;
    let f: bool = false;

    println!("{}, {}", t, f);

    let c = "hehe";
    println!("{}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let a = [1,2,3,4,5];

    let months = ["jan", "feb", "mar", "apr"];

    println!("{:?}, {:?}", a, months);
}
