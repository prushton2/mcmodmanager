use iced::widget::{text};
use std::process::Command;
use execute::Execute;
use std::fs;


use crate::ui::{ModLoader, Message};


pub fn window<'a>(this: &ModLoader) -> iced::Element<'a, Message> {

    let fabric_location: String = format!("{}/{}/fabric-installer.jar", this.home_dir, this.minecraft_dir);

    let mut command = Command::new("java");
    command.arg("-jar");
    command.arg(&fabric_location);

    let result = command.execute_output();
    
    if result.is_ok() {
        let _ = fs::remove_file(&fabric_location);
        return text("Fabric exited successfully").into();
    } else {
        let errmsg = format!("There was an error: {:?}", result.err());
        return text(errmsg.as_str()).into();
    }

}