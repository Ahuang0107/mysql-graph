#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "mysql-url")]
    mysql_url: String,
}

#[allow(dead_code)]
impl Config {
    pub fn from_yaml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_yaml::from_str::<Self>(&content)?;
        Ok(config)
    }
}
