use crate::{error::*, json::*};
use std::{collections::HashMap, path::Path};

/*
pub fn load_typeface(_image_path: &Path, metrics_path: &Path) -> Result<Typeface, Error> {
    // load image
    // load font metrics

    let metrics_json = crate::json::load_file(metrics_path)?;

    let mut typeface = Typeface {
        glyphs: HashMap::new(),
    };


    glyphs.insert

    return Ok(typeface);
}
*/

pub struct Typeface {
    pub glyphs: HashMap<char, Glyph>,
}

pub struct Glyph {
    pub advance: f64,
    pub atlas_left: f64,
    pub atlas_right: f64,
    pub atlas_top: f64,
    pub atlas_bottom: f64,
}
