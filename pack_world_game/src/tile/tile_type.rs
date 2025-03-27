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
}

// TOOD create a tile definition. and one method to return that definition instead of individual methods for each field.
impl TileType {
    pub fn user_title(&self) -> &str {
        match self {
            TileType::Dirt => tile_dirt::TITLE,
            TileType::Grass => tile_grass::TITLE,
            TileType::Boulder => tile_boulder::TITLE,
            TileType::OakTree => tile_oak_tree::TITLE,
            TileType::BirdNest => tile_bird_nest::TITLE,
            TileType::Cave => tile_cave::TITLE,
            TileType::Shrub => tile_shrub::TITLE,
            TileType::MudPit => tile_mud_pit::TITLE,
            TileType::TallGrass => tile_tall_grass::TITLE,
            TileType::Frog => tile_frog::TITLE,
        }
    }

    pub fn user_description(&self) -> Option<&str> {
        match self {
            TileType::Dirt => Some(tile_dirt::DESC),
            _ => None,
        }
    }

    pub fn get_layer(&self) -> WorldLayer {
        match self {
            TileType::Dirt => WorldLayer::Ground,
            TileType::BirdNest => WorldLayer::TreeAttachment,
            TileType::Frog => WorldLayer::Walker,
            TileType::Boulder
            | TileType::OakTree
            | TileType::Cave
            | TileType::Shrub
            | TileType::TallGrass
            | TileType::MudPit
            | TileType::Grass => WorldLayer::Floor,
        }
    }

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
        }
    }

    pub fn create_instance(&self, grid_pos: GridPos) -> TileInstance {
        TileInstance::new(*self, grid_pos, self.to_methods(grid_pos))
    }

    pub fn get_tile_footprint(&self) -> Vec<GridPos> {
        match self {
            TileType::Dirt
            | TileType::Grass
            | TileType::Boulder
            | TileType::TallGrass
            | TileType::Shrub
            | TileType::MudPit
            | TileType::BirdNest
            | TileType::Cave => {
                vec![GridPos::new(0, 0)]
            }
            TileType::Frog => GridPos::new(0, 0).to_rect_iter(4, 4).collect(),
            TileType::OakTree => GridPos::new(0, 0).to_rect_iter(2, 2).collect(),
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
            _ => Err(Error::InvalidTileTypeIndex(idx)),
        }
    }
}
