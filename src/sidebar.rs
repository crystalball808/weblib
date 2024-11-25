use iced::{
    widget::{button, column, container, scrollable, text, Column},
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
        let tabs_column: Element<Message> =
            scrollable(Column::from_vec(tabs_column).spacing(4)).into();

        let content = column![library_button, tabs_column].spacing(8);

        container(content)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style::default().background(palette.background.weak.color)
            })
            .padding(4)
            .height(Fill)
            .width(200)
            .into()
    }
}
