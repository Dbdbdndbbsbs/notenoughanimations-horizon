// NEA MapAnimation: the pose of a filled map in a player's third-person hand,
// from the original `MapAnimation.applyAnimation` and
// `MapRenderer.renderFirstPersonMap`. `hand` is `held_item_transform`'s matrix
// (the arm, then `ItemInHandLayer`'s offset), which this undoes to rebuild
// NEA's `translateToHand` chain.
use glam::{Mat4, Vec3};

/// `MapAnimation.applyAnimation` for one arm, then
/// `MapRenderer.renderFirstPersonMap`. `left` is the arm; `main_hand` says
/// whether it is the main arm; `off_hand_empty` picks the two-handed pose.
pub fn held_map_matrix(hand: Mat4, left: bool, main_hand: bool, off_hand_empty: bool) -> Mat4 {
    let deg = |d: f32| d.to_radians();
    let f = if left { -1.0f32 } else { 1.0 };
    // Undo `ItemInHandLayer`: `X-90 * Y180 * T(+-1/16, 2/16, -10/16)`.
    let undo = Mat4::from_translation(Vec3::new(-f / 16.0, -2.0 / 16.0, 10.0 / 16.0))
        * Mat4::from_rotation_y(deg(-180.0))
        * Mat4::from_rotation_x(deg(90.0));
    let (outer, inner) = if main_hand && off_hand_empty {
        // Two-handed (one map, empty off hand): the NEA main-hand outer chain
        // and the large inner variants 2/3, the pose confirmed level.
        (
            Mat4::from_rotation_x(deg(-90.0))
                * Mat4::from_rotation_y(deg(205.0))
                * Mat4::from_rotation_z(deg(10.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.24, -0.625)),
            if left {
                Mat4::from_rotation_y(deg(154.5))
                    * Mat4::from_rotation_z(deg(166.5))
                    * Mat4::from_scale(Vec3::splat(0.38))
                    * Mat4::from_translation(Vec3::new(0.585, -1.225, 0.15))
                    * Mat4::from_scale(Vec3::splat(0.013_812_5))
            } else {
                Mat4::from_rotation_y(deg(155.0))
                    * Mat4::from_rotation_z(deg(213.5))
                    * Mat4::from_scale(Vec3::splat(0.38))
                    * Mat4::from_translation(Vec3::new(-0.955, -1.8, 0.0))
                    * Mat4::from_scale(Vec3::splat(0.013_812_5))
            },
        )
    } else if left {
        // Left hand: NEA's small variant 1. The outer chain is the same in
        // both hands, so distance and size match.
        (
            Mat4::from_rotation_x(deg(-90.0))
                * Mat4::from_rotation_y(deg(200.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.125, -0.625)),
            Mat4::from_rotation_y(deg(160.0))
                * Mat4::from_rotation_z(deg(180.0))
                * Mat4::from_scale(Vec3::splat(0.38))
                * Mat4::from_translation(Vec3::new(-0.1, -1.2, 0.0))
                * Mat4::from_scale(Vec3::splat(0.009_812_5)),
        )
    } else {
        // Right hand: the same small pose as the left, with a small Z push
        // toward the face so both hands sit at the same distance.
        (
            Mat4::from_rotation_x(deg(-90.0))
                * Mat4::from_rotation_y(deg(200.0))
                * Mat4::from_translation(Vec3::new(f / 16.0, 0.125, -0.625)),
            Mat4::from_rotation_y(deg(160.0))
                * Mat4::from_rotation_z(deg(180.0))
                * Mat4::from_scale(Vec3::splat(0.38))
                * Mat4::from_translation(Vec3::new(-0.1, -1.2, -0.2))
                * Mat4::from_scale(Vec3::splat(0.009_812_5)),
        )
    };
    hand * undo * outer * inner
}
