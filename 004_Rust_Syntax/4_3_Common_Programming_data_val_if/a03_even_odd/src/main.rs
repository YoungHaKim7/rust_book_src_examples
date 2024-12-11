fn main() {
    let n = 5;
    let check_even_odd = if n % 2 == 0 {
        "짝수even"
    } else {
        "홀수odd"
    };

    println!("홀수인지 짝수인지 확인해 봅시다. : {}", check_even_odd);
}
