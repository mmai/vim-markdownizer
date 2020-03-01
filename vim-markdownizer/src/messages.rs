pub enum Messages {
    ProjectList,
    Dashboard,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "project_list" => Messages::ProjectList,
            "dashboard" => Messages::Dashboard,
            _ => Messages::Unknown(event),
        }
    }
}
