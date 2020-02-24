pub enum Messages {
    ProjectList,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "project_list" => Messages::ProjectList,
            _ => Messages::Unknown(event),
        }
    }
}
