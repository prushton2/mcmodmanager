use reqwest;
use phf::{phf_map};
use std::collections::HashMap;

pub struct ModInfo<'a> {
    slug: &'a str,
    dependencies: Vec<&'a str>
}

pub static MODS: phf::Map<&str, ModInfo> = phf_map! {
    "Sodium"      => ModInfo {slug: "sodium",            dependencies: vec![]},
    "Iris"        => ModInfo {slug: "iris",              dependencies: vec![]},
    "Carpet"      => ModInfo {slug: "carpet",            dependencies: vec![]},
    "Audioplayer" => ModInfo {slug: "audioplayer",       dependencies: vec!["Voice Chat"]},
    "Voice Chat"  => ModInfo {slug: "simple-voice-chat", dependencies: vec![]},
    // "MiniHud"     => ModInfo {slug: "minihud",           dependencies: vec![]},
    // "Tweakeroo"   => ModInfo {slug: "tweakeroo",         dependencies: vec![]},
    
    "FabricAPI"   => ModInfo {slug: "fabric-api",    dependencies: vec![]},
    "MaLiLib"     => ModInfo {slug: "malilib",       dependencies: vec![]},
};


pub async fn download(mods: HashMap<String, bool>) -> Result<String, String> {
	println!("Starting download");
    

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

        let response = reqwest::get("https://example.com").await;

        // let response = reqwest::get(
        //     format!("https:://api.modrinth.com/v2/project/{}/version",  info.slug)).await;//.await?.text().await?;
    

        println!("{}: {:?}", info.slug, response);
    }

	
	println!("Download done");


	// if response.is_err() {
	// 	return Err(String::from("Bad Request"));
	// }
	
	// // let body = response?.text().await?;

	// println!("body: {:?}", response);

	return Ok(String::from("returned from fn"))
}