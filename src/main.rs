#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, button, column, row};
use iced::{Sandbox, Settings};

mod file;
mod windows;

static FILE_PATH: &str = "./config";

fn main() -> iced::Result {
    windows::ModLoader::run(Settings::default())
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
            };
        }

        return Self {
            page: 0,
            os: config.os
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
        };
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        
        let next = button("Back").on_press(Self::Message::Previous);
        let prev = button("Next").on_press(Self::Message::Next);
        
        let selected_window;

        match self.page {
            0 => selected_window = windows::select_os(&self),
            _ => selected_window = windows::null()
        };
        
        let element = column![
            selected_window,
            row![next, prev]
        ];

        return container(element).into()
        
    }

}

fn save_state(this: &windows::ModLoader) {
    let config: file::Config = file::Config {
        os: this.os.clone()
    };

    let _ = file::write_config(FILE_PATH, config);
}