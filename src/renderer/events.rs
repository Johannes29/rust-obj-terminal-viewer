use crossterm::event::{self, Event, KeyModifiers, KeyCode};
use std::time::Duration;

pub fn get_events_from_queue() -> Vec<Event> {
    let mut events = Vec::new();
    loop {
        if event::poll(Duration::from_secs(0)).unwrap() {
            events.push(event::read().unwrap());
        }
        else {
            break
        }
    }

    return events
}

pub fn should_exit(event: &Event) -> bool {
    match event {
        Event::Key(key_event) => {
            if key_event.modifiers == KeyModifiers::NONE {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        return true
                    },
                    _ => (),
                }
            }
        },
        _ => (),
    }

    return false
}