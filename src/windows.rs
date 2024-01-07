use iced::widget::{container, checkbox, text, text_input, button, column, row};

use crate::downloader;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    OsSetWindows,
    OsSetLinux,
	ModSetSodium(bool)
}

pub struct ModLoader {
    pub page: i64,
    pub os: String,
	pub version: String,
	pub has_sodium: bool
}

pub fn null() -> iced::Element<'static, Message> {
	return container(text("Bad state, restart program")).into()
}

pub fn base_settings(this: &ModLoader) -> iced::Element<'_, Message> {
	let element = column![
		text("Select your operating system:\n\n"),
		text(format!("Selected OS: {}", this.os)),
		row![button("windows").on_press(Message::OsSetWindows), button("linux").on_press(Message::OsSetLinux)],
	
		text("Select your game version:\n\n"),
		text_input()	
	];

	return container(element).into()
}

pub fn mods(this: &ModLoader) -> iced::Element<'_, Message> {
	let element = column![
		text("Select Mods:\n"),
		checkbox("Sodium", this.has_sodium, Message::ModSetSodium)
	];

	return container(element).into()
}

pub fn download(this: &ModLoader) -> iced::Element<'_, Message> {
	
	// if this.has_sodium {
	// 	downloader.download(downloadables[0]);
	// }

	return text("Downloading Mods").into()
}