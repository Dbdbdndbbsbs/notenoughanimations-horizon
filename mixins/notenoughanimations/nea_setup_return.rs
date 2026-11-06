// After the vanilla model setup: apply NEA's animations (settings and
// conditions are read by the animation half itself).
fn notenoughanimations_setup(
    m: &crate::mobs::Mob,
    s: &crate::mobs::anim::State,
    p: &mut crate::mobs::anim::Poses,
) {
    crate::mobs::anim::native_nea_anim::apply_all(p, s, m);
}
