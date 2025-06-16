use crate::{drop_table::*, item::*, pack::*, state::inventory::*, tile::*};
use gengar_engine::{
    binary_file_system::*,
    color::*,
    model::*,
    render::{
        camera::*, frame_buffer_pack::*, image::*, light::*, material::*, render_pack::*,
        shader::*, RenderApi,
    },
    state::components::*,
    transform::*,
    vectors::*,
};
use std::collections::HashMap;

pub mod asset_library;

pub use asset_library::*;

pub struct Assets {
    pub image_dirt_clod: Image,
    pub image_stick: Image,
    pub image_rock: Image,
    pub image_oak_wood: Image,
    pub image_gold: Image,
    pub image_acorn: Image,
    pub image_dragon_egg: Image,
    pub image_baby: Image,
    pub image_berry: Image,
    pub image_question_mark: Image,
    pub image_mud_baby: Image,
    pub image_pearl: Image,
    pub image_old_boot: Image,
    pub image_seaweed: Image,
    pub image_trash_bag: Image,
    pub image_old_hat: Image,
    pub image_dew: Image,
    pub image_glow: Image,
    pub image_twitter: Image,
    pub image_bluesky: Image,

    pub image_pack_starter: Image,
    pub image_pack_stick: Image,
    pub image_pack_mud: Image,
    pub image_pack_water: Image,

    pub tile_materials: HashMap<TileType, Material>,
    pub pack_materials: HashMap<PackID, Material>,

    pub binary_file_system: BinaryFileSystem,
    pub asset_library: AssetLibrary,

    // Isn't really an asset but often needed when an asset would be
    pub tile_thumbnails: HashMap<TileType, Option<FrameBufferPack>>,

    pub missing_material: Material,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            image_stick: Image::new(),
            image_dirt_clod: Image::new(),
            image_rock: Image::new(),
            image_oak_wood: Image::new(),
            image_gold: Image::new(),
            image_acorn: Image::new(),
            image_dragon_egg: Image::new(),
            image_baby: Image::new(),
            image_berry: Image::new(),
            image_question_mark: Image::new(),
            image_mud_baby: Image::new(),
            image_pearl: Image::new(),
            image_old_boot: Image::new(),
            image_seaweed: Image::new(),
            image_trash_bag: Image::new(),
            image_old_hat: Image::new(),
            image_dew: Image::new(),
            image_glow: Image::new(),
            image_twitter: Image::new(),
            image_bluesky: Image::new(),

            image_pack_starter: Image::new(),
            image_pack_stick: Image::new(),
            image_pack_mud: Image::new(),
            image_pack_water: Image::new(),

            tile_materials: HashMap::new(),
            binary_file_system: BinaryFileSystem::new(),
            asset_library: AssetLibrary::new(),

            tile_thumbnails: HashMap::new(),
            pack_materials: HashMap::new(),
            missing_material: Material::new(),
        }
    }

    // Do assets setup. Probably better to not need this step and just have it be part of the constructor.
    pub fn build_assets(&mut self, pbr_shader: Shader, shader_color: Shader) {
        // build tile materials
        for tile_type in vec![
            TileType::Water,
            TileType::Grass,
            TileType::Dirt,
            TileType::Boulder,
            TileType::Cave,
            TileType::TallGrass,
            TileType::Shrub,
            TileType::OakTree,
            TileType::MudPit,
            TileType::BirdNest,
            TileType::Frog,
            TileType::Newt,
            TileType::Reed,
            TileType::Clam,
        ] {
            self.tile_materials.insert(
                tile_type,
                Self::build_pbr_material(
                    &tile_type.to_string_id(),
                    &self.asset_library,
                    pbr_shader,
                ),
            );
        }

        // build pack materials
        for pack in vec![PackID::Starter, PackID::Mud, PackID::Stick, PackID::Water] {
            self.pack_materials.insert(
                pack,
                Self::build_pbr_material(&pack.to_string_id(), &self.asset_library, pbr_shader),
            );
        }

        // build materials
        self.missing_material.shader = Some(shader_color);
        self.missing_material
            .set_color(Color::new(1.0, 0.0, 0.0, 1.0));
    }

    pub fn get_tile_thumbnail(&mut self, tile: &TileType) -> Option<u32> {
        if let Some(tile_thumbnail) = self.tile_thumbnails.get(tile) {
            if let Some(buffer_pack) = tile_thumbnail {
                return Some(buffer_pack.color_buffer);
            }
        } else {
            self.tile_thumbnails.insert(*tile, None);
        }

        return None;
    }

    pub fn get_item_icon(&mut self, item: &ItemType) -> u32 {
        self.get_item_image_opt(item)
            .unwrap_or(self.image_question_mark.gl_id.unwrap())
    }

    pub fn get_drop_icon(&mut self, drop: &DropType) -> u32 {
        match drop {
            DropType::Gold => return self.image_gold.gl_id.unwrap(),
            DropType::Item { item_type } => return self.get_item_icon(item_type),
        }
    }

    pub fn get_pack_icon(&self, pack_id: &PackID) -> u32 {
        self.get_pack_image_opt(pack_id)
            .expect(&format!("Missing pack image for {pack_id:?}"))
    }

    fn get_pack_image_opt(&self, pack: &PackID) -> Option<u32> {
        match pack {
            PackID::Starter => return self.image_pack_starter.gl_id,
            PackID::Stick => return self.image_pack_stick.gl_id,
            PackID::Mud => return self.image_pack_mud.gl_id,
            PackID::Water => return self.image_pack_water.gl_id,
        };
    }

    fn get_item_image_opt(&mut self, item: &ItemType) -> Option<u32> {
        match item {
            ItemType::DirtClod => return self.image_dirt_clod.gl_id,
            ItemType::Stick => return self.image_stick.gl_id,
            ItemType::Rock => return self.image_rock.gl_id,
            ItemType::OakLog => return self.image_oak_wood.gl_id,
            ItemType::Acorn => return self.image_acorn.gl_id,
            ItemType::DragonEgg => return self.image_dragon_egg.gl_id,
            ItemType::Baby => return self.image_baby.gl_id,
            ItemType::Berry => return self.image_berry.gl_id,
            ItemType::MudBaby => return self.image_mud_baby.gl_id,
            ItemType::Pearl => return self.image_pearl.gl_id,
            ItemType::OldBoot => return self.image_old_boot.gl_id,
            ItemType::Seaweed => return self.image_seaweed.gl_id,
            ItemType::TrashBag => return self.image_trash_bag.gl_id,
            ItemType::OldHat => return self.image_old_hat.gl_id,
            ItemType::Dew => return self.image_dew.gl_id,

            ItemType::Tile(tile_type) => return self.get_tile_thumbnail(tile_type),
        };
    }

    pub fn render_tile_thumbnail(
        &mut self,
        tile_type: TileType,
        test_dist: Option<f64>,
        test_height: Option<f64>,
        render_api: &mut impl RenderApi,
        components: &mut Components,
    ) {
        let cam_dist = match tile_type {
            TileType::OakTree => 9.14,
            TileType::Reed => 3.8,
            _ => 5.0,
        };

        let cam_height = match tile_type {
            TileType::OakTree => 7.5,
            TileType::Reed => 4.5,
            _ => 3.5,
        };

        let pos_dir = VecThreeFloat::new(-3.5, 5.2, 3.8).normalize();
        let mut pos = pos_dir * test_dist.unwrap_or(cam_dist);
        pos.y = test_height.unwrap_or(cam_height);

        // make frame buffer
        let buffer_pack = render_api
            .build_frame_buffer(512, 512)
            .expect("Error building framebuffer pack");

        // draw the tile into the framebuffer
        let mut render_pack = RenderPack::new(
            ProjectionType::Perspective { focal_length: 0.95 },
            VecTwo::new(512.0, 512.0),
        );

        // setup lights
        {
            let light = Light::new(components.new_transform());

            let ct: &mut Transform = &mut components.transforms[light.transform];
            ct.local_position.x = -2.0;
            ct.local_position.z = 10.0;
            ct.local_position.y = 15.0;

            render_pack.lights.push(light);

            // force the update of the new light transform
            Transform::update_all(&mut components.transforms);
        }

        let mut cam = &mut render_pack.camera;
        cam.transform.local_position = pos;
        cam.pitch = 44.0;
        cam.yaw = 133.2;

        cam.update_matricies();

        draw_tile_world_pos(
            tile_type,
            0.0,
            &VecThreeFloat::new_zero(),
            true,
            &mut render_pack,
            self,
        );

        render_api.draw_frame_buffer(buffer_pack.frame_buffer, &mut render_pack, components);

        // add to tiles hashmap
        self.tile_thumbnails.insert(tile_type, Some(buffer_pack));

        println!("Rendered thumbnail for {:?}", tile_type);
    }

    pub fn get_tile_material(&self, tile: TileType) -> &Material {
        self.tile_materials
            .get(&tile)
            .unwrap_or(&self.missing_material)
    }

    pub fn get_pack_material(&self, pack: PackID) -> &Material {
        self.pack_materials
            .get(&pack)
            .unwrap_or(&self.missing_material)
    }

    fn build_pbr_material(
        name_base: &str,
        asset_library: &AssetLibrary,
        pbr_shader: Shader,
    ) -> Material {
        let mut mat = Material::new();

        mat.shader = Some(pbr_shader);
        mat.uniforms.insert(
            "tex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: asset_library
                    .get_texture(&format!("{}_base_color", name_base))
                    .gl_id
                    .unwrap(),
                texture_slot: 0,
            }),
        );

        mat.uniforms.insert(
            "normalTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: asset_library
                    .get_texture(&format!("{}_normal", name_base))
                    .gl_id
                    .unwrap(),
                texture_slot: 1,
            }),
        );

        mat.uniforms.insert(
            "metallicTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: asset_library
                    .get_texture(&format!("{}_metallic", name_base))
                    .gl_id
                    .unwrap(),
                texture_slot: 2,
            }),
        );

        mat.uniforms.insert(
            "roughnessTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: asset_library
                    .get_texture(&format!("{}_roughness", name_base))
                    .gl_id
                    .unwrap(),
                texture_slot: 3,
            }),
        );

        mat.uniforms.insert(
            "aoTex".to_string(),
            UniformData::Texture(TextureInfo {
                image_id: asset_library
                    .get_texture(&format!("{}_ao", name_base))
                    .gl_id
                    .unwrap(),
                texture_slot: 4,
            }),
        );

        mat.uniforms
            .insert("ambientRed".to_string(), UniformData::Float(0.05));

        return mat;
    }
}
