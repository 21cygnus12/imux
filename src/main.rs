use iced::keyboard;
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{Container, button, column, container, responsive, scrollable, text};
use iced::{Center, Element, Fill, Size, Subscription};

pub fn main() -> iced::Result {
    iced::application("imux", Imux::update, Imux::view)
        .subscription(Imux::subscription)
        .run()
}

struct Imux {
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}

impl Imux {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new());

        Imux {
            panes,
            panes_created: 1,
            focus: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Split(axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::new());

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new());

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) = self.panes.adjacent(pane, direction) {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { is_pinned, .. }) = self.panes.get(pane) {
                        if !is_pinned {
                            if let Some((_, sibling)) = self.panes.close(pane) {
                                self.focus = Some(sibling);
                            }
                        }
                    }
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            if !modifiers.command() {
                return None;
            }

            handle_hotkey(key_code)
        })
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |size| {
                view_content(id, total_panes, pane.is_pinned, size)
            }))
            .style(if is_focused {
                style::pane_focused
            } else {
                style::pane_active
            })
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        container(pane_grid).padding(10).into()
    }
}

impl Default for Imux {
    fn default() -> Self {
        Imux::new()
    }
}

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::{Axis, Direction};

    match key.as_ref() {
        Key::Character("v") => Some(Message::SplitFocused(Axis::Vertical)),
        Key::Character("h") => Some(Message::SplitFocused(Axis::Horizontal)),
        Key::Character("w") => Some(Message::CloseFocused),
        Key::Named(key) => {
            let direction = match key {
                key::Named::ArrowUp => Some(Direction::Up),
                key::Named::ArrowDown => Some(Direction::Down),
                key::Named::ArrowLeft => Some(Direction::Left),
                key::Named::ArrowRight => Some(Direction::Right),
                _ => None,
            };

            direction.map(Message::FocusAdjacent)
        }
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct Pane {
    pub is_pinned: bool,
}

impl Pane {
    fn new() -> Self {
        Self { is_pinned: false }
    }
}

fn view_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = column![
        button(
            "Split horizontally",
            Message::Split(pane_grid::Axis::Horizontal, pane),
        ),
        button(
            "Split vertically",
            Message::Split(pane_grid::Axis::Vertical, pane),
        )
    ]
    .push_maybe(if total_panes > 1 && !is_pinned {
        Some(button("Close", Message::Close(pane)).style(button::danger))
    } else {
        None
    })
    .spacing(5)
    .max_width(160);

    let content = column![text!("{}x{}", size.width, size.height).size(24), controls,]
        .spacing(10)
        .align_x(Center);

    Container::new(scrollable(content))
        .align_y(Center)
        .padding(5)
        .into()
}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
