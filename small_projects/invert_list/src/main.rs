fn main() {
    let a = [1,2,3,4,5];
    let b = invert_array(a);

    println!("{:?}", b);

}

fn invert_array(a: [i32; 5]) -> [i32; 5] {
    let mut b: [i32; 5] = Default::default();

    let lena = a.len();
    for i in 0..lena {
        b[i] = a[lena-i-1];
    }

    return b;
}