use super::super::app::App;
use termion::event::Key;
// use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('t') => {
            app.fm_trash();
        }
        _ => {}
    }
}
