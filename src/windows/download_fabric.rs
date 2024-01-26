use iced::widget::{text};

use crate::ui::{ModLoader, Message, download_file};

pub static FABRIC_URL: &str = "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.0.0/fabric-installer-1.0.0.jar";

pub fn window(_this: &ModLoader) -> iced::Element<'_, Message> {
    //we only get here if fabric is not found
    return text("Downloading and launching Fabric...").into();
}


pub async fn download_fabric<'a>(this: ModLoader) -> Result<&'a str, &'a str> {
    let fabric_path = format!("{}/{}/fabric-installer.jar",
                              this.home_dir, this.minecraft_dir);

    let downloaded = download_file(&FABRIC_URL, &fabric_path.as_str()).await;
    return downloaded;
}
