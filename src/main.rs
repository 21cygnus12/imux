use imux::Imux;

pub fn main() -> iced::Result {
    iced::application("imux", Imux::update, Imux::view)
        .subscription(Imux::subscription)
        .run()
}
