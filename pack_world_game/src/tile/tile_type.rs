use crate::{
    error::*,
    grid::GridPos,
    tile::{tile_instance::*, tiles::*, TileMethods},
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

/// Static tile info. Things like layer type, title, description
pub struct TileDefinition<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub world_layer: WorldLayer,
}

// TOOD create a tile definition. and one method to return that definition instead of individual methods for each field.
impl TileType {
    /// Can you place the tile here
    pub fn can_place_here(&self, origin: GridPos, world: &World) -> bool {
        let footprint = self.get_tile_footprint();
        for p in footprint {
            let pos = origin + p;

            let val = match self {
                TileType::Dirt => TileDirt::can_place(pos, world),
                TileType::Grass => TileGrass::can_place(pos, world),
                TileType::Boulder => TileBoulder::can_place(pos, world),
                TileType::OakTree => TileOakTree::can_place(pos, world),
                TileType::Cave => TileCave::can_place(pos, world),
                TileType::Shrub => TileShrub::can_place(pos, world),
                TileType::BirdNest => TileBirdNest::can_place(pos, world),
                TileType::MudPit => TileMudPit::can_place(pos, world),
                TileType::TallGrass => TileTallGrass::can_place(pos, world),
                TileType::Frog => TileFrog::can_place(pos, world),
                TileType::Water => TileWater::can_place(pos, world),
                TileType::Newt => TileNewt::can_place(pos, world),
                TileType::Reed => TileReed::can_place(pos, world),
                TileType::Clam => TileClam::can_place(pos, world),
            };

            if !val {
                return false;
            }
        }

        return true;
    }

    pub fn to_methods(&self, origin: GridPos) -> TileMethods {
        match self {
            TileType::Dirt => TileDirt::new_methods(),
            TileType::Grass => TileGrass::new_methods(),
            TileType::Boulder => TileBoulder::new_methods(),
            TileType::OakTree => TileOakTree::new_methods(),
            TileType::BirdNest => TileBirdNest::new_methods(),
            TileType::Cave => TileCave::new_methods(),
            TileType::Shrub => TileShrub::new_methods(),
            TileType::MudPit => TileMudPit::new_methods(),
            TileType::TallGrass => TileTallGrass::new_methods(),
            TileType::Frog => TileFrog::new_methods(origin),
            TileType::Water => TileWater::new_methods(),
            TileType::Newt => TileNewt::new_methods(origin),
            TileType::Reed => TileReed::new_methods(),
            TileType::Clam => TileClam::new_methods(),
        }
    }

    pub fn create_instance(&self, grid_pos: GridPos) -> TileInstance {
        TileInstance::new(*self, grid_pos, self.to_methods(grid_pos))
    }

    pub fn get_tile_footprint(&self) -> Vec<GridPos> {
        match self {
            TileType::Dirt
            | TileType::Grass
            | TileType::Water
            | TileType::Boulder
            | TileType::TallGrass
            | TileType::Shrub
            | TileType::Clam
            | TileType::MudPit
            | TileType::Reed
            | TileType::BirdNest
            | TileType::Cave => {
                vec![GridPos::new(0, 0)]
            }
            TileType::Frog => GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
            TileType::Newt => GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
            TileType::OakTree => GridPos::new(0, 0).to_rect_iter(2, 2).collect(),
        }
    }

    pub fn get_definition(&self) -> &'static TileDefinition<'static> {
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
            TileType::Frog => &tile_frog::DEF,
            TileType::Newt => &tile_newt::DEF,
            TileType::OakTree => &tile_oak_tree::DEF,
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
