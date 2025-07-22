use crate::{error::*, save_file::*, tile::*};
use std::sync::LazyLock;

mod item_data;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ItemType {
    DirtClod,
    Stick,
    Rock,
    OakLog,
    Acorn,
    DragonEgg,
    Baby,
    Berry,
    MudHeart,
    Pearl,
    OldBoot,
    Seaweed,
    TrashBag,
    OldHat,
    Dew,
    EyeOfNewt,
    FrogLeg,
    Root,

    Tile(TileType),
}

pub const ALL_ITEM_TYPES: LazyLock<Vec<ItemType>> = LazyLock::new(|| {
    vec![
        ItemType::DirtClod,
        ItemType::Stick,
        ItemType::Rock,
        ItemType::OakLog,
        ItemType::Acorn,
        ItemType::DragonEgg,
        ItemType::Baby,
        ItemType::Berry,
        ItemType::MudHeart,
        ItemType::Pearl,
        ItemType::OldBoot,
        ItemType::Seaweed,
        ItemType::TrashBag,
        ItemType::OldHat,
        ItemType::Dew,
        ItemType::EyeOfNewt,
        ItemType::FrogLeg,
        ItemType::Root,
    ]
});

impl ItemType {
    pub fn user_title(&self) -> &str {
        match self {
            ItemType::Acorn => item_data::acorn::TITLE,
            ItemType::DirtClod => item_data::dirt_clod::TITLE,
            ItemType::Stick => item_data::stick::TITLE,
            ItemType::Rock => item_data::rock::TITLE,
            ItemType::OakLog => item_data::oak_wood::TITLE,
            ItemType::DragonEgg => item_data::dragon_egg::TITLE,
            ItemType::Baby => item_data::baby::TITLE,
            ItemType::Berry => item_data::berry::TITLE,
            ItemType::MudHeart => "Mud Heart",
            ItemType::Pearl => item_data::pearl::TITLE,
            ItemType::OldBoot => "Old Boot",
            ItemType::Seaweed => "Seaweed",
            ItemType::TrashBag => "Trash Bag",
            ItemType::OldHat => "Old Hat",
            ItemType::Dew => "Dew",
            ItemType::EyeOfNewt => "Eye of Newt",
            ItemType::FrogLeg => "Frog Leg",
            ItemType::Root => "Root",

            ItemType::Tile(tile_type) => tile_type.get_definition().title,
        }
    }

    pub fn user_description(&self) -> Option<&str> {
        let mut ret = match self {
            ItemType::Tile(tile_type) => Some(tile_type.get_definition().description),

            ItemType::Acorn => Some("An acorn"),
            ItemType::DirtClod => Some("Basic introductory resource"),
            ItemType::Stick => Some("Basic introductory resource"),
            ItemType::Rock => Some("Basic introductory resource"),
            ItemType::OakLog => Some("Basic introductory resource"),
            ItemType::DragonEgg => Some("Hatches into a dragon in the right environment!"),
            ItemType::Baby => Some("Grows up to be a big boy one day."),
            ItemType::Berry => Some("Basic food resource"),
            ItemType::MudHeart => Some("Grows up into a mud being one day."),
            ItemType::Pearl => Some("Contains life."),
            ItemType::OldBoot => Some("Just some trash."),
            ItemType::Seaweed => Some("Just some trash."),
            ItemType::TrashBag => Some("Just some trash."),
            ItemType::OldHat => Some("Just some trash."),
            ItemType::Dew => Some("Morning dew. Get enough and open a water pack."),
            ItemType::EyeOfNewt => Some("Gross!"),
            ItemType::FrogLeg => Some("Please don't kick."),
            ItemType::Root => Some("Very chewwy."),
        };

        ret
    }

    pub fn is_tile(&self) -> bool {
        match self {
            ItemType::Tile(_) => true,
            _ => false,
        }
    }

    pub fn save_file_write(
        &self,
        key_parent: String,
        save_file: &mut SaveFile,
    ) -> Result<(), Error> {
        let id_key = format!("{}.t", key_parent);

        match self {
            Self::DirtClod => {
                save_file.save_i32(&id_key, 0);
            }
            Self::Acorn => {
                save_file.save_i32(&id_key, 1);
            }
            Self::Stick => {
                save_file.save_i32(&id_key, 2);
            }
            Self::Rock => {
                save_file.save_i32(&id_key, 3);
            }
            Self::OakLog => {
                save_file.save_i32(&id_key, 4);
            }
            Self::DragonEgg => {
                save_file.save_i32(&id_key, 5);
            }
            Self::Baby => {
                save_file.save_i32(&id_key, 6);
            }
            Self::Berry => {
                save_file.save_i32(&id_key, 7);
            }
            Self::Tile(tile_type) => {
                save_file.save_i32(&id_key, 8);

                let tile_type_key = format!("{}.t", id_key);
                save_file.save_i32(&tile_type_key, tile_type.to_index());
            }

            Self::MudHeart => {
                save_file.save_i32(&id_key, 9);
            }
            Self::Pearl => {
                save_file.save_i32(&id_key, 10);
            }
            Self::OldBoot => {
                save_file.save_i32(&id_key, 11);
            }
            Self::Seaweed => {
                save_file.save_i32(&id_key, 12);
            }
            Self::TrashBag => {
                save_file.save_i32(&id_key, 13);
            }
            Self::OldHat => {
                save_file.save_i32(&id_key, 14);
            }
            Self::Dew => {
                save_file.save_i32(&id_key, 15);
            }
            Self::EyeOfNewt => {
                save_file.save_i32(&id_key, 16);
            }
            Self::FrogLeg => {
                save_file.save_i32(&id_key, 17);
            }
            Self::Root => {
                save_file.save_i32(&id_key, 18);
            }
        }

        Ok(())
    }

    pub fn save_file_load(key_parent: String, save_file: &SaveFile) -> Result<Self, Error> {
        let id_key = format!("{}.t", key_parent);
        let id = save_file.load_i32(&id_key).unwrap();

        match id {
            0 => Ok(Self::DirtClod),
            1 => Ok(Self::Acorn),
            2 => Ok(Self::Stick),
            3 => Ok(Self::Rock),
            4 => Ok(Self::OakLog),
            5 => Ok(Self::DragonEgg),
            6 => Ok(Self::Baby),
            7 => Ok(Self::Berry),
            8 => {
                let tile_type_key = format!("{}.t", id_key);
                let tile_id = save_file.load_i32(&tile_type_key).unwrap();

                return Ok(Self::Tile(TileType::from_index(tile_id)?));
            }
            9 => Ok(Self::MudHeart),
            10 => Ok(Self::Pearl),
            11 => Ok(Self::OldBoot),
            12 => Ok(Self::Seaweed),
            13 => Ok(Self::TrashBag),
            14 => Ok(Self::OldHat),
            15 => Ok(Self::Dew),
            16 => Ok(Self::EyeOfNewt),
            17 => Ok(Self::FrogLeg),
            18 => Ok(Self::Root),
            _ => panic!("Invalid item id"),
        }
    }
}

mod test {
    use super::*;
    use crate::save_file::*;

    #[test]
    fn save_load() {
        let mut save_file = SaveFile::new();

        ItemType::DirtClod
            .save_file_write("dirt".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("dirt".into(), &save_file).unwrap(),
            ItemType::DirtClod
        );

        ItemType::Acorn
            .save_file_write("acorn".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("acorn".into(), &save_file).unwrap(),
            ItemType::Acorn
        );

        ItemType::Stick
            .save_file_write("stick".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("stick".into(), &save_file).unwrap(),
            ItemType::Stick
        );

        ItemType::Rock
            .save_file_write("rock".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("rock".into(), &save_file).unwrap(),
            ItemType::Rock
        );

        ItemType::OakLog
            .save_file_write("oaklog".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("oaklog".into(), &save_file).unwrap(),
            ItemType::OakLog
        );

        ItemType::DragonEgg
            .save_file_write("dragonegg".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("dragonegg".into(), &save_file).unwrap(),
            ItemType::DragonEgg
        );

        ItemType::Baby
            .save_file_write("baby".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("baby".into(), &save_file).unwrap(),
            ItemType::Baby
        );

        ItemType::Berry
            .save_file_write("berry".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("berry".into(), &save_file).unwrap(),
            ItemType::Berry
        );

        ItemType::Tile(TileType::Boulder)
            .save_file_write("boulder".into(), &mut save_file)
            .unwrap();
        assert_eq!(
            ItemType::save_file_load("boulder".into(), &save_file).unwrap(),
            ItemType::Tile(TileType::Boulder)
        );
    }
}
