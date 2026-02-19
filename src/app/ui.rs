use iced::widget::{button, column, text};
use iced::{Alignment::Center, Element, Length::Fill};
use iced_video_player::VideoPlayer;
use crate::app::{App, message::Message};

pub fn view(app: &App) -> Element<'_, Message> {
    let video = VideoPlayer::new(&app.video);
    column![video].into()
}