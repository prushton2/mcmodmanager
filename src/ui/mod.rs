use std::fs;
use iced::{Renderer};
use iced::widget::{button, Row};

use strum::{IntoEnumIterator};
use strum_macros::{EnumIter};

#[derive(Clone)]
pub struct ModLoader {
    pub page: i32,
	pub version: String,
	pub mods: Vec<String>,
    pub search_query: String,
    pub search_results: Vec<String>,
    pub os: String,
	pub home_dir: String,
	pub minecraft_dir: String
}


#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(i32),
    
	SetVersion(String),
	
    SetQuery(String),
    SearchResultSet(Result<Vec<String>, &'static str>),
	Search,

    SetMod(String, bool),
    DownloadComplete(Result<&'static str, &'static str>),

    LaunchFabric(Result<&'static str, &'static str>),
}

#[derive(EnumIter, Debug, PartialEq)]
pub enum Page {
	VersionSelect,
	ModSelect,
	ModSearch,
	ModDownload,
	CheckFabric,
	DownloadFabric,
	LaunchFabric,
	Finish,
	Exit
}

impl Page {
	pub fn cast(i: i32) -> Self {
		let mut iterable = Page::iter();
				
		let result = iterable.nth(i as usize);

		if result.is_some() {
			return result.unwrap();
		}

		return Page::VersionSelect
	}

	pub fn count() -> i32 {
		return (Page::iter().count()-1) as i32;
	}
}


pub struct ButtonConfig<'a> {
	pub next_name: &'a str,
	pub prev_name: &'a str,
	pub next_show: bool,
	pub prev_show: bool,
	pub next_page: i32,
	pub prev_page: i32
}

impl ButtonConfig<'_> {
	
	pub fn new() -> Self {
		ButtonConfig {
			next_name: "Next",
			prev_name: "Back",
			next_show: true,
			prev_show: true,
			next_page: 1, //amount of pages to change when button is pressed
			prev_page: -1
		}
	}

	//cannot use due to lack of .clone
	pub fn _generate_buttons(&self) -> iced::Element<'_, Message> {
		let next: iced::widget::Button<'_, Message, Renderer> = button(self.next_name).on_press(Message::ChangePage(self.next_page));
		let prev: iced::widget::Button<'_, Message, Renderer> = button(self.prev_name).on_press(Message::ChangePage(self.prev_page));
		
		let mut buttons: Vec<iced::Element<'_, Message, Renderer>> = vec![];
		
		if self.prev_show {
			buttons.push(prev.into());
		}
		if self.next_show {
			buttons.push(next.into());
		}

		return Row::with_children(buttons).into()
	}

}


pub async fn download_file<'a>(url: &'a str, path: &str) -> Result<&'a str, &'a str> {
	let client = reqwest::blocking::Client::new();
	let file_response = client
		.get(url)
		.header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
		.send();

	let mut file_data = std::io::Cursor::new(file_response.unwrap().bytes().unwrap());

	let file_result = fs::File::create(path);

	let mut file = file_result.unwrap();
	let resp = std::io::copy(&mut file_data, &mut file);

	if resp.is_err() {
		return Err("Error copying data to file");
	}

	return Ok("File downloaded");
}