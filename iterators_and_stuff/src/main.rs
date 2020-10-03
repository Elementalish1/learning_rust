fn main() {
    let mut names = vec!["bob", "frank", "ferris", "mathis"];

    for name in names.iter() {
        match name {
            &"mathis" => println!("thats a me!"),
            _ => println!("Hello {}", name)
        }
    }

    for name in names.iter_mut() {
        *name = match name {
            &mut "mathis" => "ayoooo",
            _ => "hello",
        }
    }

    println!("{:?}", names)
}
