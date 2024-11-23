use iced::{
    widget::{button, column, container, text, Column},
    Element,
    Length::Fill,
    Theme,
};

use crate::{tabs::Tab, Message};

pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }
    pub fn view(&self, tabs: &Vec<Tab>) -> Element<Message> {
        let library_button: Element<Message> = button(text("Library"))
            .on_press(Message::CreateLibraryTab)
            .into();

        let tabs_column: Vec<Element<Message>> = tabs
            .iter()
            .map(|tab| match tab {
                Tab::Library { id } => button(text(format!("Library {id}"))).into(),
            })
            .collect();
        let tabs_column = Column::from_vec(tabs_column);

        let content = column![library_button, tabs_column];

        container(content)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style::default().background(palette.background.weak.color)
            })
            .height(Fill)
            .width(200)
            .into()
    }
}