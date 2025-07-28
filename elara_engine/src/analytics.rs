#[derive(Debug, Clone)]
pub enum AnalyticsEvent {
    AppStart,
    PackOpen(String),
}

impl AnalyticsEvent {
    pub fn to_id(&self) -> String {
        match self {
            Self::AppStart => "AppStart".into(),
            Self::PackOpen(s) => format!("PackOpen({})", s),
        }
    }
}
