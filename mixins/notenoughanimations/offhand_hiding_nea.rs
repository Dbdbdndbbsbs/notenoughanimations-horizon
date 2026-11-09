// Original NEA OffhandHidingAnimation: while the main hand holds a bow or
// crossbow (not a shield), the off hand's item is hidden; a charged crossbow
// hides even when idle, a bow only while drawing. Mirrored for a bow in the
// off hand. A bow paired with a crossbow hides too, as the crossbow's own
// item would. The flag is read by `hide_offhand_nea.rs` to suppress the
// vanilla render of the hidden item.
fn notenoughanimations_update_offhand_hiding(
    game: &crate::game::Game<'_>,
    mob: &crate::mobs::Mob,
) {
    use crate::game::client_ambient::native_nea_glue as glue;
    glue::set_hide_offhand(false);
    glue::set_hide_main(false);
    if !glue::settings(game).offhand_hiding {
        return;
    }
    let local_player = game
        .player_model
        .as_ref()
        .is_some_and(|p| p.id == mob.id && p.kind.id == mob.kind.id);
    let (main_item, off_item, main_charged, off_charged, using_main, using_off) =
        if local_player {
            let main = game.inventory[game.selected].as_ref();
            let off = game.offhand.as_ref();
            let using_slot = game.using.as_ref().map(|(slot, _)| *slot);
            let charged = |stack: Option<&crate::items::ItemStack>| {
                stack.is_some_and(|s| s.data().is_some_and(|d| !d.charged.is_empty()))
            };
            (
                main.map(|s| s.item.clone()),
                off.map(|s| s.item.clone()),
                charged(main),
                charged(off),
                using_slot == Some(game.selected),
                using_slot == Some(super::shield::OFFHAND_SLOT),
            )
        } else {
            let charged = mob
                .extra
                .main_hand_stack()
                .is_some_and(|s| s.data().is_some_and(|d| !d.charged.is_empty()));
            let using_off = mob.extra.using_item > 0 && mob.extra.using_off_hand;
            (
                mob.extra.main_hand.clone(),
                mob.extra.off_hand.clone(),
                charged,
                false,
                mob.extra.using_item > 0 && !mob.extra.using_off_hand,
                using_off,
            )
        };
    let Some(main_item) = main_item else { return };
    if &*main_item == "minecraft:shield" {
        return;
    }
    let is_bow =
        |item: Option<&str>| item.is_some_and(|i| matches!(i, "minecraft:bow" | "minecraft:crossbow"));
    let main_is_bow = is_bow(Some(&*main_item));
    let off_is_bow = is_bow(off_item.as_deref());
    let main_is_crossbow = &*main_item == "minecraft:crossbow";
    let off_is_crossbow = off_item.as_deref() == Some("minecraft:crossbow");
    if main_is_bow && off_is_bow {
        // NEA keeps a bow in each hand visible, but a crossbow still hides
        // the other hand's bow while it is charged or drawn.
        if main_is_crossbow && !off_is_crossbow && (main_charged || using_main) {
            glue::set_hide_offhand(true);
        } else if off_is_crossbow && !main_is_crossbow && (off_charged || using_off) {
            glue::set_hide_main(true);
        }
        return;
    }
    if main_is_bow {
        if (main_is_crossbow && main_charged) || using_main || using_off {
            glue::set_hide_offhand(true);
        }
        return;
    }
    if off_is_bow && ((off_is_crossbow && off_charged) || using_main || using_off) {
        glue::set_hide_main(true);
    }
}
