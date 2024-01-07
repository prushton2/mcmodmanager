#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, row};
use iced::{Sandbox, Settings, Renderer};
use iced::alignment::{Horizontal, Vertical};
use iced::Length;

mod file;
mod windows;

static FILE_PATH: &str = "./config";



fn main() -> iced::Result {

    let mut settings = Settings::default();
    settings.window.size = (400, 400);

    return windows::ModLoader::run(settings)
}

impl Sandbox for windows::ModLoader {
    type Message = windows::Message;

    fn new() -> Self {

        let load_result = file::load_config(FILE_PATH);
        let config: file::Config;

        if load_result.is_ok() {
            config = load_result.unwrap();
        } else {
            config = file::Config {
                os: String::from("windows"),
                version: String::from(""),
            };
        }

        return Self {
            page: 0,
            os: config.os,
            version: config.version,
            has_sodium: false
        }
    }

    fn title(&self) -> String {
        return String::from("MC Mod Loader")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::Next => self.page += 1,
            Self::Message::Previous => self.page -= 1,
            
            Self::Message::OsSetLinux => {
                self.os = String::from("linux");
                save_state(&self);
            },
            Self::Message::OsSetWindows => {
                self.os = String::from("windows");
                save_state(&self);
            },
            
            Self::Message::ModSetSodium(state) => self.has_sodium = state
        };
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        
        let next: iced::widget::Button<'_, Self::Message, Renderer> = button("Back").on_press(Self::Message::Previous);
        let prev: iced::widget::Button<'_, Self::Message, Renderer> = button("Next").on_press(Self::Message::Next);
        
        let selected_window;

        match self.page {
            0 => selected_window = windows::base_settings(&self),
            1 => selected_window = windows::mods(&self),
            1 => selected_window = windows::download(&self),
            _ => selected_window = windows::null()
        };
        
        let element = column![
            selected_window,
            text("\n\n"),
            container(row![next, prev]).align_x(Horizontal::Right).align_y(Vertical::Bottom)
        ];

        return container(element).height(Length::Fill).width(Length::Fill).into()
        
    }

}

fn save_state(this: &windows::ModLoader) {
    let config: file::Config = file::Config {
        os: this.os.clone(),
        version: this.version.clone()
    };

    let _ = file::write_config(FILE_PATH, config);
}