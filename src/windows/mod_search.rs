use iced::widget::{container, text, text_input, button, Column, row};

use crate::ui::{ModLoader, Message};


pub fn window(this: &ModLoader) -> iced::Element<'_, Message> {
    let mut elements: Vec<iced::Element<'_, Message, iced::Renderer>> = vec![];
    elements.push(text("Search\n").into());
    elements.push(text_input("Search for mods", &this.search_query).on_input(Message::SetQuery).into());
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

pub async fn search_modrinth<'a>(query: String) -> Result<Vec<String>, &'a str> {
    let client = reqwest::blocking::Client::new();

    let search_response = client
        .get(format!("https://api.modrinth.com/v2/search?query={}", query))
        .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
        .send();

    if search_response.is_err() {
        return Err("Error making request");
    }

    let body = search_response.unwrap().text().unwrap();
    let search_object = json::parse(&body).unwrap();

    let mut slugs: Vec<String> = vec![];
    let mut index: usize = 0;

    loop {
        let mut string_value = json::stringify(search_object["hits"][index]["slug"].clone());

        if string_value == "null" {
            break;
        }

        string_value.remove(0);
        string_value.pop();

        slugs.push(string_value.clone());
        index += 1;
    }

    return Ok(slugs);
}