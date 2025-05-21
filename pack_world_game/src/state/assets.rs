use crate::{drop_table::*, item::*, pack::*, state::inventory::*, tile::*};
use gengar_engine::{
    binary_file_system::*,
    model::*,
    render::{
        camera::*, frame_buffer_pack::*, image::*, material::*, render_pack::*, shader::*,
        RenderApi,
    },
    vectors::*,
};
use std::collections::HashMap;

pub mod asset_library;
pub mod tile_asset_pack;

pub use asset_library::*;
pub use tile_asset_pack::*;

pub struct Assets {
    pub image_dirt: Image,
    pub image_grass: Image,
    pub image_dirt_clod: Image,
    pub image_stick: Image,
    pub image_boulder: Image,
    pub image_rock: Image,
    pub image_oak_tree: Image,
    pub image_oak_wood: Image,
    pub image_bird_nest: Image,
    pub image_gold: Image,
    pub image_acorn: Image,
    pub image_cave: Image,
    pub image_dragon_egg: Image,
    pub image_baby: Image,
    pub image_shrub: Image,
    pub image_berry: Image,
    pub image_question_mark: Image,
    pub image_mud_pit: Image,
    pub image_tall_grass: Image,
    pub image_mud_baby: Image,
    pub image_frog: Image,
    pub image_water: Image,
    pub image_newt: Image,
    pub image_reed: Image,
    pub image_clam: Image,
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

    pub binary_file_system: BinaryFileSystem,
    pub asset_library: AssetLibrary,

    // Isn't really an asset but often needed when an asset would be
    pub tile_thumbnails: HashMap<TileType, Option<FrameBufferPack>>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            image_dirt: Image::new(),
            image_grass: Image::new(),
            image_stick: Image::new(),
            image_dirt_clod: Image::new(),
            image_boulder: Image::new(),
            image_rock: Image::new(),
            image_oak_tree: Image::new(),
            image_oak_wood: Image::new(),
            image_bird_nest: Image::new(),
            image_gold: Image::new(),
            image_acorn: Image::new(),
            image_cave: Image::new(),
            image_dragon_egg: Image::new(),
            image_baby: Image::new(),
            image_shrub: Image::new(),
            image_berry: Image::new(),
            image_question_mark: Image::new(),
            image_mud_pit: Image::new(),
            image_tall_grass: Image::new(),
            image_mud_baby: Image::new(),
            image_frog: Image::new(),
            image_water: Image::new(),
            image_newt: Image::new(),
            image_reed: Image::new(),
            image_clam: Image::new(),
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
        }
    }

    pub fn build_tile_materials(&mut self, pbr_shader: Shader) {
        pub fn build_tile_material(
            tile_type: TileType,
            asset_library: &AssetLibrary,
            pbr_shader: Shader,
        ) -> Material {
            let mut mat = Material::new();

            let tile_base_id = tile_type.to_string_id();

            mat.shader = Some(pbr_shader);
            mat.uniforms.insert(
                "tex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: asset_library
                        .get_texture(&format!("{}_base_color", tile_base_id))
                        .gl_id
                        .unwrap(),
                    texture_slot: 0,
                }),
            );

            mat.uniforms.insert(
                "normalTex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: asset_library
                        .get_texture(&format!("{}_normal", tile_base_id))
                        .gl_id
                        .unwrap(),
                    texture_slot: 1,
                }),
            );

            mat.uniforms.insert(
                "metallicTex    ".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: asset_library
                        .get_texture(&format!("{}_metallic", tile_base_id))
                        .gl_id
                        .unwrap(),
                    texture_slot: 2,
                }),
            );

            mat.uniforms.insert(
                "roughnessTex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: asset_library
                        .get_texture(&format!("{}_roughness", tile_base_id))
                        .gl_id
                        .unwrap(),
                    texture_slot: 3,
                }),
            );

            mat.uniforms.insert(
                "aoTex".to_string(),
                UniformData::Texture(TextureInfo {
                    image_id: asset_library
                        .get_texture(&format!("{}_ao", tile_base_id))
                        .gl_id
                        .unwrap(),
                    texture_slot: 4,
                }),
            );

            return mat;
        }

        self.tile_materials.insert(
            TileType::Water,
            build_tile_material(TileType::Water, &self.asset_library, pbr_shader),
        );
        self.tile_materials.insert(
            TileType::Grass,
            build_tile_material(TileType::Grass, &self.asset_library, pbr_shader),
        );
        self.tile_materials.insert(
            TileType::Dirt,
            build_tile_material(TileType::Dirt, &self.asset_library, pbr_shader),
        );
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

    pub fn get_tile_icon(&self, tile: &TileType) -> u32 {
        self.get_tile_image_opt(tile)
            .expect(&format!("Missing tile image for {:?}", tile))
    }

    pub fn get_item_icon(&mut self, item: &ItemType) -> u32 {
        if let Some(item_icon) = self.get_item_image_opt(item) {
            return item_icon;
        } else {
            return self.image_grass.gl_id.unwrap();
        }
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

    fn get_tile_image_opt(&self, tile: &TileType) -> Option<u32> {
        match tile {
            TileType::Dirt => return self.image_dirt.gl_id,
            TileType::Grass => return self.image_grass.gl_id,
            TileType::Boulder => return self.image_boulder.gl_id,
            TileType::OakTree => return self.image_oak_tree.gl_id,
            TileType::BirdNest => return self.image_bird_nest.gl_id,
            TileType::Cave => return self.image_cave.gl_id,
            TileType::Shrub => return self.image_shrub.gl_id,
            TileType::MudPit => return self.image_mud_pit.gl_id,
            TileType::TallGrass => return self.image_tall_grass.gl_id,
            TileType::Frog => return self.image_frog.gl_id,
            TileType::Water => return self.image_water.gl_id,
            TileType::Newt => return self.image_newt.gl_id,
            TileType::Reed => return self.image_reed.gl_id,
            TileType::Clam => return self.image_clam.gl_id,
        };
    }

    pub fn render_tile_thumbnail(
        &mut self,
        tile_type: TileType,
        test_dist: Option<f64>,
        test_height: Option<f64>,
        render_api: &impl RenderApi,
    ) {
        let cam_dist = match tile_type {
            _ => 5.0,
        };

        let cam_height = match tile_type {
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

        let mut cam = &mut render_pack.camera;
        cam.transform.local_position = pos;
        cam.pitch = 44.0;
        cam.yaw = 133.2;

        cam.update_matricies();

        draw_tile_world_pos(
            tile_type,
            0.0,
            &VecThreeFloat::new_zero(),
            &mut render_pack,
            self,
        );

        render_api.draw_frame_buffer(buffer_pack.frame_buffer, &mut render_pack);

        // add to tiles hashmap
        self.tile_thumbnails.insert(tile_type, Some(buffer_pack));

        println!("Rendered thumbnail for {:?}", tile_type);
    }
}
