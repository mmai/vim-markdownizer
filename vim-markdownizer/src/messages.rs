pub enum Messages {
    InitContentWindow,
    ProjectList,
    ProjectSelect,
    Dashboard,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "init_content_window" => Messages::InitContentWindow,
            "project_list" => Messages::ProjectList,
            "project_select" => Messages::ProjectSelect,
            "dashboard" => Messages::Dashboard,
            _ => Messages::Unknown(event),
        }
    }
}
