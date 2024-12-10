enum KeyboardArrowKey {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

fn input_arrow(x: KeyboardArrowKey) -> String {
    match x {
        KeyboardArrowKey::ArrowUp => "Up".to_string(),
        KeyboardArrowKey::ArrowDown => "Down".to_string(),
        KeyboardArrowKey::ArrowLeft => "Left".to_string(),
        KeyboardArrowKey::ArrowRight => "Right".to_string(),
    }
}

fn main() {
    let rightkey = KeyboardArrowKey::ArrowRight;
    dbg!(input_arrow(rightkey));
}
