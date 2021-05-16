fn main() {
    let x = five();

    let y = "yahoo";
    printit(y.to_string());

    println!("{}", plus_one(x));

    println!("{}", is_odd({
        let mut c = 5;
        c += 1;
        c
    }));
}

fn printit(s: String) {
    println!("{}", s);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn is_odd(x: i32) -> bool {
    if(x%1) == 0 {
        false
    } else {
        true
    }
}
