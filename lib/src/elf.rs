#[derive(Clone, serde::Serialize)]
pub struct ELFPayload {
    pub loaded: bool,
    pub error: String,
    pub filename: String
}

impl Default for ELFPayload {
    fn default() -> Self {
        ELFPayload {
            loaded: false,
            error: String::from(""),
            filename: String::from("")
        }
    }
}