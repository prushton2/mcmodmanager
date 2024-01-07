#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{container, text, button, column, row};
use iced::{Element, Sandbox, Settings};

mod file;

static FILE_PATH: &str = "./config";


fn main() -> iced::Result {
    
    ModLoader::run(Settings::default())
}


struct ModLoader {
    page: i64,
    os: String,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Previous,
    OsSetWindows,
    OsSetLinux
}

impl Sandbox for ModLoader {
    type Message = Message;

    fn new() -> Self {

        let load_result = file::load_config(FILE_PATH);
        let mut config: file::Config;

        if load_result.is_ok() {
            config = load_result.unwrap();
        } else {
            config = file::Config {
                os: String::from("Windows"),
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
            Message::Next => self.page += 1,
            Message::Previous => self.page -= 1,
            Message::OsSetLinux => {
                self.os = String::from("linux");
                save_state(&self);
            },
            Message::OsSetWindows => {
                self.os = String::from("windows");
                save_state(&self);
            },
        };
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        
        let next = button("Back").on_press(Message::Previous);
        let prev = button("Next").on_press(Message::Next);
        


        
        let element = column![
            text(format!("OS: {}", self.os)),
            row![button("windows").on_press(Message::OsSetWindows), button("linux").on_press(Message::OsSetLinux)],
            row![next, prev]
        ];

        return container(element).into()
        
    }

}

fn save_state(this: &ModLoader) -> Result<&str, &str> {
    let config: file::Config = file::Config {
        os: this.os.clone()
    };

    return file::write_config(FILE_PATH, config)
}
