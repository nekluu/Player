use crate::app::App;

mod app;

fn main() -> iced::Result {
    iced::application(App::default,App::update,App::view).subscription(App::subscription).theme(App::theme).run()
}