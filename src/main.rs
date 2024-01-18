#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, Row};
use iced::{Application, Settings, Renderer, executor, Theme, Command};

use std::collections::HashMap;
use std::process::exit;

// use tokio::time::{sleep, Duration};

use std::cmp::{min, max};

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

        let mut hm: HashMap<String, bool> = HashMap::new();

        for (key, _value) in downloader::MODS.entries.iter() {
            hm.insert(key.to_string(), false);
        }

        return (Self {
            page: 0,
            os: "".to_string(),
            version: "".to_string(),
            mods: hm,
            response: "".to_string()
        }, Command::none())
    }

    fn title(&self) -> String {
        return String::from("MC Mod Loader")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        
        let mut pageinit = false; 
        //kinda gross, used to make sure actions that should run on page init run once
        let mut command = Command::none();

        match message {
            Self::Message::ChangePage(pages) => {
                self.page += pages;
                pageinit = true;
            }
            Self::Message::VersionSet(state) => {
                self.version = state;
            },
            Self::Message::SetOS(state) => {
                self.os = state;
            },
            Self::Message::SetMod(state, mod_name) => {
                self.mods.insert(mod_name, state);
            },
            Self::Message::DownloadComplete(result) => {
                if result.is_err() {
                    println!("{:?}", result.err());
                }
                self.page += 1;
            },
            Self::Message::LaunchFabric(_result) => {
                self.page += 1;
            }
        };
        self.page = max(min(self.page, 10), 0);

        if self.page == 1 && pageinit {
            self.mods = downloader::get_installed_mods(self.os.clone()).clone();
        }

        match self.page {
            2 => command = Command::perform(downloader::download(self.version.clone(), self.os.clone(), self.mods.clone()), Self::Message::DownloadComplete),
            4 => command = Command::perform(downloader::download_fabric(self.os.clone()), Self::Message::LaunchFabric),
            _ => command = Command::none(),
        }

        return command;
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        
        struct ButtonConfig<'a> {
            next_name: &'a str,
            prev_name: &'a str,
            show_next: bool,
            show_prev: bool,
            next_page: i32,
            prev_page: i32
        }

        let mut button_config: ButtonConfig = ButtonConfig {
            next_name: "Next",
            prev_name: "Back",
            show_next: true,
            show_prev: true,
            next_page: 1, //amount of pages to change when button is pressed
            prev_page: -1
        };

        let selected_window;

        match self.page {
            0 => selected_window = windows::base_settings(&self),
            1 => {
                selected_window = windows::mods(&self, &downloader::MODS);
                button_config.next_name = "Download";
            },
            2 => {
                selected_window = windows::download(&self);
                button_config.show_next = false;
                button_config.show_prev = false;
            },
            3 => {
                let has_fabric_result = downloader::has_fabric_installed(self.os.clone(), self.version.clone());

                
                selected_window = windows::find_fabric(&self, has_fabric_result.clone());

                if has_fabric_result.is_ok() {
                    if !has_fabric_result.clone().unwrap() {
                        button_config.next_name = "Install Fabric";
                    } else {
                        button_config.next_page = 3;
                    }
                }

            },
            4 => {
                selected_window = windows::install_fabric(&self);
            },
            5 => {
                let home_os_config: (String, downloader::Directories) = downloader::get_home_os_config(self.os.clone());

                let fabric_dir = format!("{}{}{}{}fabric-installer.jar",
                    home_os_config.0, home_os_config.1.seperator, home_os_config.1.minecraft_dir, home_os_config.1.seperator);

                selected_window = windows::launch_fabric(&self, fabric_dir.clone());
            }
            6 => {
                selected_window = windows::done(&self);
            },
            7 => {
                exit(0);
            }
            _ => selected_window = windows::null()
        };
        
        // exit(0);

        let next: iced::widget::Button<'_, Self::Message, Renderer> = button(button_config.next_name).on_press(Self::Message::ChangePage(button_config.next_page));
        let prev: iced::widget::Button<'_, Self::Message, Renderer> = button(button_config.prev_name).on_press(Self::Message::ChangePage(button_config.prev_page));
        
        let mut buttons: Vec<iced::Element<'_, Self::Message, Renderer>> = vec![];

        if button_config.show_prev {
            buttons.push(prev.into());
        }
        if button_config.show_next {
            buttons.push(next.into());
        }

        let elements = column![
            selected_window,
            text("\n\n"),
            text(format!("page {}\n", self.page)),
            Row::with_children(buttons)
        ];

        return container(elements).into()
        
    }

}