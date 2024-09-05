#[allow(unused)]
pub const TOKEN: &str = "fs_rs_rwa_token";

#[derive(Debug)]
pub struct AppState {
    pub token: Option<String>,
}

impl AppState {
    #[allow(unused)]
    pub fn new() -> Self {
        Self { token: None }
    }
}
