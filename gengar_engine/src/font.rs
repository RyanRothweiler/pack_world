use crate::{
    color::*,
    error::*,
    json::*,
    matricies::matrix_four_four::*,
    rect::*,
    render::{image::*, material::*, render_command::*, shader::*, RenderApi},
    vectors::*,
};
use std::collections::HashMap;

pub fn load(
    image_read: impl std::io::Read,
    metrics_data: &str,
    shader: Shader,
    render_api: &impl RenderApi,
) -> Result<Typeface, Error> {
    let mut typeface = Typeface::new();

    // load image
    typeface.atlas = crate::render::load_image(image_read).unwrap();
    typeface.atlas_id = render_api.upload_texture(&typeface.atlas, false).unwrap();

    // create material for rendering
    typeface.material.shader = Some(shader);
    typeface.material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(Color::new(1.0, 1.0, 1.0, 1.0).into()),
    );

    typeface.material.uniforms.insert(
        "tex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: typeface.atlas_id,
            texture_slot: 0,
        }),
    );

    // load font metrics
    let metrics_json = crate::json::load(metrics_data)?;
    let json_glyphs = metrics_json
        .get(vec!["glyphs".into()])
        .ok_or(Error::FontErrorLoading)?;

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

                        match glyph_class_json.get_class(vec!["atlasBounds".into()]) {
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

                                glyph.atlas_left = glyph.atlas_left / typeface.atlas.width as f64;
                                glyph.atlas_right = glyph.atlas_right / typeface.atlas.width as f64;
                                glyph.atlas_top =
                                    1.0 - (glyph.atlas_top / typeface.atlas.height as f64);
                                glyph.atlas_bottom =
                                    1.0 - (glyph.atlas_bottom / typeface.atlas.height as f64);
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

    return Ok(typeface);
}

pub struct Typeface {
    pub glyphs: HashMap<char, Glyph>,
    pub atlas: Image,
    pub atlas_id: u32,
    pub material: Material,
}

impl Typeface {
    pub fn new() -> Self {
        Typeface {
            glyphs: HashMap::new(),
            atlas: Image::new(),
            atlas_id: 0,
            material: Material::new(),
        }
    }

    pub fn render(&self, pos: VecTwo, render_commands: &mut Vec<RenderCommand>) {
        let mut r = Rect::new_square(100.0);

        r.set_center(pos);

        let glyph: &Glyph = self.glyphs.get(&'A').unwrap();

        let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
        let uvs: Vec<VecTwo> = vec![
            VecTwo::new(glyph.atlas_left, glyph.atlas_top),
            VecTwo::new(glyph.atlas_right, glyph.atlas_top),
            VecTwo::new(glyph.atlas_left, glyph.atlas_bottom),
            //
            VecTwo::new(glyph.atlas_left, glyph.atlas_bottom),
            VecTwo::new(glyph.atlas_right, glyph.atlas_top),
            VecTwo::new(glyph.atlas_right, glyph.atlas_bottom),
        ];

        let rc = RenderCommand {
            kind: VertexDataKind::DynamicMesh {
                mesh: r.get_mesh(-1.0),
                uvs: uvs,
            },

            prog_id: self.material.shader.unwrap().prog_id,
            indices: indices,
            uniforms: self.material.uniforms.clone(),
        };
        render_commands.push(rc);
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
