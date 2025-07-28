use std::collections::HashMap;

/// Allows including data in binary and then fetching it using that file path.
/// Useful for platforms which don't have easy or any file systems.
/// Somewhat allows using the standard file api to load files locally included in the binary.
pub struct BinaryFileSystem {
    pub files: HashMap<String, Vec<u8>>,
}

impl BinaryFileSystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

#[macro_export]
macro_rules! bin_include {
    ($fs:expr, $x:expr) => {
        $fs.files.insert($x.into(), include_bytes!($x).into());
    };
}

fn build_asset_files(fs: &mut BinaryFileSystem) {
    bin_include!(fs, "../engine_resources/shaders/pbr.vs");
}
