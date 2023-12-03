#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) grpc_port: u16,
    pub(crate) db_uri: String,
    pub(crate) db_user: String,
    pub(crate) db_pass: String,
    pub(crate) notification_cache_interval: u8,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            grpc_port: 58946,
            db_uri: "ws://localhost:8000".to_string(),
            db_user: "root".to_string(),
            db_pass: "root".to_string(),
            notification_cache_interval: 10,
        }
    }
}
