use iced::widget::{button, column, text, Container};
use iced::{Alignment::Center, Element, Length::Fill};
use iced_video_player::VideoPlayer;
use crate::app::{App, message::Message};

pub fn view(app: &App) -> Element<'_, Message> {
    let video = Container::new(
        VideoPlayer::new(&app.video)
            .width(Fill)
            .height(Fill)
            .content_fit(iced::ContentFit::Contain)
            .on_new_frame(Message::NewFrame),
    )
        .align_x(Center)
        .align_y(Center)
        .width(Fill)
        .height(Fill);

    column![video].into()
}
