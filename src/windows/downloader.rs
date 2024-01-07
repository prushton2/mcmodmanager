use reqwest;

// #[derive(Clone, Copy, Debug)]
pub struct Downloadable<'a> {
	name: &'a str,
	slug: &'a str //slug is a named id of a mod in modrinth
}

pub const downloadables: [Downloadable; 1]  = [
	Downloadable {
		name: "Sodium",
		slug: "sodium"
	}
];


pub async fn download(downloadable: &Downloadable<'_>) -> Result<String, String> {
	// let response = reqwest::get(format!("https:://api.modrinth.com/v2/project/{}/version", downloadable.slug));
	
	// if response.is_err {
	// 	return Err(String::from("Bad Request"));
	// }
	
	// let body = response.await?.text().await?;

	// println!("body: {:?}", body);

	Ok(String::from("Hi"))
}