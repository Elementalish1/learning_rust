fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {}", result);

    counter = 3;
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    };

    for x in 1..11 {
        if x == 5 {
            continue;
        }
        println!("x is {}", x);
    };

    loop {
        println!("again!");
        break; // hee hee
    }

}
