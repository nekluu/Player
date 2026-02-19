use iced::Event;

#[derive(Clone, Debug)]
pub enum Message {
    TogglePause,
    ToggleLoop,
    Event(Event),
}
