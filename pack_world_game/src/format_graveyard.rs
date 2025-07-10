// Things which are too large for rustfmt to handle go here.
// Really it should only be constants.

use std::sync::LazyLock;

pub const OFFLINE_PROGRESS_DESC: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Increase offline progress from {} hour to {} hours. More features coming in the future!",
        crate::save_file::SIM_LIMIT_H_FREE,
        crate::save_file::SIM_LIMIT_H_PREMIUM
    )
});
