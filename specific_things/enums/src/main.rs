enum Errors {
    Gloop,
    Bloop,
    Krr(String),
    Click { x: i64, y: i64 },
}

fn show_error(err: Errors) {
    match err {
        Errors::Gloop => println!("Gloopety poopety"),
        Errors::Bloop => println!("Bloopety poopety"),
        Errors::Krr(s) => println!("{}", s),
        Errors::Click { x, y } => {
            println!("x: {} \ny: {}", x, y);
        }
    }
}

fn main() {
    let gloop = Errors::Gloop;
    let bloop = Errors::Bloop;

    let krr = Errors::Krr("skiddle".to_string());

    let click = Errors::Click { x: 10, y: 23 };

    show_error(gloop);
    show_error(bloop);
    show_error(krr);
    show_error(click);
}
