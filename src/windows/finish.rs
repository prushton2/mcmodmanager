use iced::widget::{text};

use crate::ui::{ModLoader, Message};

pub fn window(_this: &ModLoader) -> iced::Element<'_, Message> {
    return text("Mod download is done. Launch the game under the Fabric profile").into()
}