use reqwest;
use json;
use std::fs;
use dirs;
use regex::Regex;
use std::env;

pub static FABRIC_URL: &str = "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.0.0/fabric-installer-1.0.0.jar";

#[derive(Clone)]
pub struct Directories<'a> {
    pub seperator: char,
    pub minecraft_dir: &'a str,
    pub home_dir: String
}


async fn download_file<'a>(url: &'a str, path: &str) -> Result<&'a str, &'a str> {
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

pub async fn download<'a>(version: String, mods: Vec<String>) -> Result<&'a str, &'a str> {
    let client = reqwest::blocking::Client::new();
    
    let config_result = get_os_config();

    if config_result.is_err() {
        return Err("Error getting config");
    }

    let config: Directories = config_result.unwrap();

    let mods_dir = fs::create_dir_all(
        format!("{}{}{}{}mods", 
            config.home_dir, config.seperator, config.minecraft_dir, config.seperator)
    );

    if mods_dir.is_err() {
        return Err("Error making mods directory")
    }

    let mods_dir_result = fs::read_dir(
        format!("{}{}{}{}mods", config.home_dir, config.seperator,
        config.minecraft_dir, config.seperator)
    );

    if mods_dir_result.is_err() {
        return Err("Error deleting existing mods");
    }

    let mods_dir = mods_dir_result.unwrap();

    for mod_path in mods_dir {
        let mod_name_os_string = mod_path.unwrap().file_name();
        let file = mod_name_os_string.into_string().unwrap();
        
        let file_name = &file[..file.len()-4];
        let extension = &file[file.len()-8..file.len()-4];
        let mod_name = &file[..file.len()-8];

        if extension != "-mcm" {
            continue;
        }
        
        if mods.iter().any(|i| i == &mod_name ) {
            continue;
        }

        let res = fs::remove_file(
            format!("{}{}{}{}mods{}{}",
                config.home_dir, config.seperator, config.minecraft_dir, config.seperator,
                config.seperator, file)
        );
        if res.is_ok() {
            println!("Removed {}", file);
        }        
    }


    for slug in mods.iter() {

        //get the right file by looking for a file with the correct version and mod loader
        let version_response = client
            .get(format!("https://api.modrinth.com/v2/project/{}/version", slug))
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
        
        let file_path = format!("{}{}{}{}mods{}{}-mcm.jar",
            config.home_dir, config.seperator, config.minecraft_dir, config.seperator, 
            config.seperator, slug);

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

pub fn get_installed_mods() -> Result<Vec<String>, &'static str> {
    let config_result = get_os_config();

    if config_result.is_err() {
        return Err("Error getting config");
    }

    let config: Directories = config_result.unwrap();

    let mut mods: Vec<String> = vec![];

    let mods_result = fs::read_dir(
        format!("{}{}{}{}mods{}",
                config.home_dir, config.seperator, config.minecraft_dir, config.seperator,
                config.seperator));

    if mods_result.is_err() {
        return Err("Error finding versions: {:?}");
    }
 
    let mods_dir = mods_result.unwrap();

    for mod_name in mods_dir {
        if mod_name.is_err() {
            continue;
        }

        let mut file_name = mod_name.unwrap().file_name().into_string().unwrap();
        
        //get rid of the .jar at the end of the file name
        file_name.pop();
        file_name.pop();
        file_name.pop();
        file_name.pop();

        let mut mcm_extension = String::from("");

        //get rid of the -mcm at the end of the file name
        mcm_extension.push(file_name.pop().unwrap()); 
        mcm_extension.push(file_name.pop().unwrap());
        mcm_extension.push(file_name.pop().unwrap());
        mcm_extension.push(file_name.pop().unwrap());

        if mcm_extension == "mcm-" {
            mods.push(file_name);
        }
    }
    
    return Ok(mods);
}

pub fn has_fabric_installed(version: &String) -> Result<bool, String> {
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

pub async fn download_fabric<'a>() -> Result<&'a str, &'a str> {
    let config_result = get_os_config();

    if config_result.is_err() {
        return Err("Error getting config");
    }

    let config: Directories = config_result.unwrap();

    let fabric_path = format!("{}/{}/fabric-installer.jar", 
        config.home_dir, config.minecraft_dir);

    let downloaded = download_file(&FABRIC_URL, &fabric_path.as_str()).await;
    return downloaded;
}

pub async fn search_modrinth<'a>(query: String) -> Result<Vec<String>, &'a str> {
    let client = reqwest::blocking::Client::new();

    let search_response = client
        .get(format!("https://api.modrinth.com/v2/search?query={}", query))
        .header(reqwest::header::USER_AGENT, "github/prushton2/mcmodmanager")
        .send();
    
    if search_response.is_err() {
        return Err("Error making request");
    }

    let body = search_response.unwrap().text().unwrap();
    let search_object = json::parse(&body).unwrap();

    let mut slugs: Vec<String> = vec![];
    let mut index: usize = 0;
    
    loop {
        let mut string_value = json::stringify(search_object["hits"][index]["slug"].clone());
        
        if string_value == "null" {
            break;
        }

        string_value.remove(0);
        string_value.pop();

        slugs.push(string_value.clone());
        index += 1;
    }
    
    return Ok(slugs);
}
