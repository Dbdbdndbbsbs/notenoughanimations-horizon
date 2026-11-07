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
    });

    // NEA FreezingAnimation: the local player stands in powder snow (the
    // game already tracks it on `PlayerMotion`). BurningAnimation reads the
    // local player's remaining fire ticks, since the model's own field is
    // only synced for remote players.
    let Some(p) = game.player.as_ref() else {
        anim::set_powder_snow(false);
        anim::set_fire_ticks(0);
        anim::reset_animation_state();
        return;
    };
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
