use iced::widget::{container, checkbox, text, text_input, button, column, Column, row};
use std::collections::HashMap;

use crate::downloader;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    
	VersionSet(String),

	SetOS(String),
	
	SetMod(bool, String),

    ConfirmDownload,
    DownloadComplete(Result<String, String>)
}

pub struct ModLoader {
    pub page: i64,
    pub os: String,
	pub version: String,
	pub mods: HashMap<String, bool>
}

pub fn null() -> iced::Element<'static, Message> {
	return container(text("Bad state, restart program")).into()
}

pub fn base_settings(this: &ModLoader) -> iced::Element<'_, Message> {
	let element = column![
		text("Select your operating system:\n\n"),
		text(format!("Selected OS: {}", this.os)),
		row![
            button("windows").on_press(Message::SetOS(String::from("windows"))), 
            button("linux").on_press(Message::SetOS(String::from("linux")))],
	
		text("\n\nSelect your game version:"),
		text_input("1.20.4", &this.version).on_input(Message::VersionSet)	
	];

	return container(element).into()
}

pub fn mods<'a>(this: &ModLoader, mods: &phf::Map<&str, downloader::ModInfo<'a>>) -> iced::Element<'a, Message> {
	

        
    let mut elements: Vec<iced::Element<'_, Message, iced::Renderer>> = vec![];
    elements.push(
        text("Select Mods:\n").into()
    );


    for (key, _val) in mods.entries.iter() {
        elements.push(
            checkbox(key.to_string(), *(this.mods.get(*key).unwrap()), |v| Message::SetMod(v, key.to_string())).into()
        );
    }

    let element = Column::with_children(elements);

	return container(element).into()
}

pub fn download(_this: &ModLoader) -> iced::Element<'_, Message> {
	
    let element = column![
		button("Download").on_press(Message::ConfirmDownload)
	];

	return container(element).into()
}