use iced::keyboard::{self, Key};
use iced::widget::pane_grid::{self, Axis, Direction, PaneGrid};
use iced::widget::{Container, container, image, mouse_area, responsive, text};
use iced::{Center, Element, Fill, Subscription};
use std::path::PathBuf;

pub struct Imux {
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Hovered(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    CloseFocused,
}

impl Imux {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(None));

        Imux { panes, focus: None }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new(None));

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) = self.panes.adjacent(pane, direction) {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Message::Hovered(pane) => {
                self.focus = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
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

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, _modifiers| handle_hotkey(key))
    }

    pub fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            pane_grid::Content::new(
                mouse_area(responsive(|_size| view_content(&pane.image_path)))
                    .on_enter(Message::Hovered(id)),
            )
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
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
    match key.as_ref() {
        Key::Character("5") => Some(Message::SplitFocused(Axis::Vertical)),
        Key::Character("t") => Some(Message::SplitFocused(Axis::Horizontal)),
        Key::Character("w") => Some(Message::CloseFocused),
        Key::Character("h") => Some(Message::FocusAdjacent(Direction::Left)),
        Key::Character("j") => Some(Message::FocusAdjacent(Direction::Down)),
        Key::Character("k") => Some(Message::FocusAdjacent(Direction::Up)),
        Key::Character("l") => Some(Message::FocusAdjacent(Direction::Right)),
        _ => None,
    }
}

#[derive(Clone)]
struct Pane {
    is_pinned: bool,
    image_path: Option<PathBuf>,
}

impl Pane {
    fn new(image_path: Option<PathBuf>) -> Self {
        if image_path.is_some() {
            Self {
                is_pinned: false,
                image_path,
            }
        } else {
            Self {
                is_pinned: false,
                image_path: None,
            }
        }
    }
}

fn view_content<'a>(image_path: &Option<PathBuf>) -> Element<'a, Message> {
    if let Some(image_path) = image_path {
        let content = image(image_path);
        Container::new(content).align_y(Center).padding(5).into()
    } else {
        let content = text("test");
        Container::new(content).align_y(Center).padding(5).into()
    }
}
