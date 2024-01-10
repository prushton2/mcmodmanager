use iced::widget::{container, checkbox, text, text_input, button, column, row};
use std::thread;

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
	pub has_sodium: bool
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

pub fn mods(this: &ModLoader) -> iced::Element<'_, Message> {
	let element = column![
		text("Select Mods:\n"),
		checkbox("Sodium", this.has_sodium, |v| Message::SetMod(v, String::from("Sodium")))
	];

	return container(element).into()
}

pub fn download(this: &ModLoader) -> iced::Element<'_, Message> {
	


	// if this.has_sodium {
    // 	println!("Spawning thread");
	// 	let downloader = thread::spawn(|| {
    //         downloader::download(&downloader::downloadables[0]);
    //     });

    // 	println!("joining thread");
    //     downloader.join();
	// }

    let element = column![
		button("Download").on_press(Message::ConfirmDownload)
	];

	return container(element).into()
}