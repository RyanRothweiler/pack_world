#[derive(Eq, PartialEq)]
pub enum BuildType {
    Release,
    Development,
}

pub fn build_type() -> BuildType {
    if let Some(env) = option_env!("BUILD_TYPE") {
        if env == "Release" {
            return BuildType::Release;
        }
    }

    return BuildType::Development;
}

pub fn build_type_development() -> bool {
    build_type() == BuildType::Development
}
