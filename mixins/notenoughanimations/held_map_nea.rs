// NEA MapRendering: draws a filled map held in a player's or mannequin's hand
// instead of the flat item icon, and an empty map as its paper. Inlined at the
// start of `held_item_geometry`; `map_hide_nea` then makes the resolver return
// an empty state for the same third-person map so the flat icon is not drawn.
fn notenoughanimations_held_map(
    game: &crate::game::Game<'_>,
    mob: &crate::mobs::Mob,
    camera: glam::DVec3,
    alpha: f64,
    light: u32,
    out: &mut Vec<crate::mesher::Vertex>,
) {
    use crate::game::client_ambient::native_nea_glue as glue;
    if !glue::settings(game).maps {
        return;
    }
    if &*mob.kind.id != "minecraft:player"
        && mob.species() != crate::mobs::Species::Mannequin
    {
        return;
    }
    let local_player = game
        .player_model
        .as_ref()
        .is_some_and(|p| p.id == mob.id && p.kind.id == mob.kind.id);
    let data = game.res.mob_data();
    for (item, off_hand) in [
        (&mob.extra.main_hand, false),
        (&mob.extra.off_hand, true),
    ] {
        let Some(item) = item else { continue };
        // An empty map stays vanilla: a flat item, never the 3D map render.
        if &**item == "minecraft:map" {
            continue;
        }
        let left = off_hand != mob.main_arm_left();
        let stack = if local_player {
            let Some(stack) = game.stack_in(if off_hand {
                super::shield::OFFHAND_SLOT
            } else {
                game.selected
            }) else {
                continue;
            };
            stack.clone()
        } else {
            let Some(stack) =
                crate::mobs::render::held_item_stack(&mob.extra, off_hand)
            else {
                continue;
            };
            stack
        };
        let main_hand = !off_hand;
        let off_hand_empty = main_hand && mob.extra.off_hand.is_none();
        let Some(hand) =
            crate::mobs::render::held_item_transform(mob, alpha as f32, camera, data, left)
        else {
            continue;
        };
        let m = crate::mobs::render::native_nea_render::held_map_matrix(
            hand,
            left,
            main_hand,
            off_hand_empty,
        );
        let Some(id) = stack.data().and_then(|d| d.map_id) else {
            continue;
        };
        let Some(rect) = game.map_rect(id) else {
            continue;
        };
        game.map_quads(id, rect, false, m, light, out);
    }
}
