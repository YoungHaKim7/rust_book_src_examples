fn main() {
    let my_vector = vec!["test", ""];
    let int_vector = vec![1, 2, 3, 4];
    let my_tuple = ("test내맘대로", 1, true, my_vector, int_vector);
    // println!("my tuple : {:?}", my_tuple);
    dbg!(&my_tuple);
    println!("my tuple : {:?}", my_tuple);
}
