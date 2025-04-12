use crate::{
    error::*,
    grid::GridPos,
    tile::{tile_definition::*, tile_instance::*, tiles::*, TileMethods},
    world::*,
};

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
}

impl TileType {
    /// Can you place the tile here
    pub fn can_place_here(&self, origin: GridPos, world: &World) -> bool {
        let definition = self.get_definition();
        let world_snapshot = world.get_world_snapshot();

        for p in &definition.footprint {
            let pos = origin + *p;

            for cond in &definition.placement_constraints {
                if !cond.valid(pos, &world_snapshot) {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn create_instance(&self, grid_pos: GridPos) -> TileInstance {
        let def = self.get_definition();
        let methods = (def.build_methods)(grid_pos);

        let mut inst = TileInstance::new(*self, grid_pos, methods);
        (def.add_components)(&mut inst, grid_pos);

        return inst;
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
            _ => Err(Error::InvalidTileTypeIndex(idx)),
        }
    }
}
