use iced::widget::{container, text, text_input, column};

use crate::ui::{ModLoader, Message};

pub fn window(this: &ModLoader) -> iced::Element<'_, Message> {
    
    let element = column![
		text(format!("Detected OS: {}", this.os)),	
		text("\n\nSelect your game version:"),
		text_input("1.20.4", &this.version).on_input(Message::SetVersion)
	];

	return container(element).into()
}
