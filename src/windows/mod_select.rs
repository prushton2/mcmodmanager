use iced::widget::{container, text, text_input, button, Column, row};

use std::fs;

use crate::ui::{ModLoader, Message};

pub fn window<'a>(this: &ModLoader) -> iced::Element<'a, Message> {
	
    let mut elements: Vec<iced::Element<'_, Message, iced::Renderer>> = vec![];
    elements.push(
        text("Selected Mods:\n").into()
    );
    elements.push(
        text_input("Search for mods", &this.search_query).on_input(Message::SetQuery).into()
    );
    elements.push(
        text("\n").into()
    );

    for key in this.mods.clone() {
        elements.push(
            container(
            row![
                button(" - ").on_press(Message::SetMod(key.clone(), false)),
                text(key.as_str())
            ]).into()
            
        );
    }

    let element = Column::with_children(elements);

	return container(element).into()
}


pub fn get_installed_mods(this: &ModLoader) -> Result<Vec<String>, &'static str> {
    let mut mods: Vec<String> = vec![];

    let mods_result = fs::read_dir(
        format!("{}/{}/mods/",
            this.home_dir, this.minecraft_dir));

    if mods_result.is_err() {
        return Err("Error finding versions: {:?}");
    }
 
    let mods_dir = mods_result.unwrap();

    for mod_name in mods_dir {
        if mod_name.is_err() {
            continue;
        }

        let file = mod_name.unwrap().file_name().into_string().unwrap();
        
        let _file_name = &file[..file.len()-4];
        let extension = &file[file.len()-8..file.len()-4];
        let mod_name = &file[..file.len()-8];

        if extension == "-mcm" {
            mods.push(String::from(mod_name));
        }
    }
    
    return Ok(mods);
}