use crate::{
    color::*,
    debug::*,
    error::*,
    json::*,
    matricies::matrix_four_four::*,
    rect::*,
    render::{image::*, material::*, render_command::*, shader::*, RenderApi},
    vectors::*,
};
use std::collections::HashMap;

const EM_SCALE: f64 = 10.0;
const KERNING_ADJ: f64 = 0.98;

pub fn load(
    image_read: impl std::io::Read,
    font_data: &str,
    shader: Shader,
    render_api: &impl RenderApi,
) -> Result<Typeface, Error> {
    let mut typeface: Typeface = Default::default();

    // load image
    typeface.atlas = crate::render::load_image(image_read).unwrap();
    typeface.atlas_id = render_api.upload_texture(&typeface.atlas, false).unwrap();

    // create material for rendering
    typeface.material.shader = Some(shader);
    typeface.material.set_color(Color::new(1.0, 1.0, 1.0, 1.0));

    typeface.material.uniforms.insert(
        "tex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: typeface.atlas_id,
            texture_slot: 0,
        }),
    );

    // parse json
    let font_json = crate::json::load(font_data)?;

    // load font metrics
    let metrics_json = font_json
        .get(vec!["metrics".into()])
        .ok_or(Error::FontErrorLoading)?;
    match metrics_json {
        crate::json::JsonData::Class(d) => {
            typeface.line_height = d
                .get_float(vec!["lineHeight".into()].into())
                .ok_or(Error::FontErrorLoading)?;
        }

        _ => return Err(Error::FontErrorLoading),
    };

    // load font glyphs
    let json_glyphs = font_json
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
                                glyph.atlas.top_left.x = bounds_json
                                    .get_float(vec!["left".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.atlas.bottom_right.x = bounds_json
                                    .get_float(vec!["right".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.atlas.top_left.y = bounds_json
                                    .get_float(vec!["top".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.atlas.bottom_right.y = bounds_json
                                    .get_float(vec!["bottom".into()])
                                    .ok_or(Error::FontErrorLoading)?;

                                glyph.atlas.top_left.x =
                                    glyph.atlas.top_left.x / typeface.atlas.width as f64;
                                glyph.atlas.bottom_right.x =
                                    glyph.atlas.bottom_right.x / typeface.atlas.width as f64;
                                glyph.atlas.top_left.y =
                                    1.0 - (glyph.atlas.top_left.y / typeface.atlas.height as f64);
                                glyph.atlas.bottom_right.y = 1.0
                                    - (glyph.atlas.bottom_right.y / typeface.atlas.height as f64);
                            }
                            None => (),
                        };

                        match glyph_class_json.get_class(vec!["planeBounds".into()]) {
                            Some(bounds_json) => {
                                glyph.plane.top_left.x = bounds_json
                                    .get_float(vec!["left".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.plane.bottom_right.x = bounds_json
                                    .get_float(vec!["right".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.plane.top_left.y = bounds_json
                                    .get_float(vec!["top".into()])
                                    .ok_or(Error::FontErrorLoading)?;
                                glyph.plane.bottom_right.y = bounds_json
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

    return Ok(typeface);
}

#[derive(Default, Clone)]
pub struct FontStyle {
    pub size: f64,
    pub typeface: Typeface,
}

impl FontStyle {
    pub fn get_word_width(&self, word: &str) -> f64 {
        let mut ret: f64 = 0.0;
        for c in word.chars() {
            let glyph: &Glyph = self.typeface.glyphs.get(&c).unwrap();
            ret += glyph.advance * EM_SCALE * KERNING_ADJ * self.size;
        }

        ret
    }
}

#[derive(Clone, Default)]
pub struct Typeface {
    pub glyphs: HashMap<char, Glyph>,
    pub atlas: Image,
    pub atlas_id: u32,
    pub material: Material,
    pub line_height: f64,
}

#[derive(Default, Debug, Clone)]
pub struct Glyph {
    pub advance: f64,
    pub atlas: Rect,
    pub plane: Rect,
}

pub fn render_paragraph(
    paragraph: String,
    rect: Rect,
    style: &FontStyle,
    color: Color,
    render_commands: &mut Vec<RenderCommand>,
) {
    let line_height = style.typeface.line_height * EM_SCALE * style.size;
    let space_width = style.get_word_width(" ");

    let mut cursor = rect.top_left;
    cursor.y += line_height;

    for word in paragraph.split(' ') {
        let word_width = style.get_word_width(word);
        if cursor.x + word_width >= rect.bottom_right.x {
            cursor.x = rect.top_left.x;
            cursor.y += line_height;
        }

        render_word(word.into(), style, cursor, color, render_commands);
        cursor.x += word_width + space_width;
    }
}

pub fn render_word(
    word: String,
    style: &FontStyle,
    pos: VecTwo,
    color: Color,
    render_commands: &mut Vec<RenderCommand>,
) {
    let mut cursor = pos;
    for c in word.chars() {
        render_letter(c, style, cursor, color, render_commands);

        let glyph: &Glyph = style.typeface.glyphs.get(&c).unwrap();
        cursor.x += glyph.advance * EM_SCALE * KERNING_ADJ * style.size;
    }
}

pub fn render_letter(
    letter: char,
    style: &FontStyle,
    bottom_left: VecTwo,
    color: Color,
    render_commands: &mut Vec<RenderCommand>,
) {
    let glyph: &Glyph = style.typeface.glyphs.get(&letter).unwrap();

    let mut r = glyph.plane.clone() * EM_SCALE * style.size;

    // Filp vertically because top left is 0
    // Fonts assume bottom left is 0
    r.top_left.y *= -1.0;
    r.bottom_right.y *= -1.0;

    r.top_left.x += bottom_left.x;
    r.top_left.y += bottom_left.y;
    r.bottom_right.x += bottom_left.x;
    r.bottom_right.y += bottom_left.y;

    let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
    let uvs: Vec<VecTwo> = vec![
        VecTwo::new(glyph.atlas.left(), glyph.atlas.top()),
        VecTwo::new(glyph.atlas.right(), glyph.atlas.top()),
        VecTwo::new(glyph.atlas.left(), glyph.atlas.bottom()),
        //
        VecTwo::new(glyph.atlas.left(), glyph.atlas.bottom()),
        VecTwo::new(glyph.atlas.right(), glyph.atlas.top()),
        VecTwo::new(glyph.atlas.right(), glyph.atlas.bottom()),
    ];

    let mut uniforms = style.typeface.material.uniforms.clone();
    uniforms.insert("color".into(), UniformData::VecFour(color.into()));

    let rc = RenderCommand {
        kind: VertexDataKind::DynamicMesh {
            mesh: r.get_mesh(-1.0),
            uvs: uvs,
        },

        prog_id: style.typeface.material.shader.unwrap().prog_id,
        indices: indices,
        uniforms: uniforms,
    };
    render_commands.push(rc);
}
