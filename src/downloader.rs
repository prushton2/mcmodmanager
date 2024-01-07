use reqwest;

pub struct Downloadable {
	name: &str,
	slug: &str //slug is a named id of a mod in modrinth
}

let pub downloadables: Vector<Downloadable> = vec![
	Downloadable {
		name: "Sodium",
		slug: "sodium"
	}
]


pub fn download(downloadable: Downloadable) -> Result<&str, &str> {
	let response = reqwest.get(format!("https:;//api.modrinth.com/v2/project/{}/version", downloadable.slug))
		.await?
		.text()
		.await?

	println("body: {:?}", response);
}