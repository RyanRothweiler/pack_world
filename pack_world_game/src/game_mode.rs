pub mod game_mode_inventory;
pub mod game_mode_shop;
pub mod game_mode_world;

pub use game_mode_inventory::*;
pub use game_mode_shop::*;
pub use game_mode_world::*;

#[derive(Debug)]
pub enum GameMode {
    World(GameModeWorldState),
    Shop,
    Inventory(GameModeInventoryState),
}

impl GameMode {
    pub fn update(
        &mut self,
        prev_delta_time: f64,
        es: &mut EngineState,
        mut ui_frame_state: &mut UIFrameState,
        input: &mut Input,
        render_api: &mut impl RenderApi,
        platform_api: &PlatformApi,
        inventory: &mut Inventory,
        assets: &mut Assets,
        ui_context: &mut UIContext,
        world: &mut World,
    ) -> Vec<UpdateSignal> {
        match self {
            GameMode::World(state) => {
                return state.update(
                    prev_delta_time,
                    es,
                    input,
                    render_api,
                    platform_api,
                    world,
                    assets,
                    inventory,
                    ui_context,
                    ui_frame_state,
                );
            }
            GameMode::Shop => {}
            GameMode::Inventory(state) => state.update(
                prev_delta_time,
                es,
                ui_frame_state,
                input,
                render_api,
                platform_api,
                inventory,
                assets,
                ui_context,
            ),
        }

        return vec![];
    }

    pub fn into_kind(&self) -> GameModeKind {
        match self {
            GameMode::World(state) => GameModeKind::World,
            GameMode::Shop => GameModeKind::Shop,
            GameMode::Inventory(state) => GameModeKind::Inventory,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameModeKind {
    World,
    Shop,
    Inventory,
}

impl GameModeKind {
    pub fn into_mode(&self) -> GameMode {
        match self {
            GameModeKind::World => GameMode::World(GameModeWorldState::new()),
            GameModeKind::Shop => GameMode::Shop,
            GameModeKind::Inventory => GameMode::Inventory(GameModeInventoryState {}),
        }
    }
}
