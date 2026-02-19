use iced::futures::{Stream, stream};
use iced::Subscription;
use crate::app::Message;


pub fn subscription() -> Subscription<Message> {
    iced::event::listen().map(Message::Event)
}