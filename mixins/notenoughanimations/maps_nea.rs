// NEA MapRendering: `map_rect` looks up a warmed map's atlas rect on `&self`
// (the entity pass cannot upload); `map_quads` draws the map texture and its
// vanilla decorations at an already-computed pose. `map_paper_quads` draws
// just the paper for an empty map, as vanilla and NEA show it.
use crate::game::Game;
use crate::game::maps::{shown_on_frame, decoration_pose};
use crate::hud::{Rect, map_slot_sprite};
use crate::mesher::Vertex;
use glam::Mat4;

/// One quad of a map face, at `z` under the pose `m`.
fn push_quad(
    m: Mat4,
    corners: [[f32; 3]; 4],
    uv: [[f32; 2]; 4],
    light: u32,
    out: &mut Vec<Vertex>,
) {
    for (c, uv) in corners.into_iter().zip(uv) {
        out.push(Vertex {
            normal: [0; 4],
            pos: m.transform_point3(glam::Vec3::from_array(c)).to_array(),
            uv,
            layer: 0,
            color: 0xFFFF_FFFF,
            light,
            glint_uv: [0.0, 0.0],
        });
    }
}

/// The reverse-wound back face: mirrors U so the paper reads correctly from
/// behind.
fn push_back_quad(
    m: Mat4,
    corners: [[f32; 3]; 4],
    uv: [[f32; 2]; 4],
    u0: f32,
    u1: f32,
    light: u32,
    out: &mut Vec<Vertex>,
) {
    for i in [3, 2, 1, 0] {
        let c = corners[i];
        let [u, v] = uv[i];
        out.push(Vertex {
            normal: [0; 4],
            pos: m.transform_point3(glam::Vec3::from_array(c)).to_array(),
            uv: [u0 + u1 - u, v],
            layer: 0,
            color: 0xFFFF_FFFF,
            light,
            glint_uv: [0.0, 0.0],
        });
    }
}

impl Game<'_> {
    /// The atlas rect of map `id` once [`Game::map_texture`] uploaded it: the
    /// entity pass draws on `&self` and cannot upload.
    pub(super) fn map_rect(&self, id: i32) -> Option<Rect> {
        if !self.maps.data.contains_key(&id) {
            return None;
        }
        let slot = self.maps.slots.slot(&id)?;
        self.gui.atlas.sprites.get(map_slot_sprite(slot)?).copied()
    }

    /// The two paper faces, front at `-0.005` and back at `+0.005`, for an
    /// already-computed pose.
    fn paper_quads(&self, m: Mat4, light: u32, out: &mut Vec<Vertex>) {
        let Some(r) = self.gui.atlas.sprites.get("map/map_background").copied() else {
            return;
        };
        let corners = [
            [-7.0, 135.0, -0.005],
            [135.0, 135.0, -0.005],
            [135.0, -7.0, -0.005],
            [-7.0, -7.0, -0.005],
        ];
        let uv = [
            [r.u0, r.v1],
            [r.u1, r.v1],
            [r.u1, r.v0],
            [r.u0, r.v0],
        ];
        push_quad(m, corners, uv, light, out);
        push_back_quad(m, corners.map(|c| [c[0], c[1], 0.005]), uv, r.u0, r.u1, light, out);
    }

    /// [`Game::map_vertices`]' texture and decorations for a map already in
    /// the atlas, drawn where only `&self` is available.
    pub(super) fn map_quads(
        &self,
        id: i32,
        rect: Rect,
        show_only_frame: bool,
        m: Mat4,
        light: u32,
        out: &mut Vec<Vertex>,
    ) {
        // NEA `MapRenderer.renderFirstPersonMap`: a map with data shows its
        // texture over the checkerboard; one without is just paper.
        let has_data = self.maps.data.contains_key(&id);
        if has_data {
            if let Some(r) = self
                .gui
                .atlas
                .sprites
                .get("map/map_background_checkerboard")
                .copied()
            {
                let corners = [
                    [-7.0, 135.0, -0.005],
                    [135.0, 135.0, -0.005],
                    [135.0, -7.0, -0.005],
                    [-7.0, -7.0, -0.005],
                ];
                let uv = [
                    [r.u0, r.v1],
                    [r.u1, r.v1],
                    [r.u1, r.v0],
                    [r.u0, r.v0],
                ];
                push_quad(m, corners, uv, light, out);
                push_back_quad(
                    m,
                    corners.map(|c| [c[0], c[1], 0.005]),
                    uv,
                    r.u0,
                    r.u1,
                    light,
                    out,
                );
            }
            let corners = [
                [0.0, 128.0, -0.04],
                [128.0, 128.0, -0.04],
                [128.0, 0.0, -0.04],
                [0.0, 0.0, -0.04],
            ];
            let uv = [
                [rect.u0, rect.v1],
                [rect.u1, rect.v1],
                [rect.u1, rect.v0],
                [rect.u0, rect.v0],
            ];
            push_quad(m, corners, uv, light, out);
        } else {
            self.paper_quads(m, light, out);
        }
        let decorations = self
            .maps
            .data
            .get(&id)
            .map_or(&[][..], |m| &m.decorations);
        let mut count = 0;
        for d in decorations {
            if show_only_frame && !shown_on_frame(d.sprite) {
                continue;
            }
            let Some(s) = self.gui.atlas.sprites.get(d.sprite).copied() else {
                continue;
            };
            let dm = decoration_pose(d, -0.06 - count as f32 * 0.001);
            let p = |x: f32, y: f32| dm.transform_point3(glam::Vec3::new(x, y, 0.0)).to_array();
            let mut q = |corners: [[f32; 3]; 4], uv: [[f32; 2]; 4]| {
                for (c, uv) in corners.into_iter().zip(uv) {
                    out.push(Vertex {
                        normal: [0; 4],
                        pos: m.transform_point3(glam::Vec3::from_array(c)).to_array(),
                        uv,
                        layer: 0,
                        color: 0xFFFF_FFFF,
                        light,
                        glint_uv: [0.0, 0.0],
                    });
                }
            };
            q(
                [p(-1.0, 1.0), p(1.0, 1.0), p(1.0, -1.0), p(-1.0, -1.0)],
                [[s.u0, s.v0], [s.u1, s.v0], [s.u1, s.v1], [s.u0, s.v1]],
            );
            count += 1;
        }
    }
}
