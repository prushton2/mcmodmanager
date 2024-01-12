#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, row};
use iced::{Application, Settings, Renderer, executor, Theme, Command};
use iced::alignment::{Horizontal, Vertical};
use iced::Length;

use std::collections::HashMap;

// use tokio::time::{sleep, Duration};

use std::cmp::{min, max};

mod file;
mod windows;
mod downloader;

static FILE_PATH: &str = "./config";


fn main() -> iced::Result {

    let mut settings = Settings::default();
    settings.window.size = (400, 400);

    return windows::ModLoader::run(settings)
}

impl Application for windows::ModLoader {
    type Message = windows::Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {

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

        let mut hm: HashMap<String, bool> = HashMap::new();

        for (key, _value) in downloader::MODS.entries.iter() {
            hm.insert(key.to_string(), false);
        }

        return (Self {
            page: 0,
            os: config.os,
            version: config.version,
            mods: hm
        }, Command::none())
    }

    fn title(&self) -> String {
        return String::from("MC Mod Loader")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        
        let mut download = false;
        
        match message {
            Self::Message::Next => self.page += 1,
            Self::Message::Previous => self.page -= 1,

            Self::Message::VersionSet(state) => {
                self.version = state;
                save_state(&self);
            },
            Self::Message::SetOS(state) => {
                self.os = state;
                save_state(&self);
            },
            Self::Message::SetMod(state, mod_name) => {
                self.mods.insert(mod_name, state);
            },
            Self::Message::ConfirmDownload => {
                download = true;
            },
            Self::Message::DownloadComplete(_result) => {
                self.page += 1;
            }
        };
        self.page = max(min(self.page, 3), 0);

        if download {
            return Command::perform(downloader::download(self.version.clone(), self.os.clone(), self.mods.clone()), Self::Message::DownloadComplete)
        } else {
            return Command::none()
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        
        let next: iced::widget::Button<'_, Self::Message, Renderer> = button("Back").on_press(Self::Message::Previous);
        let prev: iced::widget::Button<'_, Self::Message, Renderer> = button("Next").on_press(Self::Message::Next);
        
        let selected_window;


        match self.page {
            0 => selected_window = windows::base_settings(&self),
            1 => selected_window = windows::mods(&self, &downloader::MODS),
            2 => selected_window = windows::download(&self),
            3 => selected_window = windows::done(&self),
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