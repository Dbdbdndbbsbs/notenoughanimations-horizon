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
    // game already tracks it on `PlayerMotion`).
    anim::set_powder_snow(game.player.as_ref().is_some_and(|p| p.in_powder_snow));
}
