use iced::{
    widget::{button, column, container, scrollable, text, Column},
    Element,
    Length::Fill,
    Theme,
};
use uuid::Uuid;

use crate::{
    tabs::{Tab, TabHistoryEntry},
    Message,
};

pub struct Sidebar;

impl Sidebar {
    pub fn view(tabs: &Vec<Tab>, active_tab_id: Option<Uuid>) -> Element<Message> {
        let library_button: Element<Message> = button(text("Library"))
            .on_press(Message::CreateLibraryTab)
            .into();

        let tabs_column: Vec<Element<Message>> = tabs
            .iter()
            .map(|tab| match tab.active_entry() {
                TabHistoryEntry::Library => {
                    if active_tab_id.is_some() && tab.id == active_tab_id.unwrap() {
                        button(text("Library"))
                            .style(button::primary)
                            .width(Fill)
                            .on_press(Message::SelectTab(tab.id))
                            .into()
                    } else {
                        button(text("Library"))
                            .style(button::secondary)
                            .width(Fill)
                            .on_press(Message::SelectTab(tab.id))
                            .into()
                    }
                }
            })
            .collect();
        let tabs_column: Element<Message> =
            scrollable(Column::from_vec(tabs_column).spacing(4).width(Fill))
                .width(Fill)
                .into();

        let content = column![library_button, tabs_column].spacing(8).width(Fill);

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
