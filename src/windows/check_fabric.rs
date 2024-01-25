pub fn window<'a>(_this: &'a ModLoader, has_fabric: Result<bool, String>) -> iced::Element<'a, Message> {

    let fabric_found = column![
        text("Fabric was found on your system.\n\n"),
        button("Install Fabric Anyway").on_press(Message::ChangePage(1))
    ];

    let fabric_not_found = column![
        text("Fabric was not found on your system"),
        // button("Install Fabric").on_press(Message::ChangePage(1))
    ];
    // fabric_install_result;
    if has_fabric.is_err() {
        return column![
            text(format!("Error locating fabric: {:?}", has_fabric.err()))
        ].into();
    }

    if has_fabric.clone().unwrap() {
        return fabric_found.into();
    }

    return fabric_not_found.into();
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