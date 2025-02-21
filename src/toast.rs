use iced::{
    Border, Color, Element,
    widget::{container, text},
};
use uuid::Uuid;

use crate::Message;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum ToastVariant {
    Info,
    Error,
}
pub struct Toast {
    title: &'static str,
    variant: ToastVariant,
    pub id: Uuid,
}

impl Toast {
    pub fn new(title: &'static str, variant: ToastVariant) -> Self {
        Self {
            title,
            variant,
            id: Uuid::new_v4(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let border_color = match self.variant {
            ToastVariant::Info => Color::from_rgb8(23, 23, 240),
            ToastVariant::Error => Color::from_rgb8(240, 23, 23),
        };
        let bg_color = match self.variant {
            ToastVariant::Info => Color::from_rgb8(60, 60, 210),
            ToastVariant::Error => Color::from_rgb8(200, 60, 60),
        };
        container(text(self.title))
            .style(move |_| container::Style {
                border: Border {
                    color: border_color,
                    width: 3.,
                    radius: 8.0.into(),
                },
                background: Some(bg_color.into()),
                ..Default::default()
            })
            .padding(16.)
            .into()
    }
}
