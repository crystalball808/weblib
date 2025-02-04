use iced::{
    widget::{container, text},
    Border, Color, Element,
};
use uuid::Uuid;

use super::Message;

pub enum ToastVariant {
    Info,
    Error,
}
pub struct Toast {
    title: String,
    variant: ToastVariant,
    id: Uuid,
}

impl Toast {
    pub fn new(title: &str, variant: ToastVariant) -> Self {
        Self {
            title: title.to_owned(),
            variant,
            id: Uuid::new_v4(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let border_color = match self.variant {
            ToastVariant::Info => Color::from_rgb8(23, 23, 240),
            ToastVariant::Error => Color::from_rgb8(240, 23, 23),
        };
        container(text(&self.title))
            .style(move |_| container::Style {
                border: Border {
                    color: border_color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }
}
