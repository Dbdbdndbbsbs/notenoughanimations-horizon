// The frame hook: refresh the mod's settings (and the local player's
// powder-snow flag) for the animation half, once a frame.
fn notenoughanimations_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::client_ambient::native_nea_glue as glue;
    use crate::game::client_ambient::native_nea_runtime as m;
    use crate::mobs::anim::native_nea_anim as anim;

    let s = glue::settings(game);
    anim::set_nea(anim::NeaSettings {
        master: s.enabled,
        eat_drink: s.eat_drink,
        ladder: s.ladder,
        boat: s.boat,
        item_swap: s.item_swap,
        burning: s.burning,
        freezing: s.freezing,
        crawling: s.crawling,
        sleep: s.sleep,
        maps: s.maps,
        offhand_hiding: s.offhand_hiding,
    });

    let Some(p) = game.player.as_mut() else {
        anim::set_powder_snow(false);
        anim::set_fire_ticks(0);
        anim::set_climbing(false);
        anim::set_ladder_lock(false);
        anim::set_burning(false);
        anim::set_freezing(false);
        anim::reset_animation_state();
        return;
    };

    // NEA LadderAnimation: onClimbable && !onGround && lastClimbable is a
    // ladder or trapdoor. `last_climbable` is set only when the player is on
    // a climbable block, so its presence stands in for `climbable()`.
    // `PlayerMotion.on_climbable` is the client's own `onClimbable()` result
    // for this tick; not on the ground, it means the player is climbing
    // (matching the original's `onClimbable() && !onGround`).
    let climbing = p.on_climbable && !p.ground.on_ground;
    // The body locks to the ladder's own block: its `facing` points away from
    // the wall, so the player faces the opposite way. `last_climbable` is the
    // block the player occupies, so its position is the ladder itself. Vines
    // and scaffolding have no `facing` and keep the vanilla body and head.
    let mut ladder_face = 0.0;
    let mut ladder_lock = false;
    if climbing
        && let Some(pos) = p.last_climbable
    {
        let id = game.world.block(pos.x, pos.y, pos.z);
        if let Some(state) = game.pack.assets.registry.state(id)
            && let Some(facing) = state.properties.get("facing")
        {
            ladder_face = match facing.as_str() {
                "north" => 0.0,
                "south" => std::f32::consts::PI,
                "west" => 3.0 * std::f32::consts::FRAC_PI_2,
                "east" => std::f32::consts::FRAC_PI_2,
                _ => 0.0,
            };
            ladder_lock = true;
        }
    }
    anim::set_climbing(climbing);
    anim::set_ladder_face(ladder_face);
    anim::set_ladder_lock(ladder_lock);
    // The model's world yaw is what actually turns the body: lock it to the
    // ladder so the player's own turning does not rotate it.
    if climbing && ladder_lock && s.enabled && s.ladder {
        if let Some(m) = game.player_model.as_mut() {
            let deg = ladder_face.to_degrees();
            m.body_yaw = deg;
            m.previous_body_yaw = deg;
        }
    }

    // NEA FreezingAnimation: in powder snow and able to freeze.
    let freezing = p.in_powder_snow && p.can_freeze();
    anim::set_freezing(freezing);

    // NEA BurningAnimation: on fire without fire resistance.
    let burning = p.fire_ticks > 0 && !p.status.fire_resistance;
    anim::set_burning(burning);

    // NEA FreezingAnimation: the local player stands in powder snow (the
    // game already tracks it on `PlayerMotion`). BurningAnimation reads the
    // local player's remaining fire ticks, since the model's own field is
    // only synced for remote players.
    anim::set_powder_snow(p.in_powder_snow);
    anim::set_fire_ticks(p.fire_ticks);

    // NEA ItemSwapAnimation: the hands traded since the last frame.
    let main = game.inventory[game.selected]
        .as_ref()
        .map(|s| s.item.clone());
    let off = game.offhand.as_ref().map(|s| s.item.clone());
    let age = game.player_model.as_ref().map_or(0, |m| m.age);
    anim::update_hands(age as f32, main, off);

    // NEA BoatAnimation: the boat the local player rides, and its paddles.
    let boat = game
        .riding
        .and_then(|id| game.boats.iter().find(|b| b.id == id))
        .map(|b| (b.paddling, b.paddles));
    anim::update_boat(boat);
}
