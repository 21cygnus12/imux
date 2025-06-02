use imux::app::App;

fn main() -> iced::Result {
    iced::run("imux", App::update, App::view)
}
