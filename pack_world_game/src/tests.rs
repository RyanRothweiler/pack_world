use crate::{drop_table::*, grid::*, testing_infra::*, tile::*, tiles::*, world::*};
use gengar_engine::time::*;

#[test]
pub fn tile_instance_drop_count_conditions() {
    let mut world = World::new();
    let world_snapshot = world.get_world_snapshot();

    let plat_api = windows_plaform_api();

    // insert tiles

    let mut fish_inst = {
        let mut inst =
            TileInstance::new(TileType::MudFish, GridPos::new(0, 0), TileMethods::MudFish);

        let mut tch = TileCompHarvest::new(
            Time::new(TimeUnit::Seconds(0.0)),
            FixedTableID::TestGold,
            false,
        );
        tch.add_drop_count_condition(2.0, WorldCondition::AdjacentTo(TileSnapshot::MudPit));
        inst.comp_harvest = Some(tch);

        inst
    };

    let mut mud_pit_inst = tile_mud_pit::new_instance(GridPos::new(1, 0));

    // insert fish into world
    let _ = world.insert_tile_instance(GridPos::new(0, 0), fish_inst);

    // harvest fish
    let world_cell: WorldCell = world.get_entities(GridPos::new(0, 0));
    for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
        let tile = world.get_entity_mut(eid);
        tile.harvest(&world_snapshot, &plat_api);

        assert_eq!(tile.drops_queue.len(), 1);

        tile.drops_queue.clear();
    }

    // insert mud pit adjacent
    let _ = world.insert_tile_instance(GridPos::new(1, 0), mud_pit_inst);

    // harvest fish again. should have double the drops now
    let world_cell: WorldCell = world.get_entities(GridPos::new(0, 0));
    for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
        let tile = world.get_entity_mut(eid);
        tile.harvest(&world_snapshot, &plat_api);

        assert_eq!(tile.drops_queue.len(), 2);

        tile.drops_queue.clear();
    }
}

#[test]
pub fn global_drop_count_mod() {
    let mut world = World::new();

    let plat_api = windows_plaform_api();

    // Crate tile instances

    let mut grass_inst = {
        let mut inst = TileInstance::new(TileType::Grass, GridPos::new(0, 0), TileMethods::Grass);
        inst.comp_harvest = Some(TileCompHarvest::new(
            Time::new(TimeUnit::Seconds(0.0)),
            FixedTableID::TestGold,
            false,
        ));
        inst
    };

    let mut spring_inst = tile_spring::new_instance(GridPos::new(1, 0));

    // insert grass into world
    let _ = world.insert_tile_instance(GridPos::new(0, 0), grass_inst);

    let world_snapshot = world.get_world_snapshot();

    // harvest grass
    let world_cell: WorldCell = world.get_entities(GridPos::new(0, 0));
    for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
        let tile = world.get_entity_mut(eid);
        tile.harvest(&world_snapshot, &plat_api);

        assert_eq!(tile.drops_queue.len(), 1);

        tile.drops_queue.clear();
    }

    // insert mud pit adjacent
    let _ = world.insert_tile_instance(GridPos::new(1, 0), spring_inst);

    let world_snapshot = world.get_world_snapshot();

    // harvest grass again. should have double the drops now
    let world_cell: WorldCell = world.get_entities(GridPos::new(0, 0));
    for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
        let tile = world.get_entity_mut(eid);
        tile.harvest(&world_snapshot, &plat_api);

        assert_eq!(tile.drops_queue.len(), 2);

        tile.drops_queue.clear();
    }
}
