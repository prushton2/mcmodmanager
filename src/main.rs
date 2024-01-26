#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, Row, row};
use iced::{Application, Settings, Renderer, executor, Theme, Command};
use iced::{Alignment, Color, Length};

use dirs;

use num::clamp;

mod ui;
mod windows;

use crate::ui::Page;

use crate::windows::{version_select, mod_select, mod_search, mod_download, check_fabric, download_fabric, launch_fabric, finish};
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

            Self::Message::SetVersion(v) => {
                self.version = v;
            },

            Self::Message::SetQuery(q) => {
                if q == "" {
                    self.page = 1;
                } else {
                    self.page = 2;
                }

                self.search_query = q;
            },

            Self::Message::SetMod(mod_name, state) => {
                if state {
                    self.mods.push(mod_name.clone());
                    self.search_query = "".to_string();
                    self.page = 1;
                } else {

                    let mut i = 0; //https://imgflip.com/i/8cu5sh
                    while i < self.mods.len() {
                        if self.mods[i] == mod_name {
                            let _ = self.mods.remove(i);
                            break;
                        }
                        i += 1;
                    }

                }
            },


            Self::Message::SearchResultSet(vec) => {
                self.search_results = vec.unwrap().clone();
            }

            Self::Message::Search => {
                command = Command::perform(mod_search::search_modrinth(self.search_query.clone()), Self::Message::SearchResultSet);
            }

            Self::Message::DownloadComplete(result) => {
                self.page += 1;
            }

            Self::Message::LaunchFabric(result) => {
                self.page += 1;
            }

            _ => ()
        };

        self.page = clamp(self.page, 0, ui::Page::count());

        //random page specific functions (launching commands, reading filesystem, etc)
        match ui::Page::cast(self.page) {

            Page::VersionSelect => {
                let result = mod_select::get_installed_mods(&self);
                if result.is_err() {
                    exit(1);
                }
                self.mods = result.unwrap();
            },

            Page::ModDownload => {command = Command::perform(mod_download::download(self.clone()), Self::Message::DownloadComplete)},
            Page::DownloadFabric => {command = Command::perform(download_fabric::download_fabric(self.clone()), Self::Message::LaunchFabric)},
            _ => {}
        }

        return command;
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        let mut button_config = ui::ButtonConfig::new();
        let selected_window: iced::Element<'_, Self::Message>;// = text(format!("{:?}", ui::Page::cast(self.page))).into(); 
        

        match Page::cast(self.page) {
            Page::VersionSelect => selected_window = version_select::window(&self),
            Page::ModSelect => {
                selected_window = mod_select::window(&self);
                button_config.next_page = 2;
                button_config.next_name = "Download";
            },
            Page::ModSearch => {
                selected_window = mod_search::window(&self);
                button_config.next_show = false;
                button_config.prev_show = false;
            },
            Page::ModDownload => {
                selected_window = mod_download::window(&self);
                button_config.next_show = false;
                button_config.prev_show = false;
            },
            Page::CheckFabric => {
                let has_fabric_result = check_fabric::has_fabric_installed(&self);
                selected_window = check_fabric::window(&self, &has_fabric_result);
                button_config.prev_page = -4;

                if has_fabric_result.is_ok() {
                    if !has_fabric_result.unwrap() { //what
                        button_config.next_name = "Install";
                    } else {
                        button_config.next_name = "Skip";
                        button_config.next_page = 3;
                    }
                }

            },
            Page::DownloadFabric => {
                selected_window = download_fabric::window(&self);
            },
            Page::LaunchFabric => {
                selected_window = launch_fabric::window(&self);
            },
            Page::Finish => {
                selected_window = finish::window(&self);
                button_config.next_name = "Finish";                
            }
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
