use rapier3d::{
    geometry::{ActiveCollisionTypes, ColliderBuilder, SharedShape, TriMeshFlags},
    math::{Isometry, Point},
    parry::transformation::vhacd::VHACDParameters, pipeline::{ActiveEvents, ActiveHooks},
};

use crate::{tuple::Tuple, Triple};

pub struct ColliderBuilderCustom;

impl ColliderBuilderCustom {
    pub fn compound(shapes: Vec<Tuple<Isometry<f32>, SharedShape>>) -> ColliderBuilder {
        ColliderBuilder::compound(
            shapes
                .iter()
                .map(|v| v.as_rust())
                .collect::<Vec<(Isometry<f32>, SharedShape)>>(),
        )
    }

    pub fn polyline(
        vertices: Vec<Point<f32>>,
        indices: Option<Vec<Tuple<u32, u32>>>,
    ) -> ColliderBuilder {
        ColliderBuilder::polyline(
            vertices,
            indices.map(|v| v.iter().map(|v| v.as_slice()).collect()),
        )
    }

    pub fn trimesh(
        vertices: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
    ) -> ColliderBuilder {
        ColliderBuilder::trimesh(vertices, indices.iter().map(|v| v.as_slice()).collect())
    }

    pub fn trimesh_with_flags(
        vertices: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
        flags: TriMeshFlags,
    ) -> ColliderBuilder {
        ColliderBuilder::trimesh_with_flags(
            vertices,
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>(),
            flags,
        )
    }

    pub fn convex_decomposition(
        vertices: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
    ) -> ColliderBuilder {
        ColliderBuilder::convex_decomposition(
            vertices.as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
        )
    }

    pub fn round_convex_decomposition(
        vertices: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
        border_radius: f32,
    ) -> ColliderBuilder {
        ColliderBuilder::round_convex_decomposition(
            vertices.as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            border_radius,
        )
    }

    pub fn convex_decomposition_with_params(
        vertices: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
        params: &VHACDParameters,
    ) -> ColliderBuilder {
        ColliderBuilder::convex_decomposition_with_params(
            vertices.as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            params,
        )
    }

    pub fn round_convex_decomposition_with_params(
        vertices: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
        params: &VHACDParameters,
        border_radius: f32,
    ) -> ColliderBuilder {
        ColliderBuilder::round_convex_decomposition_with_params(
            vertices.as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            params,
            border_radius,
        )
    }

    pub fn convex_hull(points: Vec<Point<f32>>) -> Option<ColliderBuilder> {
        ColliderBuilder::convex_hull(points.as_slice())
    }

    pub fn round_convex_hull(
        points: Vec<Point<f32>>,
        border_radius: f32,
    ) -> Option<ColliderBuilder> {
        ColliderBuilder::round_convex_hull(points.as_slice(), border_radius)
    }

    pub fn convex_mesh(
        points: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
    ) -> Option<ColliderBuilder> {
        ColliderBuilder::convex_mesh(
            points,
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
        )
    }

    pub fn round_convex_mesh(
        points: Vec<Point<f32>>,
        indices: Vec<Triple<u32, u32, u32>>,
        border_radius: f32,
    ) -> Option<ColliderBuilder> {
        ColliderBuilder::round_convex_mesh(
            points,
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            border_radius,
        )
    }
}

pub struct BitflagsHelper;

impl BitflagsHelper {
    // ============ ActiveEvents ============

    pub fn active_events_collison_events() -> u32 {
        ActiveEvents::COLLISION_EVENTS.bits()
    }

    pub fn active_events_contact_force_events() -> u32 {
        ActiveEvents::CONTACT_FORCE_EVENTS.bits()
    }

    // ============ ActiveHooks ============

    pub fn active_hooks_filter_contact_pairs() -> u32 {
        ActiveHooks::FILTER_CONTACT_PAIRS.bits()
    }

    pub fn active_hooks_filter_intersection_pair() -> u32 {
        ActiveHooks::FILTER_INTERSECTION_PAIR.bits()
    }

    pub fn active_hooks_modify_solver_contacts() -> u32 {
        ActiveHooks::MODIFY_SOLVER_CONTACTS.bits()
    }

    // ============ ActiveCollisionTypes ============

    pub fn active_collision_types_dynamic_dynamic() -> u16 {
        ActiveCollisionTypes::DYNAMIC_DYNAMIC.bits()
    }

    pub fn active_collision_types_dynamic_kinematic() -> u16 {
        ActiveCollisionTypes::DYNAMIC_KINEMATIC.bits()
    }

    pub fn active_collision_types_dynamic_fixed() -> u16 {
        ActiveCollisionTypes::DYNAMIC_FIXED.bits()
    }

    pub fn active_collision_types_kinematic_kinematic() -> u16 {
        ActiveCollisionTypes::KINEMATIC_KINEMATIC.bits()
    }

    pub fn active_collision_types_kinematic_fixed() -> u16 {
        ActiveCollisionTypes::KINEMATIC_FIXED.bits()
    }

    pub fn active_collision_types_fixed_fixed() -> u16 {
        ActiveCollisionTypes::FIXED_FIXED.bits()
    }

}
