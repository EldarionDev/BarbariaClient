pub struct ResourceManager {
    asset_path: String,
    config_path: String,
}

impl ResourceManager {
    pub fn new(asset_path: &str, config_path: &str) -> Self {
        ResourceManager {
            asset_path: asset_path.to_owned(),
            config_path: config_path.to_owned(),
        }
    }

    pub fn set_world(world_path: &str) {

    }
}