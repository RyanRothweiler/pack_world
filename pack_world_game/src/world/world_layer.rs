#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum WorldLayer {
    /// The ground itself. Dirt, water, sand.
    Ground,

    /// Stuff on the floor. Trees, grass.
    Floor,

    /// Things planted in the soil.
    Planted,

    /// Stuff that walks on the ground, like an animal
    Walker,

    /// Find something better to do here. This is just for the birds nest. What to do about attachments?
    TreeAttachment,
}

impl WorldLayer {
    pub fn to_index(&self) -> i32 {
        match self {
            WorldLayer::Ground => 0,
            WorldLayer::Floor => 1,
            WorldLayer::TreeAttachment => 2,
            WorldLayer::Walker => 3,
            WorldLayer::Planted => 4,
        }
    }
}
