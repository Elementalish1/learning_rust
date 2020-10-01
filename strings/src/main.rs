fn main() {
    let s = String::from("hello");
    println!("{}", s);

    let mut s = String::new();
    println!("{}", s.len());

    s.push_str("hi there, I am mathis! This is a long text with plenty of whitespace!");
    println!("{}", s);
    println!("{}", s.len());

    let mut split = s.split_whitespace();
    loop {
        let next = split.next();
        if next == None {
            break;
        }
        println!("{:?}", split.next());
    }
}
