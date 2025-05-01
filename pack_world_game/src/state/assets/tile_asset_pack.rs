use gengar_engine::{
    model::*,
    render::{image::*, material::*},
};

pub struct TileAssetPack {
    pub model: Model,
    pub material: Material,

    pub tex_albedo: Image,
    pub tex_ao: Image,
    pub tex_roughness: Image,
    pub tex_metallic: Image,
    pub tex_normal: Image,
}

/*
impl TileAssetPack {
    pub fn load() -> Self {
        let mut pack = Self {
            model: Model::new(),
            material: Material::new(),

            tex_albedo: Image::new(),
            tex_ao: Image::new(),
            tex_roughness: Image::new(),
            tex_metallic: Image::new(),
            tex_normal: Image::new(),
        };

        gs.assets.tile_grass_albedo = load_image_cursor(
            include_bytes!("../resources/models/tile_grass/BaseColor.png"),
            render_api,
        )
        .unwrap();

        gs.assets.tile_grass_ao = load_image_cursor(
            include_bytes!("../resources/models/tile_grass/AO.png"),
            render_api,
        )
        .unwrap();

        gs.assets.tile_grass_normal = load_image_cursor(
            include_bytes!("../resources/models/tile_grass/Normal.png"),
            render_api,
        )
        .unwrap();

        gs.assets.tile_grass_roughness = load_image_cursor(
            include_bytes!("../resources/models/tile_grass/Roughness.png"),
            render_api,
        )
        .unwrap();

        gs.assets.tile_grass_metallic = load_image_cursor(
            include_bytes!("../resources/models/tile_grass/Metallic.png"),
            render_api,
        )
        .unwrap();

        // grass material
        gs.assets.tile_grass_material.shader = Some(es.pbr_shader);
        gs.assets.tile_grass_material.uniforms.insert(
            "tex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: gs.assets.tile_grass_albedo.gl_id.unwrap(),
                texture_slot: 0,
            }),
        );

        gs.assets.tile_grass_material.uniforms.insert(
            "normalTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: gs.assets.tile_grass_normal.gl_id.unwrap(),
                texture_slot: 1,
            }),
        );
        gs.assets.tile_grass_material.uniforms.insert(
            "metallicTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: gs.assets.tile_grass_metallic.gl_id.unwrap(),
                texture_slot: 2,
            }),
        );
        gs.assets.tile_grass_material.uniforms.insert(
            "roughnessTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: gs.assets.tile_grass_roughness.gl_id.unwrap(),
                texture_slot: 3,
            }),
        );
        gs.assets.tile_grass_material.uniforms.insert(
            "aoTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: gs.assets.tile_grass_ao.gl_id.unwrap(),
                texture_slot: 4,
            }),
        );

        return pack;
    }
}
*/
