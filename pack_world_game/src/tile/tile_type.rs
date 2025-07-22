use crate::{
    error::*,
    grid::GridPos,
    tile::{tile_definition::*, tile_instance::*, tiles::*, TileMethods},
    world::*,
};
use std::sync::LazyLock;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum TileType {
    Dirt,
    Grass,
    Boulder,
    OakTree,
    BirdNest,
    Cave,
    Shrub,
    MudPit,
    TallGrass,
    Frog,
    Water,
    Newt,
    Reed,
    Clam,
    MudFish,
    Spring,
    Kelp,
    Crab,
    MudHenge,
    MudChicken,
    Goblin,
}

pub const ALL_TILE_TYPES: LazyLock<Vec<TileType>> = LazyLock::new(|| {
    vec![
        TileType::Dirt,
        TileType::Grass,
        TileType::Boulder,
        TileType::OakTree,
        TileType::BirdNest,
        TileType::Cave,
        TileType::Shrub,
        TileType::MudPit,
        TileType::TallGrass,
        TileType::Frog,
        TileType::Water,
        TileType::Newt,
        TileType::Reed,
        TileType::Clam,
        TileType::MudFish,
        TileType::Spring,
        TileType::Kelp,
        TileType::Crab,
        TileType::MudHenge,
        TileType::MudChicken,
        TileType::Goblin,
    ]
});

impl TileType {
    /// Can you place the tile here
    pub fn can_place_here(&self, origin: GridPos, world: &World) -> bool {
        let definition = self.get_definition();
        let world_snapshot = world.get_world_snapshot();

        for p in &definition.footprint {
            let pos = origin + *p;

            if !self.pos_passes_placement_constraints(pos, world) {
                return false;
            }
        }

        return true;
    }

    /// Doesn't mean the tile is placeable. Only means this pos pases the placement constraints.
    pub fn pos_passes_placement_constraints(&self, pos: GridPos, world: &World) -> bool {
        let definition = self.get_definition();
        let world_snapshot = world.get_world_snapshot();

        for cond in &definition.placement_constraints {
            if !cond.valid(pos, &world_snapshot) {
                return false;
            }
        }

        return true;
    }

    pub fn create_instance(&self, grid_pos: GridPos) -> TileInstance {
        let def = self.get_definition();
        (def.new_instance)(grid_pos)
    }

    pub fn to_string_id(&self) -> String {
        format!("tile_{:?}", self).to_lowercase()
    }

    pub fn get_definition(&self) -> &'static TileDefinition {
        match self {
            TileType::Grass => &tile_grass::DEF,
            TileType::Water => &tile_water::DEF,
            TileType::Dirt => &tile_dirt::DEF,
            TileType::Boulder => &tile_boulder::DEF,
            TileType::TallGrass => &tile_tall_grass::DEF,
            TileType::Shrub => &tile_shrub::DEF,
            TileType::Clam => &tile_clam::DEF,
            TileType::MudPit => &tile_mud_pit::DEF,
            TileType::Reed => &tile_reed::DEF,
            TileType::BirdNest => &tile_bird_nest::DEF,
            TileType::Cave => &tile_cave::DEF,
            TileType::Newt => &tile_newt::DEF,
            TileType::OakTree => &tile_oak_tree::DEF,
            TileType::Frog => &tile_frog::DEF,
            TileType::MudFish => &tile_mud_fish::DEF,
            TileType::Spring => &tile_spring::DEF,
            TileType::Kelp => &tile_kelp::DEF,
            TileType::Crab => &tile_crab::DEF,
            TileType::MudHenge => &tile_mud_henge::DEF,
            TileType::MudChicken => &tile_mud_chicken::DEF,
            TileType::Goblin => &tile_goblin::DEF,
        }
    }

    pub fn to_index(&self) -> i32 {
        match self {
            Self::Dirt => 0,
            Self::Grass => 1,
            Self::Boulder => 2,
            Self::Shrub => 3,
            Self::BirdNest => 4,
            Self::Cave => 5,
            Self::OakTree => 6,
            Self::MudPit => 7,
            Self::TallGrass => 8,
            Self::Frog => 9,
            Self::Water => 10,
            Self::Newt => 11,
            Self::Reed => 12,
            Self::Clam => 13,
            Self::MudFish => 14,
            Self::Spring => 15,
            Self::Kelp => 16,
            Self::Crab => 17,
            Self::MudHenge => 18,
            Self::MudChicken => 19,
            Self::Goblin => 20,
        }
    }

    pub fn from_index(idx: i32) -> Result<Self, Error> {
        match idx {
            0 => Ok(Self::Dirt),
            1 => Ok(Self::Grass),
            2 => Ok(Self::Boulder),
            3 => Ok(Self::Shrub),
            4 => Ok(Self::BirdNest),
            5 => Ok(Self::Cave),
            6 => Ok(Self::OakTree),
            7 => Ok(Self::MudPit),
            8 => Ok(Self::TallGrass),
            9 => Ok(Self::Frog),
            10 => Ok(Self::Water),
            11 => Ok(Self::Newt),
            12 => Ok(Self::Reed),
            13 => Ok(Self::Clam),
            14 => Ok(Self::MudFish),
            15 => Ok(Self::Spring),
            16 => Ok(Self::Kelp),
            17 => Ok(Self::Crab),
            18 => Ok(Self::MudHenge),
            19 => Ok(Self::MudChicken),
            20 => Ok(Self::Goblin),
            _ => Err(Error::InvalidTileTypeIndex(idx)),
        }
    }
}
