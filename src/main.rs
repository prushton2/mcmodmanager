#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, Row, row};
use iced::{Application, Settings, Renderer, executor, Theme, Command};
use iced::{Alignment, Color, Length};

use dirs;

use num::clamp;

mod ui;
mod windows;

use crate::ui::Page;
use crate::windows::{version_select, mod_select, mod_search, mod_download, check_fabric};
use std::process::exit;
use std::env::consts;


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

        let home_dir_option = dirs::home_dir();
        
        if home_dir_option.is_none() {
            exit(1);
        }

        let minecraft_dir: &str;

        match consts::OS {
            "windows" => minecraft_dir = "AppData/Roaming/.minecraft",
            "linux" => minecraft_dir = ".minecraft",
            _ => exit(1)
        };

        return (Self {
            page: 0,
            version: "".to_string(),
            mods: vec![],
            search_query: "".to_string(),
            search_results: vec![],
            os: consts::OS.to_string(),
            home_dir: home_dir_option.unwrap().to_str().unwrap().to_string(),
            minecraft_dir: minecraft_dir.to_string()
        }, Command::none())
    }

    fn title(&self) -> String {
        return String::from("MC Mod Loader")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        
        let mut command = Command::none();

        match message {
            Self::Message::ChangePage(n) => {
                self.page += n;
            },
            _ => ()
        }

        self.page = clamp(self.page, 0, ui::Page::count());

        return command;
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        let button_config = ui::ButtonConfig::new();
        let selected_window: iced::Element<'_, Self::Message>;// = text(format!("{:?}", ui::Page::cast(self.page))).into(); 
        

        match Page::cast(self.page) {
            Page::VersionSelect => selected_window = version_select::window(&self),
            Page::ModSelect => selected_window = mod_select::window(&self),
            Page::ModSearch => selected_window = mod_search::window(&self),
            Page::ModDownload => selected_window = mod_download::window(&self),
            Page::CheckFabric => {
                let has_fabric_result = check_fabric::has_fabric_installed(&self);
                selected_window = check_fabric::window(&self, &has_fabric_result);
            },
            Page::Exit => exit(0),
            _ => selected_window = windows::null()
        }
        
        
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
                    text(format!("{} | {:?}", self.page, Page::cast(self.page))),
                    Row::with_children(buttons).padding(5).spacing(5)
                ].height(Length::Fill).align_items(Alignment::End)
            ].width(Length::Fill).height(Length::Fill).align_items(Alignment::End)
        
        ].width(Length::Fill).height(Length::Fill);

        return container(elements).width(Length::Fill).height(Length::Fill).into();        
    }

}
