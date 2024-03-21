use std::{thread::sleep, time::Duration};

use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};


fn main() {
    let mut enigo = Enigo::default();

    let (x, y) = enigo.main_display_size();
    loop {
        sleep(Duration::from_millis(500));
        enigo.mouse_move_to(x/3, y/3);

        sleep(Duration::from_millis(500));
        enigo.mouse_move_to(2*x/3, y/3);

        sleep(Duration::from_millis(500));
        enigo.mouse_move_to(2*x/3, 2*y/3);

        sleep(Duration::from_millis(500));
        enigo.mouse_move_to(x/3, 2*y/3);



        enigo.key_down (Key::Meta);
        enigo.key_click(Key::R);
        enigo.key_up(Key::Meta);

        enigo.key_sequence("notepad"); 
        enigo.key_click(Key::Return);
        sleep(Duration::from_secs(1));

        enigo.mouse_move_to(x/2, y/2);
        enigo.mouse_click(enigo::MouseButton::Left);

        enigo.key_down (Key::Control);
        enigo.key_click(Key::A);
        enigo.key_up   (Key::Control);
        enigo.key_click(Key::Backspace);
        enigo.key_sequence("en fait non... rien");

        enigo.key_down (Key::Alt);
        enigo.key_click(Key::F4);
        enigo.key_up(Key::Alt);
        enigo.key_click(Key::RightArrow);
        enigo.key_click(Key::Return);

        sleep(Duration::from_secs(30));
    }
}