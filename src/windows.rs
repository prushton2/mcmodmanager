use iced::widget::{container, text, button, column, row};

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    OsSetWindows,
    OsSetLinux
}

pub struct ModLoader {
    pub page: i64,
    pub os: String,
}

pub fn null() -> iced::Element<'static, Message> {
	return container(text("Bad state, restart program")).into()
}

pub fn select_os(this: &ModLoader) -> iced::Element<'_, Message> {
	let element = column![
		text("Select your operating system:\n\n"),
		text(format!("Selected OS: {}", this.os)),
		row![button("windows").on_press(Message::OsSetWindows), button("linux").on_press(Message::OsSetLinux)],
	];

	return container(element).into()
}