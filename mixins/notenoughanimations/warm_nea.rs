// NEA MapRendering: uploads map textures for all held maps before the mob
// geometry pass draws them. Inlined at the start of `entity_geometry`.
fn notenoughanimations_warm_held_maps(game: &mut crate::game::Game<'_>) {
    let mut ids = Vec::new();
    {
        let mut note = |stack: Option<&crate::items::ItemStack>| {
            if let Some(id) = stack.and_then(|s| s.data()).and_then(|d| d.map_id) {
                ids.push(id);
            }
        };
        for mob in &game.mobs {
            for off_hand in [false, true] {
                let stack =
                    horizon_client::mobs::render::held_item_stack(&mob.extra, off_hand);
                note(stack.as_ref());
            }
        }
        note(game.stack_in(game.selected));
        note(game.stack_in(crate::game::shield::OFFHAND_SLOT));
    }
    for id in ids {
        let _ = game.map_texture(id);
    }
}
