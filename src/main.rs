#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, Row, row, Column};
use iced::{Application, Settings, Renderer, executor, Theme, Command};
use iced::{Alignment, Color, Element, Length};
use iced::alignment::Horizontal;

// use std::process::exit;
// use std::env::consts;
// use std::cmp::{min, max};

mod ui;

fn main() -> iced::Result {

    let mut settings = Settings::default();
    settings.window.size = (400, 400);

    return ui::ModLoader::run(settings)
}

impl Application for ui::ModLoader {
    type Message = ui::Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {

        return (Self {
            page: 0,
            os: "FIXME".to_string(),
            version: "".to_string(),
            mods: vec![],
            search_query: "".to_string(),
            search_results: vec![]
        }, Command::none())
    }

    fn title(&self) -> String {
        return String::from("MC Mod Loader")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        
        let mut command = Command::none();

        return command;
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        let button_config = ui::ButtonConfig::new();
        let selected_window: iced::Element<'_, Self::Message> = text("window").into(); 
        
        
        















        let next: iced::widget::Button<'_, Self::Message, Renderer> = button(button_config.next_name).on_press(Self::Message::ChangePage(button_config.next_page));
		let prev: iced::widget::Button<'_, Self::Message, Renderer> = button(button_config.prev_name).on_press(Self::Message::ChangePage(button_config.prev_page));
        let mut buttons: Vec<iced::Element<'_, Self::Message, Renderer>> = vec![];		
		if button_config.prev_show {
			buttons.push(prev.into());
		}
		if button_config.next_show {
			buttons.push(next.into());
		}

        let elements = column![
            column![selected_window].padding(5),

            column![
                row![
                    Row::with_children(buttons).padding(5).spacing(5)
                ].height(Length::Fill).align_items(Alignment::End)
            ].width(Length::Fill).height(Length::Fill).align_items(Alignment::End)
        
        ].width(Length::Fill).height(Length::Fill);

        return container(elements).width(Length::Fill).height(Length::Fill).into();        
    }

}