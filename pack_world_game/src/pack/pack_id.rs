use crate::{
    error::*,
    pack::{packs::*, Pack},
    save_file::*,
};
use std::sync::LazyLock;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum PackID {
    Starter,
    Stick,
    Mud,
    Water,
    MudHeart,
}

pub const ALL_PACKS: LazyLock<Vec<PackID>> = LazyLock::new(|| {
    vec![
        PackID::Starter,
        PackID::Stick,
        PackID::Mud,
        PackID::Water,
        PackID::MudHeart,
    ]
});

impl PackID {
    pub fn to_index(&self) -> i32 {
        match self {
            Self::Starter => 0,
            Self::Stick => 1,
            Self::Mud => 2,
            Self::Water => 3,
            Self::MudHeart => 4,
        }
    }

    pub fn to_string_id(&self) -> String {
        format!("pack_{:?}", self).to_lowercase()
    }

    pub fn from_index(index: i32) -> Self {
        match index {
            0 => Self::Starter,
            1 => Self::Stick,
            2 => Self::Mud,
            3 => Self::Water,
            4 => Self::MudHeart,
            _ => panic!("Invalid PackID index"),
        }
    }

    pub fn get_pack_info(&self) -> &'static Pack {
        match self {
            PackID::Starter => &STARTER,
            PackID::Stick => &STICK,
            PackID::Mud => &MUD,
            PackID::Water => &WATER,
            PackID::MudHeart => &MUD_HEART,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_id_index() {
        fn validate(pack_id: PackID) {
            match pack_id {
                PackID::Starter => assert_eq!(
                    PackID::from_index(PackID::Starter.to_index()),
                    PackID::Starter
                ),
                PackID::Stick => {
                    assert_eq!(PackID::from_index(PackID::Stick.to_index()), PackID::Stick)
                }
                PackID::Mud => {
                    assert_eq!(PackID::from_index(PackID::Mud.to_index()), PackID::Mud)
                }
                PackID::Water => {
                    assert_eq!(PackID::from_index(PackID::Water.to_index()), PackID::Water)
                }
                PackID::MudHeart => {
                    assert_eq!(
                        PackID::from_index(PackID::MudHeart.to_index()),
                        PackID::MudHeart
                    )
                }
            };
        }

        validate(PackID::Starter);
        validate(PackID::Stick);
        validate(PackID::Mud);
        validate(PackID::Water);
        validate(PackID::MudHeart);
    }
}
