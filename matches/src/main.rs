fn main() {
    for i in 1..22 {
        disp_num(i);
    }
}

fn disp_num(number: i32) {

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 4 | 5 | 6 | 7 => println!("2 to 7"),
        7..=19 => println!("8 to 19"),
        _ => println!("anything else!")
    }
}
