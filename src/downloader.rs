use reqwest;
use phf::{phf_map};
use std::collections::HashMap;
use json;
use std::fs;
use dirs;
use regex::Regex;

pub struct ModInfo<'a> {
    slug: &'a str,
    dependencies: Vec<&'a str>, //[String; 8]
}

#[derive(Clone)]
pub struct Directories<'a> {
    seperator: char,
    minecraft_dir: &'a str,
}

pub static MODS: phf::Map<&str, ModInfo> = phf_map! {
    "Sodium"      => ModInfo {slug: "sodium",            dependencies: vec![]},
    "Iris"        => ModInfo {slug: "iris",              dependencies: vec![]}, 
    "Carpet"      => ModInfo {slug: "carpet",            dependencies: vec![]},
    "Audioplayer" => ModInfo {slug: "audioplayer",       dependencies: vec![]},
    "Voice Chat"  => ModInfo {slug: "simple-voice-chat", dependencies: vec![]},
    "WorldEdit"   => ModInfo {slug: "worldedit",         dependencies: vec![]},
    "Nvidium"     => ModInfo {slug: "nvidium",           dependencies: vec![]},
    "Bobby"       => ModInfo {slug: "bobby",             dependencies: vec![]},
    "ModMenu"     => ModInfo {slug: "modmenu",           dependencies: vec![]},

    // please put these on modrinth please please please
    // "MiniHud"     => ModInfo {slug: "minihud",           dependencies: vec![]},
    // "Tweakeroo"   => ModInfo {slug: "tweakeroo",         dependencies: vec![]},
    
    "FabricAPI"   => ModInfo {slug: "fabric-api",    dependencies: vec![]},
    "MaLiLib"     => ModInfo {slug: "malilib",       dependencies: vec![]},
};

pub static WINDOWS_DIR: Directories = Directories {
    seperator: '\\',
    minecraft_dir: "AppData\\Roaming\\.minecraft"
};

pub static LINUX_DIR: Directories = Directories {
    seperator: '/',
    minecraft_dir: ".minecraft"
};

pub static FABRIC_URL: &str = "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.0.0/fabric-installer-1.0.0.jar";

pub async fn download(version: String, os: String, mods: HashMap<String, bool>) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let os_config: Directories;
    let home_dir_option = dirs::home_dir().unwrap();
    let home_dir = home_dir_option.to_str().unwrap();

    match os.as_str() {
        "Windows" => os_config = WINDOWS_DIR.clone(),
        "Linux" => os_config = LINUX_DIR.clone(),
        _ => os_config = WINDOWS_DIR.clone()
    }

    let mods_dir = fs::create_dir_all(
        format!("{}{}{}{}mods", 
            &home_dir, os_config.seperator, os_config.minecraft_dir, os_config.seperator)
    );


    if mods_dir.is_err() {
        println!("Error making directory: {:?}\ndir: {:?}", 
            mods_dir.err(), 
            format!("{}{}{}{}mods", 
                &home_dir, os_config.seperator, os_config.minecraft_dir, os_config.seperator
            )
        );
        return Err("Error making mods directory".to_string())
    }

    for (key, value) in mods.iter() {
        
        let mod_result = MODS.get(key);
        
        if mod_result.is_none() {
            println!("Error with finding mod {}", key);
            continue;
        }
        
        let mod_info = mod_result.unwrap();
        
        if !value {
            let _ = fs::remove_file(
                format!("{}{}{}{}mods{}{}.jar",
                home_dir, os_config.seperator, os_config.minecraft_dir, os_config.seperator, 
                os_config.seperator, mod_info.slug)
            );
            continue;
        }

        let version_response = client
            .get(format!("https://api.modrinth.com/v2/project/{}/version", mod_info.slug))
            .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
            .send();

        let body = version_response.unwrap().text().unwrap();
        let version_object = json::parse(&body).unwrap();
        
        let mut file_index: usize = 0;

        loop {
            if version_object[file_index]["game_versions"].contains(json::JsonValue::from(version.clone())) &&
               version_object[file_index]["loaders"].contains(json::JsonValue::from("fabric")) {
                break;
            }
            file_index += 1;
        }

        let file_response = client
            .get(version_object[file_index]["files"][0]["url"].as_str().unwrap())
            .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
            .send();

        let mut file_data = std::io::Cursor::new(file_response.unwrap().bytes().unwrap());

        let file_result = fs::File::create(
            format!("{}{}{}{}mods{}{}.jar",
                home_dir, os_config.seperator, os_config.minecraft_dir, os_config.seperator, 
                os_config.seperator, mod_info.slug));

        let mut file = file_result.unwrap();
        let _ = std::io::copy(&mut file_data, &mut file);
    }
	
	return Ok(String::from("Operation completed"))
}



pub fn get_installed_mods(os: String) -> HashMap<String, bool> {
    let home_dir_option = dirs::home_dir().unwrap();
    let home_dir = home_dir_option.to_str().unwrap();
    let os_config: Directories;

    let mut hashmap: HashMap<String, bool> = HashMap::new();

    match os.as_str() {
        "Windows" => os_config = WINDOWS_DIR.clone(),
        "Linux" => os_config = LINUX_DIR.clone(),
        _ => os_config = WINDOWS_DIR.clone()
    };

    for (key, value) in MODS.entries.iter() {

        hashmap.insert(
            key.to_string(), 
            fs::metadata(
                format!("{}{}{}{}mods{}{}.jar",
                home_dir, os_config.seperator, os_config.minecraft_dir, os_config.seperator,
                os_config.seperator, value.slug)).is_ok()
            );

    }

    return hashmap
}

pub fn has_fabric_installed(os: String, version: String) -> Result<bool, String> {
    let home_dir_option = dirs::home_dir().unwrap();
    let home_dir = home_dir_option.to_str().unwrap();
    let os_config: Directories;

    match os.as_str() {
        "Windows" => os_config = WINDOWS_DIR.clone(),
        "Linux" => os_config = LINUX_DIR.clone(),
        _ => os_config = WINDOWS_DIR.clone()
    };

    let versions_result = fs::read_dir(
        format!("{}{}{}{}versions{}",
                home_dir, os_config.seperator, os_config.minecraft_dir, os_config.seperator,
                os_config.seperator));

    if versions_result.is_err() {
        return Err(format!("Error finding versions: {:?}", versions_result.err()));
    }
    let versions = versions_result.unwrap();


    let re = Regex::new(
        format!(r"fabric-loader-[0-9]*\.[0-9]*\.[0-9]*-{}", version).as_str()
    ).unwrap();

    for path in versions {
        let path_string = path.unwrap().file_name().into_string().unwrap();
        
        if re.is_match(&path_string) {
            return Ok(true);
        }
    }

    return Ok(false);
}