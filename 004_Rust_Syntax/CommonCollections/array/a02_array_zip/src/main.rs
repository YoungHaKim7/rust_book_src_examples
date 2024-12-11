use std::iter::zip;

fn main() {
    let xs = [1, 2, 3];
    let ys = [4, 5, 6];

    let mut iter_val = zip(xs, ys);
    for i in iter_val.by_ref() {
        println!("{:?}", i);
    }
}
