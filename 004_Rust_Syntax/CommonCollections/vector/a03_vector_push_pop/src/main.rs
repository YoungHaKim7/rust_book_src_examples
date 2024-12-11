fn main() {
    let mut my_vec = vec![1, 2, 3, 4];
    let mut my_vec02 = vec![100, 200, 300, 400];

    my_vec.push(10);
    my_vec.append(&mut my_vec02);

    println!("my_ vec (total): {:?}", my_vec);
}
