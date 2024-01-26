use iced::widget::{column, text, container};

use std::fs;
use reqwest;

use crate::ui::{ModLoader, Message, download_file};

pub fn window(_this: &ModLoader) -> iced::Element<'_, Message> {
	
    let element = column![
		text("Downloading and removing mods...")
	];

	return container(element).into()
}

pub async fn download<'a>(this: ModLoader) -> Result<&'a str, &'a str> {
    let client = reqwest::blocking::Client::new();
    
    let mods_dir = fs::create_dir_all(
        format!("{}/{}/mods", 
            this.home_dir, this.minecraft_dir)
    );

    if mods_dir.is_err() {
        return Err("Error making mods directory")
    }

    let mods_dir_result = fs::read_dir(
        format!("{}/{}/mods",
			this.home_dir, this.minecraft_dir)
    );

    if mods_dir_result.is_err() {
        return Err("Error deleting existing mods");
    }

    let mods_dir = mods_dir_result.unwrap();

    for mod_path in mods_dir {
        let mod_name_os_string = mod_path.unwrap().file_name();
        let file = mod_name_os_string.into_string().unwrap();
        
        let _file_name = &file[..file.len()-4];
        let extension = &file[file.len()-8..file.len()-4];
        let mod_name = &file[..file.len()-8];

        if extension != "-mcm" {
            continue;
        }
        
        if this.mods.iter().any(|i| i == &mod_name ) {
            continue;
        }

        let res = fs::remove_file(
            format!("{}/{}/mods/{}",
                this.home_dir, this.minecraft_dir, file)
        );
        if res.is_ok() {
            println!("Removed {}", file);
        }        
    }


    for slug in this.mods.iter() {

        //get the right file by looking for a file with the correct version and mod loader
        let version_response = client
            .get(format!("https://api.modrinth.com/v2/project/{}/version", slug))
            .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
            .send();

        let body = version_response.unwrap().text().unwrap();
        let version_object = json::parse(&body).unwrap();
        
        let mut file_index: usize = 0;

        loop {
            if version_object[file_index]["game_versions"].contains(json::JsonValue::from(this.version.clone())) &&
               version_object[file_index]["loaders"].contains(json::JsonValue::from("fabric")) {
                break;
            }
            file_index += 1;
        }
        
        let file_path = format!("{}/{}/mods/{}-mcm.jar",
            this.home_dir, this.minecraft_dir, slug);

        let result = download_file(
                version_object[file_index]["files"][0]["url"].as_str().unwrap(),
                file_path.as_str()
            ).await;


        if result.is_err() {
            return Err("Error downloading Mod");
        }
        println!("Downloaded {}", slug);
    }
	
	return Ok("Operation completed");
}