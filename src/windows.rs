use iced::widget::{container, checkbox, text, text_input, button, column, Column, row};
use std::collections::HashMap;

use crate::downloader;

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(i32),
    
	VersionSet(String),
	SetOS(String),
	SetMod(bool, String),
    DownloadComplete(Result<String, String>),

    LaunchFabric(Result<&'static str, &'static str>),
}

pub struct ModLoader {
    pub page: i32,
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

pub fn find_fabric<'a>(this: &'a ModLoader, has_fabric: Result<bool, String>) -> iced::Element<'a, Message> {

    let fabric_found = column![
        text("Fabric was found on your system.")
    ];

    let fabric_not_found = column![
        text("Fabric was not found on your system"),
        // button("Install Fabric").on_press(Message::ChangePage(1))
    ];

    if has_fabric.is_err() {
        return column![
            text(format!("Error locating fabric: {:?}", has_fabric.err()))
        ].into();
    }

    if has_fabric.clone().unwrap() {
        return fabric_found.into();
    }

    return fabric_not_found.into();
}

pub fn install_fabric(this: &ModLoader) -> iced::Element<'_, Message> {
    //we only get here if fabric is not found
    return text("Downloading Fabric...").into();
}

pub fn done(_this: &ModLoader) -> iced::Element<'_, Message> {
    return text("Mod download is done. Now install fabric with this link: https://fabricmc.net/").into()
}