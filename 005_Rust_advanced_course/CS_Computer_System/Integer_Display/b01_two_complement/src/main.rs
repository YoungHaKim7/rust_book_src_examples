fn main() {
    let raw_two_complement = 0xB; // 1011
    let two_val = !raw_two_complement + 1; // 0100 + 0001
    println!("컴퓨터는 0과 1뿐이라서..");
    println!("마이너스로 빼기를 구현하기 위해 2의 보수 개념이 필요하다.");
    println!("0xB(10진수=11)의 2의 보수 : {}", two_val); // 0101    11(10진법) -> -11 로 변함
}
