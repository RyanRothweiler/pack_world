use std::sync::LazyLock;

/// no item description
pub const NO_ITEM_DESC: &str = "No item description";

/// exponential increase for cost of bank slot
pub const BANK_LIMIT_EXPO_PRICE: f64 = 2.5;

/// base starting cost of first slot
pub const BANK_LIMIT_COST_BASE: i64 = 50;

/// starting bank slots count
pub const BANK_LIMIT_START: usize = 15;

// Offline progress description
pub const OFFLINE_PROGRESS_DESC: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Increase offline progress from {} hour to {} hours. More features to come as development continues.",
        crate::save_file::SIM_LIMIT_H_FREE,
        crate::save_file::SIM_LIMIT_H_PREMIUM
    )
});
