fn main() {
    // array 고정된거
    // array 스택에 할당
    let mut my_array = [0; 3];
    println!("{:?}", my_array);
    // index는 0부터 시작 함.
    my_array[0] = 10;
    println!("{:?}", my_array);
    // my_array.push(3);
    let mut my_array02 = [0; 4];
    my_array02[3] = 3;
    println!("{:?}", my_array02);

    // Vector벡터는 늘어날 가능성있는거
    // push랑 pop 기능이 가능
    // 벡터는 Heap에 할당이 된다.  안에구현된걸 보면 Box가
    // let mut my_vec = Vec::new();
    let mut my_vec = vec![];
    my_vec.push(0);
    println!("{:?}", my_vec);
    my_vec.push(1);
    println!("{:?}", my_vec);
    my_vec.pop();
    println!("{:?}", my_vec);
}
