pub mod message;
pub mod ui;
pub mod utils;

use iced::{Subscription, Task, Theme};
use iced_video_player::Video;
pub use message::Message;
use crate::app::utils::subscription;

#[derive(Debug)]
pub struct App {
    pub video: Video,
    pub video_is_playing: bool,
    pub video_is_looping: bool,
}
impl Default for App {
    fn default() -> Self {
        App {
            video: Video::new(
                &url::Url::from_file_path(
                    std::env::current_dir()
                        .unwrap()
                        .join("media/brh.mp4")
                        .canonicalize()
                        .unwrap()
                ).unwrap(),
            ).unwrap(),
            video_is_playing: true,
            video_is_looping: false,
        }
    }
}
impl App {
    pub fn theme(&self) -> Theme {
        Theme::CatppuccinMacchiato
    }
    pub fn subscription(&self) -> Subscription<Message> {
        subscription()
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TogglePause => {
                self.video_is_playing = !self.video_is_playing;
                self.video.set_paused(!self.video_is_playing);
                Task::none()
            }
            Message::ToggleLoop => {
                self.video.set_looping(self.video_is_looping);
                Task::none()
            }

            Message::Event(event) => {
                if let iced::Event::Keyboard(kbd_event) = event {
                    match kbd_event {
                        iced::keyboard::Event::KeyPressed { key, .. } => {
                            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) = key {
                                return Task::done(Message::TogglePause);
                            }
                            Task::none()
                        }
                        iced::keyboard::Event::KeyReleased { key, .. } => {
                            println!("released: {:?}", key);
                            Task::none()
                        }
                        iced::keyboard::Event::ModifiersChanged(_) => {
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::NewFrame => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        ui::view(self)
    }
}