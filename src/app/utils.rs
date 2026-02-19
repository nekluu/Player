use iced::futures::{Stream, stream};
use iced::Subscription;
use crate::app::Message;

pub fn stream_logic() -> impl Stream<Item = usize> {
    let counter = 0;
    stream::unfold(counter, move |mut state| async move {
        if state >= 1000 {
            None
        } else {
            state += 1;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            Some((state, state))
        }
    })
}
pub fn subscription() -> Subscription<Message> {
    iced::event::listen().map(Message::Event)
}