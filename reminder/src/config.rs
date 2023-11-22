#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) db_uri: String,
    pub(crate) db_user: String,
    pub(crate) db_pass: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_uri: "ws://localhost:8000".to_string(),
            db_user: "root".to_string(),
            db_pass: "root".to_string(),
        }
    }
}
