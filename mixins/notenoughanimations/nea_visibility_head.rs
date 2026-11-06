// After the vanilla `humanoid` pose (this hook runs on the player model's
// visibility call, right after it): apply NEA's animations.
fn notenoughanimations_visibility_head(p: &mut crate::mobs::anim::Poses) {
    crate::mobs::anim::native_nea_anim::apply_all(p);
}
