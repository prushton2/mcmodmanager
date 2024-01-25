use iced::widget::{container, text, text_input, button, Column, row};

use crate::ui::{ModLoader, Message};

pub fn window<'a>(this: &ModLoader) -> iced::Element<'a, Message> {
	
    let mut elements: Vec<iced::Element<'_, Message, iced::Renderer>> = vec![];
    elements.push(
        text("Selected Mods:\n").into()
    );
    elements.push(
        text_input("Search for mods", &this.search_query).on_input(Message::QuerySet).into()
    );
    elements.push(
        text("\n").into()
    );

    for key in this.mods.clone() {
        elements.push(
            container(
            row![
                button(" - ").on_press(Message::SetMod(key.clone(), false)),
                text(key.as_str())
            ]).into()
            
        );
    }

    let element = Column::with_children(elements);

	return container(element).into()
}
