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
    DownloadComplete(Result<String, String>),

    InstallFabric
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
            button("windows").on_press(Message::SetOS(String::from("Windows"))), 
            button("linux").on_press(Message::SetOS(String::from("Linux")))],
	
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
		text("Downloading and removing mods...")
	];

	return container(element).into()
}

pub fn downloadFabric(this: &ModLoader, has_fabric: Result<bool, &str>) -> iced::Element<'_, Message> {

    let fabric_found = column![
        text("Fabric was found on your system.")
    ];

    let fabric_not_found = column![
        text("Fabric was not found on your system"),
        button("windows").on_press(Message::InstallFabric)
    ];

    let fabric_err = column![
        text("There was an error locating fabric"),
    ];

    if has_fabric.is_err() {
        return fabric_err.into();
    }

    if has_fabric.unwrap() {
        return fabric_found.into();
    }

    return fabric_not_found;
}

pub fn done(_this: &ModLoader) -> iced::Element<'_, Message> {

    return text("Mod download is done. Now install fabric with this link: https://fabricmc.net/").into()



}