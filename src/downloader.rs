use reqwest;
use phf::{phf_map};
use std::collections::HashMap;
use json;
use std::fs;
use dirs;
use regex::Regex;
use std::env;

pub struct ModInfo<'a> {
    slug: &'a str,
    dependencies: Vec<&'a str>, //[String; 8]
}

#[derive(Clone)]
pub struct Directories<'a> {
    pub seperator: char,
    pub minecraft_dir: &'a str,
    pub home_dir: String
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
    "FabricAPI"   => ModInfo {slug: "fabric-api",    dependencies: vec![]},
    "MaLiLib"     => ModInfo {slug: "malilib",       dependencies: vec![]},
    // please put these on modrinth please please please
    // "MiniHud"     => ModInfo {slug: "minihud",           dependencies: vec![]},
    // "Tweakeroo"   => ModInfo {slug: "tweakeroo",         dependencies: vec![]},
};

// pub static WINDOWS_DIR: Directories = Directories {
//     seperator: '\\',
//     minecraft_dir: "AppData\\Roaming\\.minecraft"
// };

// pub static LINUX_DIR: Directories = Directories {
//     seperator: '/',
//     minecraft_dir: ".minecraft"
// };

pub static FABRIC_URL: &str = "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.0.0/fabric-installer-1.0.0.jar";

async fn download_file(url: &str, path: &str) -> Result<&'static str, &'static str> {
    let client = reqwest::blocking::Client::new();
    let file_response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
        .send();

    let mut file_data = std::io::Cursor::new(file_response.unwrap().bytes().unwrap());

    let file_result = fs::File::create(path);

    let mut file = file_result.unwrap();
    let resp = std::io::copy(&mut file_data, &mut file);

    if resp.is_err() {
        return Err("Error copying data to file");
    }

    return Ok("File downloaded");
}

pub fn get_os_config() -> Result<Directories<'static>, String> {
    let config: Directories;
    
    let home_dir_option = dirs::home_dir();
    
    if home_dir_option.is_none() {
        return Err(String::from("Could not find home directory"));
    }
    
    let home_dir: String = home_dir_option.unwrap().to_str().unwrap().to_string();

    match env::consts::OS {
        "windows" => config = Directories {
            seperator: '\\',
            minecraft_dir: "AppData\\Roaming\\.minecraft",
            home_dir: home_dir,
        },
        "linux" => config = Directories {
            seperator: '/',
            minecraft_dir: ".minecraft",
            home_dir: home_dir,
        },
        _ => {
            return Err(String::from("Invalid OS"));
        }
    }

    return Ok(config.clone());
}

pub async fn download(version: String, mods: HashMap<String, bool>) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    
    let config_result = get_os_config();

    if config_result.is_err() {
        return Err(String::from("Error getting config"));
    }

    let config: Directories = config_result.unwrap();

    let mods_dir = fs::create_dir_all(
        format!("{}{}{}{}mods", 
            config.home_dir, config.seperator, config.minecraft_dir, config.seperator)
    );


    if mods_dir.is_err() {
        println!("Error making directory: {:?}\ndir: {:?}", 
            mods_dir.err(), 
            format!("{}{}{}{}mods", 
                config.home_dir, config.seperator, config.minecraft_dir, config.seperator
            )
        );
        return Err("Error making mods directory".to_string())
    }

    for (key, value) in mods.iter() {
        
        //Get the mod from the static hashmap
        let mod_result = MODS.get(key);
        
        if mod_result.is_none() { //this shouldnt happen but just in case
            println!("Error with finding mod {}", key);
            continue;
        }
        
        let mod_info = mod_result.unwrap();
        
        //value is the checked, so remove it if its unchecked
        if !value {
            let _ = fs::remove_file(
                format!("{}{}{}{}mods{}{}.jar",
                config.home_dir, config.seperator, config.minecraft_dir, config.seperator, 
                config.seperator, mod_info.slug)
            );
            continue;
        }

        //get the right file by looking for a file with the correct version and mod loader
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
        
        let file_path = format!("{}{}{}{}mods{}{}.jar",
            config.home_dir, config.seperator, config.minecraft_dir, config.seperator, 
            config.seperator, mod_info.slug);

        let result = download_file(
                version_object[file_index]["files"][0]["url"].as_str().unwrap(),
                file_path.as_str()
            ).await;


        if result.is_err() {
            return Err(format!("Error downloading {}: {:?}", mod_info.slug, result.err()));
        }
    }
	
	return Ok(String::from("Operation completed"));
}



pub fn get_installed_mods() -> HashMap<String, bool> {
    let config_result = get_os_config();

    if config_result.is_err() {
        println!("Error getting config");
    }

    let config: Directories = config_result.unwrap();

    let mut hashmap: HashMap<String, bool> = HashMap::new();

    for (key, value) in MODS.entries.iter() {

        hashmap.insert(
            key.to_string(), 
            fs::metadata(
                format!("{}{}{}{}mods{}{}.jar",
                config.home_dir, config.seperator, config.minecraft_dir, config.seperator,
                config.seperator, value.slug)).is_ok()
            );

    }

    return hashmap
}

pub fn has_fabric_installed(version: String) -> Result<bool, String> {
    let config_result = get_os_config();

    if config_result.is_err() {
        return Err(String::from("Error getting config"));
    }

    let config: Directories = config_result.unwrap();


    let versions_result = fs::read_dir(
        format!("{}{}{}{}versions{}",
                config.home_dir, config.seperator, config.minecraft_dir, config.seperator,
                config.seperator));

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

pub async fn download_fabric() -> Result<&'static str, &'static str> {
    let config_result = get_os_config();

    if config_result.is_err() {
        return Err("Error getting config");
    }

    let config: Directories = config_result.unwrap();


    let fabric_path = format!("{}/{}/fabric-installer.jar", 
        config.home_dir, config.minecraft_dir);

    let downloaded = download_file(&FABRIC_URL, fabric_path.as_str()).await;
    return downloaded;
}