// At the start of the model setup: capture the player's animation state, to
// be applied after the vanilla pose (the model-visibility hook).
fn notenoughanimations_setup_head(m: &crate::mobs::Mob, s: &crate::mobs::anim::State) {
    crate::mobs::anim::native_nea_anim::set_state(
        crate::mobs::anim::native_nea_anim::NeaState::capture(m, s),
    );
}
