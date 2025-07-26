use dioxus::document;
use dioxus::prelude::spawn;
use serde::Deserialize;
use ui::SHOW_MODIFIERS;

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum KeyEvent {
    #[serde(rename = "keyup")]
    Up,
    #[serde(rename = "keydown")]
    Down,
}

#[derive(Debug, Deserialize)]
struct KeyboardEvent {
    #[serde(rename = "altKey")]
    is_alt_pressed: bool,
    // #[serde(rename = "ctrlKey")]
    // is_ctrl_pressed: bool,
    // #[serde(rename = "shiftKey")]
    // is_shift_pressed: bool,
    code: String,
    key: String,
    #[serde(rename = "type")]
    event_type: KeyEvent,
}

impl KeyboardEvent {
    fn is_pressed(&self) -> bool {
        self.event_type == KeyEvent::Down
    }
}

pub fn create_keyboard_event_loop() {
    let js = r#"
        if (!window.keyboardEventsRegistered) {
            const eventToString = (e) => JSON.stringify({
                altKey: e.altKey,
                ctrlKey: e.ctrlKey,
                shiftKey: e.shiftKey,
                code: e.code,
                key: e.key,
                type: e.type,
            });
            
            const sendEvent = (e) => dioxus.send(eventToString(e));
        
            document.addEventListener('keydown', sendEvent);
            document.addEventListener('keyup', sendEvent);
            window.keyboardEventsRegistered = true;
        }
    "#;
    let mut eval = document::eval(js);

    spawn(async move {
        // ToDo: For now we're ignoring errors here
        while let Ok(event_string) = eval.recv::<String>().await {
            if let Ok(event) = serde_json::from_str::<KeyboardEvent>(&event_string) {
                handle_keyboard_event(event);
            }
        }
    });
}

fn handle_keyboard_event(event: KeyboardEvent) {
    // Is Alt being held down
    if event.key == "Alt" {
        *SHOW_MODIFIERS.write() = event.is_pressed();
    }

    // Key pressed with Alt:
    if event.is_alt_pressed && event.is_pressed() {
        match event.code.as_str() {
            "KeyC" => println!("Change View: Company"),
            "KeyR" => println!("Change View: Role"),
            "KeyI" => println!("Change View: Interview"),
            "KeyQ" => println!("Change View: Questions"),
            _ => {}
        }
    }
}
