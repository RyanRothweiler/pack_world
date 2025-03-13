#[derive(Debug, Clone, Copy)]
pub enum AnalyticsEvent {
    AppStart,
}

impl AnalyticsEvent {
    pub fn to_id(&self) -> String {
        match self {
            Self::AppStart => "AppStart".into(),
        }
    }
}
