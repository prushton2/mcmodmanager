use iced::widget::{container, text};


use crate::ui::{Message};

pub mod version_select;
pub mod mod_select;
pub mod mod_search;
pub mod mod_download;
pub mod check_fabric;
pub mod download_fabric;
pub mod launch_fabric;
pub mod finish;

pub fn null() -> iced::Element<'static, Message> {
	return container(text("Bad state, restart program")).into()
}