use conrod;
use std::time::Instant;

pub mod message {
    pub use custom_widget::Message;
}
#[cfg(feature="keypad")]
pub use conrod_keypad::english;
#[cfg(feature="keypad")]
pub use conrod_keypad::sprite;
#[derive(Clone,Debug)]
pub enum ConrodMessage {
    Event(Instant, conrod::event::Input),
    Thread(Instant),
}
