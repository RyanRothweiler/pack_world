use crate::{
    color::*,
    debug::*,
    error::*,
    json::*,
    math::*,
    matricies::matrix_four_four::*,
    rect::*,
    render::{accumulate_draw::*, image::*, material::*, render_command::*, shader::*, RenderApi},
    vectors::*,
};
use std::collections::HashMap;

const EM_SCALE: f64 = 10.0;
const KERNING_ADJ: f64 = 0.98;

pub fn load(
    image_read: impl std::io::Read,
    font_data: &str,
    shader: Shader,
    render_api: &mut impl RenderApi,
) -> Result<Font, Error> {
    let mut typeface: Font = Default::default();

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
    pub typeface: Font,
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

    pub fn get_word_height(&self, word: &str) -> f64 {
        let mut height_max: f64 = 0.0;

        for c in word.chars() {
            let glyph: &Glyph = self.typeface.glyphs.get(&c).unwrap();
            let acc_h = glyph.plane.height() * EM_SCALE * self.size;
            if acc_h > height_max {
                height_max = acc_h;
            }
            // if glyph

            // ret += glyph.advance * EM_SCALE * KERNING_ADJ * self.size;
        }

        height_max
    }
}

#[derive(Clone, Default)]
pub struct Font {
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
    let mut accum_draw = AccumulateDraw::new();

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

        accumulate_draw_word(word.into(), style, cursor, &mut accum_draw);
        cursor.x += word_width + space_width;
    }

    render_accumulate(accum_draw, style, color, render_commands);
}

pub fn render_word(
    word: String,
    style: &FontStyle,
    pos: VecTwo,
    color: Color,
    render_commands: &mut Vec<RenderCommand>,
) {
    let mut accum_draw = AccumulateDraw::new();
    accumulate_draw_word(word, style, pos, &mut accum_draw);
    render_accumulate(accum_draw, style, color, render_commands);
}

/// Add word to accumulate draw call
fn accumulate_draw_word(
    word: String,
    style: &FontStyle,
    pos: VecTwo,
    accum_draw: &mut AccumulateDraw,
) {
    let mut cursor = pos;

    for c in word.chars() {
        accumulate_draw_letter(c, style, cursor, accum_draw);

        let glyph: &Glyph = style.typeface.glyphs.get(&c).unwrap();
        cursor.x += glyph.advance * EM_SCALE * KERNING_ADJ * style.size;
    }
}

/// Add letters to accumulate draw call
fn accumulate_draw_letter(
    letter: char,
    style: &FontStyle,
    bottom_left: VecTwo,
    draw: &mut AccumulateDraw,
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

    let mesh = r.get_mesh(-1.0);
    let uvs: Vec<VecTwo> = vec![
        VecTwo::new(glyph.atlas.left(), glyph.atlas.top()),
        VecTwo::new(glyph.atlas.right(), glyph.atlas.top()),
        VecTwo::new(glyph.atlas.left(), glyph.atlas.bottom()),
        //
        VecTwo::new(glyph.atlas.left(), glyph.atlas.bottom()),
        VecTwo::new(glyph.atlas.right(), glyph.atlas.top()),
        VecTwo::new(glyph.atlas.right(), glyph.atlas.bottom()),
    ];

    for i in 0..6 {
        draw.add(uvs[i], mesh[i]);
    }
}

/// Render an accumulated draw call
fn render_accumulate(
    accum_draw: AccumulateDraw,
    style: &FontStyle,
    color: Color,
    render_commands: &mut Vec<RenderCommand>,
) {
    // Guess the correct pxRange
    let px_range = lerp(1.0, 24.0, style.size / 20.0);

    let mut uniforms = style.typeface.material.uniforms.clone();
    uniforms.insert("color".into(), UniformData::VecFour(color.into()));
    uniforms.insert("pxRange".into(), UniformData::Float(px_range));

    let rc = RenderCommand {
        kind: VertexDataKind::DynamicMesh {
            mesh: accum_draw.vertex,
            uvs: accum_draw.uv,
        },

        prog_id: style.typeface.material.shader.unwrap().prog_id,
        indices: accum_draw.indices,
        uniforms: uniforms,
    };
    render_commands.push(rc);
}
