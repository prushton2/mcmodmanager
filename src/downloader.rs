use reqwest;
use phf::{phf_map};
use std::collections::HashMap;
use json;
use std::fs;
use std::env;
use std::io::{copy, Write};

pub struct ModInfo<'a> {
    slug: &'a str,
    dependencies: Vec<&'a str>, //[String; 8]
}

pub static MODS: phf::Map<&str, ModInfo> = phf_map! {
    "Sodium"      => ModInfo {slug: "sodium",            dependencies: vec![]},
    "Iris"        => ModInfo {slug: "iris",              dependencies: vec![]}, 
    "Carpet"      => ModInfo {slug: "carpet",            dependencies: vec![]},
    "Audioplayer" => ModInfo {slug: "audioplayer",       dependencies: vec![]},
    "Voice Chat"  => ModInfo {slug: "simple-voice-chat", dependencies: vec![]},
    // "MiniHud"     => ModInfo {slug: "minihud",           dependencies: vec![]},
    // "Tweakeroo"   => ModInfo {slug: "tweakeroo",         dependencies: vec![]},
    
    "FabricAPI"   => ModInfo {slug: "fabric-api",    dependencies: vec![]},
    "MaLiLib"     => ModInfo {slug: "malilib",       dependencies: vec![]},
};

pub async fn download(version: String, mods: HashMap<String, bool>) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();

    let string = format!("{}/AppData/Roaming/.minecraft/mods", env::home_dir().unwrap().display());
    let mods_dir = fs::create_dir_all(string.clone());


    println!("PATH HERE {}", string.clone());

    if mods_dir.is_err() {
        println!("Error making directory: {:?}", mods_dir.unwrap());
    }

    for (key, value) in mods.iter() {
        if !value {
            continue;
        }

        let result = MODS.get(key);

        if result.is_none() {
            println!("Error with finding mod {}", key);
            continue;
        }

        let info = result.unwrap();

        let version_response = client
            .get(format!("https://api.modrinth.com/v2/project/{}/version", info.slug))
            .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
            .send();

        let body = version_response.unwrap().text().unwrap();
        let object = json::parse(&body).unwrap();
        
        let mut file_index: usize = 0;

        loop {
            if object[file_index]["game_versions"].contains(json::JsonValue::from(version.clone())) {
                break;
            }
            file_index += 1;
        }

        let file_response = client
            .get(object[file_index]["files"][0]["url"].as_str().unwrap())
            .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
            .send();

        let mut file_data = std::io::Cursor::new(file_response.unwrap().bytes().unwrap());

        let file_result = fs::File::create(format!("{}/AppData/Roaming/.minecraft/mods/{}.jar", env::home_dir().unwrap().display(), info.slug));
        
        let mut file = file_result.unwrap();
        std::io::copy(&mut file_data, &mut file);


        println!("{}: {:?}", info.slug, object[file_index]["files"][0]["url"] );
    }

	
	println!("Download done");


	// if response.is_err() {
	// 	return Err(String::from("Bad Request"));
	// }
	
	// // let body = response?.text().await?;

	// println!("body: {:?}", response);

	return Ok(String::from("returned from fn"))
}