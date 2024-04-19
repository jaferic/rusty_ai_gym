use enigo::*;

pub fn input_capture(action: &str) {
    let mut enigo = Enigo::new();

    match action {
        "w" => enigo.key_click(Key::Layout('w')),
        "s" => enigo.key_click(Key::Layout('s')),
        "a" => enigo.key_click(Key::Layout('a')),
        "d" => enigo.key_click(Key::Layout('d')),
        _ => println!("unknown action"),
    }
}
