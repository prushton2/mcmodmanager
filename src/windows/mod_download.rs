use iced::widget::{column, text, container};

use crate::ui::{ModLoader, Message};

pub fn window(_this: &ModLoader) -> iced::Element<'_, Message> {
	
    let element = column![
		text("Downloading and removing mods...")
	];

	return container(element).into()
}