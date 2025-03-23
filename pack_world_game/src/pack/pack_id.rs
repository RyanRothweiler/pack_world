use crate::{error::*, save_file::*};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum PackID {
    Starter,
    Stick,
}

impl PackID {
    pub fn to_index(&self) -> i32 {
        match self {
            Self::Starter => 0,
            Self::Stick => 1,
        }
    }

    pub fn from_index(index: i32) -> Self {
        match index {
            0 => Self::Starter,
            1 => Self::Stick,
            _ => panic!("Invalid PackID index"),
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
            };
        }

        validate(PackID::Starter);
        validate(PackID::Stick);
    }
}
