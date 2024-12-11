fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn iter_to_array<Element, const N: usize>(mut iter: impl Iterator<Item = Element>) -> [Element; N] {
    // Here I use `()` to make array zero-sized -> no real use in runtime.
    // `map` creates new array, which we fill by values of iterator.
    let res: [_; N] = std::array::from_fn(|_| iter.next().unwrap());
    // Ensure that iterator finished
    // assert!(matches!(iter.next(), None));
    assert!(iter.next().is_none());
    res
}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5, 7];
    let my_array: [&i32; 6] = iter_to_array(my_vec.iter());
    println!("my array : {:?}", my_array);
    print_type_of(&my_array);
    println!("\narray를 다시 벡터로");
    // 벡터로 바뀌어서 push가 가능함
    let mut my_vec = my_array.to_vec();
    my_vec.push(&5);
    print_type_of(&my_vec);
    println!("my_vec : {:?}", my_vec);
}
