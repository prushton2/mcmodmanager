use iced::widget::{text, button, column};

use regex::Regex;

use std::fs;

use crate::ui::{ModLoader, Message};


pub fn window<'a>(_this: &ModLoader, has_fabric: &Result<bool, &str>) -> iced::Element<'a, Message> {

    let fabric_found = column![
        text("Fabric was found on your system.\n\n"),
        button("Install Fabric Anyway").on_press(Message::ChangePage(1))
    ];

    let fabric_not_found = column![
        text("Fabric was not found on your system"),
    ];
    if has_fabric.is_err() {
        return column![
            text(format!("Error locating fabric: {:?}", has_fabric.err()))
        ].into();
    }

    if has_fabric.clone().unwrap() {
        return fabric_found.into();
    }

    return fabric_not_found.into();
}


pub fn has_fabric_installed<'a>(this: &'a ModLoader) -> Result<bool, &'a str> {

    let versions_result = fs::read_dir(
        format!("{}/{}/versions/",
                this.home_dir, this.minecraft_dir));

    if versions_result.is_err() {
        return Err("Error finding versions");
    }
    let versions = versions_result.unwrap();

    let re = Regex::new(
        format!(r"fabric-loader-[0-9]*\.[0-9]*\.[0-9]*-{}", this.version).as_str()
    ).unwrap();

    for path in versions {
        let path_string = path.unwrap().file_name().into_string().unwrap();
        
        if re.is_match(&path_string) {
            return Ok(true);
        }
    }

    return Ok(false);
}