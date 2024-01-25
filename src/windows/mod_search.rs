use iced::widget::{container, text, text_input, button, Column, row};

use crate::ui::{ModLoader, Message};


pub fn window(this: &ModLoader) -> iced::Element<'_, Message> {
    let mut elements: Vec<iced::Element<'_, Message, iced::Renderer>> = vec![];
    elements.push(text("Search\n").into());
    elements.push(text_input("Search for mods", &this.search_query).on_input(Message::QuerySet).into());
    elements.push(button("Search").on_press(Message::Search).into());


    for result in this.search_results.clone() {
        elements.push(
            container(row![
                button(" + ").on_press(Message::SetMod(result.clone(), true)),
                text(result.as_str())
            ]).into()
        );
    }

    let element = Column::with_children(elements);
    return container(element).into()
}