use crate::{
    error::*,
    grid::GridPos,
    tile::{
        tile_instance::*,
        tiles::{
            tile_bird_nest::TileBirdNest, tile_boulder::TileBoulder, tile_cave::TileCave,
            tile_dirt::TileDirt, tile_grass::TileGrass, tile_oak_tree::TileOakTree,
            tile_shrub::TileShrub, *,
        },
    },
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
            TileType::Boulder
            | TileType::OakTree
            | TileType::Cave
            | TileType::Shrub
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
            };

            if !val {
                return false;
            }
        }

        return true;
    }

    pub fn create_instance(&self, grid_pos: GridPos) -> TileInstance {
        let methods = match self {
            TileType::Dirt => TileDirt::new_methods(),
            TileType::Grass => TileGrass::new_methods(),
            TileType::Boulder => TileBoulder::new_methods(),
            TileType::OakTree => TileOakTree::new_methods(),
            TileType::BirdNest => TileBirdNest::new_methods(),
            TileType::Cave => TileCave::new_methods(),
            TileType::Shrub => TileShrub::new_methods(),
        };

        TileInstance::new(*self, grid_pos, methods)
    }

    pub fn get_tile_footprint(&self) -> Vec<GridPos> {
        match self {
            TileType::Dirt
            | TileType::Grass
            | TileType::Boulder
            | TileType::Shrub
            | TileType::BirdNest
            | TileType::Cave => {
                vec![GridPos::new(0, 0)]
            }
            TileType::OakTree => vec![
                GridPos::new(0, 0),
                GridPos::new(1, 1),
                GridPos::new(0, 1),
                GridPos::new(1, 0),
            ],
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
            _ => Err(Error::InvalidTileTypeIndex(idx)),
        }
    }
}
