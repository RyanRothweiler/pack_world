pub use crate::{
    assets::*, grid::*, inventory::*, item::*, state::*, tile::*, update_signal::*, world::*,
};
pub use elara_engine::{
    collisions::*,
    color::*,
    input::{Input, KeyCode},
    matricies::*,
    platform_api::*,
    rect::*,
    render::{camera::*, material::*, render_command::*, render_pack::*, *},
    state::State as EngineState,
    transform::*,
    ui::*,
    vectors::*,
};

#[derive(Debug)]
pub struct GameModeWorld {
    rotate_time: f64,
    tile_placing: Option<TileType>,

    // For paint mode
    prev_pos_placed: Option<GridPos>,
}

impl GameModeWorld {
    pub fn new() -> Self {
        Self {
            rotate_time: 0.0,
            tile_placing: None,
            prev_pos_placed: None,
        }
    }

    pub fn update(
        &mut self,
        prev_delta_time: f64,
        es: &mut EngineState,
        input: &mut Input,
        render_api: &mut impl RenderApi,
        platform_api: &PlatformApi,
        world: &mut World,
        assets: &mut Assets,
        inventory: &mut Inventory,
        ui_context: &mut UIContext,
        mut ui_frame_state: &mut UIFrameState,
    ) -> Vec<UpdateSignal> {
        let mut ret: Vec<UpdateSignal> = vec![];

        // camera controls
        {
            let cam_pack = es
                .render_system
                .render_packs
                .get_mut(&RenderPackID::NewWorld)
                .unwrap();

            cam_pack.camera.move_plane(false, input, prev_delta_time);
            cam_pack.camera.update_position(prev_delta_time);

            /*
            es.render_system
                .render_packs
                .get_mut(&RenderPackID::NewWorld)
                .unwrap()
                .camera
                .move_fly(0.3, input);
            */
        }

        // render tiles
        {
            // TODO chagne this to use delta_time
            self.rotate_time += 0.08;

            for (grid_pos, world_cell) in &world.entity_map {
                for (layer, eid) in &world_cell.layers {
                    // Skip ground tils if there is a floor
                    if *layer == WorldLayer::Ground {
                        if world_cell.layers.contains_key(&WorldLayer::Floor) {
                            continue;
                        }
                    }

                    let entity = &world.get_entity(&eid);

                    entity.render(
                        self.rotate_time,
                        &entity.grid_pos,
                        es.color_texture_shader,
                        es.render_system
                            .render_packs
                            .get_mut(&RenderPackID::NewWorld)
                            .unwrap(),
                        &assets,
                    );
                }
            }
        }

        // Get mouse grid position
        let mouse_grid: GridPos = {
            let mut val = GridPos::new(0, 0);

            let cam: &Camera = &es
                .render_system
                .render_packs
                .get(&RenderPackID::NewWorld)
                .unwrap()
                .camera;
            let pos = cam.screen_to_world(input.mouse.pos);

            let dir = (pos - cam.transform.local_position).normalize();

            if let Some(len) = plane_intersection_distance(
                cam.transform.local_position,
                dir,
                VecThreeFloat::new(0.0, 0.0, 0.0),
                VecThreeFloat::new(0.0, -1.0, 0.0),
            ) {
                let world_pos = cam.transform.local_position + (dir * len);
                val = world_to_grid(&world_pos.xz());
            }

            val
        };

        // don't allow tile placing if over selection UI
        if input.mouse.pos.x > 300.0 {
            // placing tiles
            if let Some(tile) = self.tile_placing {
                // escape key reseting
                if input.keyboard.get_key(KeyCode::Escape).on_press {
                    self.tile_placing = None;
                }

                // render tile placing
                if tile.get_definition().placing_draw_footprint {
                    let footprint = &tile.get_definition().footprint;

                    for p in footprint {
                        let pos = mouse_grid + *p;

                        let can_place = tile.pos_passes_placement_constraints(pos, &world);

                        draw_tile_grid_pos(
                            tile,
                            0.0,
                            &pos,
                            can_place,
                            es.render_system
                                .render_packs
                                .get_mut(&RenderPackID::NewWorld)
                                .unwrap(),
                            &assets,
                        );
                    }
                } else {
                    let can_place = tile.can_place_here(mouse_grid, &world);

                    draw_tile_grid_pos(
                        tile,
                        0.0,
                        &mouse_grid,
                        can_place,
                        es.render_system
                            .render_packs
                            .get_mut(&RenderPackID::NewWorld)
                            .unwrap(),
                        &assets,
                    );
                }

                // place tile
                let can_place = tile.can_place_here(mouse_grid, &world);
                let mut want_place = input.mouse.button_left.pressing;

                // check for painting
                if want_place {
                    match self.prev_pos_placed {
                        Some(pos) => {
                            if pos == mouse_grid {
                                want_place = false;
                            }
                        }
                        None => {}
                    }
                }

                if want_place && can_place {
                    self.prev_pos_placed = Some(mouse_grid);

                    if let Ok(update_sigs) = world.try_place_tile(mouse_grid, tile) {
                        let count = inventory.give_item(ItemType::Tile(tile), -1).unwrap();
                        if count == 0 {
                            self.tile_placing = None;
                        }

                        let mut us: Vec<UpdateSignal> = update_sigs;
                        ret.append(&mut us);
                    }
                }

                if !input.mouse.button_left.pressing {
                    self.prev_pos_placed = None;
                }
            }
        }

        // tile hovering
        {
            if self.tile_placing.is_none() {
                let world_cell: WorldCell = world.get_entities(mouse_grid);
                let world_snapshot = world.get_world_snapshot();

                for (i, (layer, eid)) in world_cell.layers.iter().enumerate() {
                    let tile = world.get_entity_mut(eid);

                    // Harvesting
                    if input.mouse.button_left.pressing && tile.can_harvest() {
                        tile.harvest(&world_snapshot, platform_api);
                    }

                    // render hover rect
                    {
                        let mut mat = Material::new();
                        mat.shader = Some(es.shader_color);
                        mat.set_color(Color::new(1.0, 1.0, 1.0, 0.8));

                        let mut trans = Transform::new();
                        trans.local_position = grid_to_world(&mouse_grid);
                        trans.update_global_matrix(&M44::new_identity());

                        es.render_system
                            .render_packs
                            .get_mut(&RenderPackID::NewWorld)
                            .unwrap()
                            .commands
                            .push(RenderCommand::new_model(
                                &trans,
                                assets.asset_library.get_model("tile_outline"),
                                &mat,
                            ));
                    }

                    // render info
                    {
                        let mut ui_frame_state = UIFrameState::new(&input, es.window_resolution);

                        let y = layer.to_index() as f64 * 40.0;

                        draw_text(
                            &format!("{:?}", tile.tile_type),
                            VecTwo::new(450.0, 100.0 + y),
                            COLOR_WHITE,
                            &ui_context.font_body.clone(),
                            &mut ui_frame_state,
                            ui_context,
                        );

                        tile.render_hover_info(
                            y,
                            es.shader_color.clone(),
                            es.render_system
                                .render_packs
                                .get_mut(&RenderPackID::UI)
                                .unwrap(),
                            ui_context,
                            &mut ui_frame_state,
                        );
                    }
                }
            }
        }

        // tile selecting
        {
            let icon_size: f64 = 100.0;
            let gutter: f64 = 100.0;

            begin_panel(
                Rect::new(VecTwo::new(0.0, 57.0), VecTwo::new(250.0, 4000.0)),
                Color::new(0.0, 0.0, 0.0, 0.70),
                ui_frame_state,
                ui_context,
            );
            {
                let mut i: i32 = 0;
                let tile_inv = inventory.get_all_tiles();
                for (item_type, count) in tile_inv {
                    let disp = format!("{count}");
                    let icon = assets.get_item_icon(item_type);

                    match item_type {
                        ItemType::Tile(tile_type) => {
                            if let Some(tile_thumbnail) = assets.get_tile_thumbnail(tile_type) {
                                let mut r = Rect::new_size(icon_size, icon_size);
                                r.translate(VecTwo::new(70.0, icon_size * 0.5));
                                r.translate(VecTwo::new(
                                    (i as f64 % 2.0) * gutter,
                                    (i / 2) as f64 * gutter,
                                ));

                                if draw_button_id(
                                    i,
                                    &disp,
                                    ButtonStyleData::new_shrink(None, Some(tile_thumbnail), 4.0),
                                    &r,
                                    ui_frame_state,
                                    std::line!(),
                                    ui_context,
                                ) {
                                    self.tile_placing = Some(*tile_type);
                                }
                            }
                        }
                        _ => {}
                    };

                    i += 1;
                }
            }
            end_panel(&mut ui_frame_state, ui_context);
        }

        return ret;
    }
}
