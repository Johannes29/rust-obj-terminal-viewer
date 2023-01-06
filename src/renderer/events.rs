use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

pub fn get_events_from_queue() -> Vec<Event> {
    let mut events = Vec::new();
    loop {
        if event::poll(Duration::from_secs(0)).unwrap() {
            events.push(event::read().unwrap());
        } else {
            break;
        }
    }

    return events;
}

pub fn should_exit(event: &Event) -> bool {
    match event {
        Event::Key(key_event) => match key_event.modifiers {
            KeyModifiers::NONE => {
                if vec![KeyCode::Esc, KeyCode::Char('q')].contains(&key_event.code) {
                    return true;
                }
            }
            KeyModifiers::CONTROL => {
                if key_event.code == KeyCode::Char('c') {
                    return true;
                }
            }
            _ => (),
        },
        _ => (),
    }

    return false;
}
