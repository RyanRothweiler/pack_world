use crate::{
    error::*,
    json::*,
    render::{image::*, RenderApi},
};
use std::collections::HashMap;

pub fn load(
    image_read: impl std::io::Read,
    metrics_data: &str,
    render_api: &impl RenderApi,
) -> Result<Typeface, Error> {
    // load image
    // load font metrics

    let metrics_json = crate::json::load(metrics_data)?;
    let json_glyphs = metrics_json
        .get(vec!["glyphs".into()])
        .ok_or(Error::FontErrorLoading)?;

    let mut typeface = Typeface::new();

    match json_glyphs {
        crate::json::JsonData::Array(d) => {
            //
            // each glyph json
            for glyph_data in d {
                match glyph_data {
                    crate::json::JsonData::Class(glyph_class_json) => {
                        let char_uni = glyph_class_json
                            .get_float(vec!["unicode".into()])
                            .ok_or(Error::FontErrorLoading)?;

                        let mut glyph: Glyph = Default::default();

                        glyph.advance = glyph_class_json
                            .get_float(vec!["advance".into()])
                            .ok_or(Error::FontErrorLoading)?;

                        // pretty dangerous. will be weird with invalid data
                        let c: char =
                            char::from_u32(char_uni as u32).ok_or(Error::FontErrorLoading)?;

                        match glyph_class_json.get_class(vec!["planeBounds".into()]) {
                            Some(bounds_json) => {
                                glyph.atlas_left = bounds_json
                                    .get_float(vec!["left".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.atlas_right = bounds_json
                                    .get_float(vec!["right".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.atlas_top = bounds_json
                                    .get_float(vec!["top".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.atlas_bottom = bounds_json
                                    .get_float(vec!["bottom".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                            }
                            None => (),
                        };

                        typeface.glyphs.insert(c, glyph);
                    }

                    _ => return Err(Error::FontErrorLoading),
                }
            }
            //
        }
        _ => return Err(Error::FontErrorLoading),
    }

    typeface.atlas = crate::render::load_image(image_read).unwrap();
    typeface.atlas_id = render_api.upload_texture(&typeface.atlas, false).unwrap();

    return Ok(typeface);
}

pub struct Typeface {
    pub glyphs: HashMap<char, Glyph>,
    pub atlas: Image,
    pub atlas_id: u32,
}

impl Typeface {
    pub fn new() -> Self {
        Typeface {
            glyphs: HashMap::new(),
            atlas: Image::new(),
            atlas_id: 0,
        }
    }
}

#[derive(Default, Debug)]
pub struct Glyph {
    pub advance: f64,
    pub atlas_left: f64,
    pub atlas_right: f64,
    pub atlas_top: f64,
    pub atlas_bottom: f64,
}
