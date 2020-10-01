fn main() {
    let a = [1,2,3];
    let mut m = [1,2,3];
    println!("{:?} {:?}", a, m);

    let a = [10; 20];
    println!("{:?}", a);

    for e in a.iter() {
        println!("{}", e);
    }
}
