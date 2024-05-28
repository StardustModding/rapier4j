use nalgebra::{Normed, RealField, Unit};
use rapier3d::{
    geometry::{ActiveCollisionTypes, ColliderBuilder, SharedShape, TriMeshFlags},
    math::{Isometry, Point},
    parry::transformation::vhacd::VHACDParameters,
    pipeline::{ActiveEvents, ActiveHooks},
};

use crate::{tuple::Tuple, Triple, Vector3};

pub struct ColliderBuilderCustom;

impl ColliderBuilderCustom {
    pub fn compound(shapes: Vec<Tuple<Vector3, SharedShape>>) -> ColliderBuilder {
        ColliderBuilder::compound(
            shapes
                .iter()
                .map(|v| {
                    let it = v.as_rust();

                    (it.0.into(), it.1)
                })
                .collect::<Vec<(Isometry<f32>, SharedShape)>>(),
        )
    }

    pub fn polyline(
        vertices: Vec<Vector3>,
        indices: Option<Vec<Tuple<u32, u32>>>,
    ) -> ColliderBuilder {
        ColliderBuilder::polyline(
            vertices.iter().map(|v| v.into()).collect(),
            indices.map(|v| v.iter().map(|v| v.as_slice()).collect()),
        )
    }

    pub fn trimesh(vertices: Vec<Vector3>, indices: Vec<Triple<u32, u32, u32>>) -> ColliderBuilder {
        ColliderBuilder::trimesh(
            vertices.iter().map(|v| v.into()).collect(),
            indices.iter().map(|v| v.as_slice()).collect(),
        )
    }

    pub fn trimesh_with_flags(
        vertices: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
        flags: TriMeshFlags,
    ) -> ColliderBuilder {
        ColliderBuilder::trimesh_with_flags(
            vertices.iter().map(|v| v.into()).collect(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>(),
            flags,
        )
    }

    pub fn convex_decomposition(
        vertices: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
    ) -> ColliderBuilder {
        ColliderBuilder::convex_decomposition(
            vertices.iter().map(|v| v.into()).collect().as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
        )
    }

    pub fn round_convex_decomposition(
        vertices: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
        border_radius: f32,
    ) -> ColliderBuilder {
        ColliderBuilder::round_convex_decomposition(
            vertices.iter().map(|v| v.into()).collect().as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            border_radius,
        )
    }

    pub fn convex_decomposition_with_params(
        vertices: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
        params: &VHACDParameters,
    ) -> ColliderBuilder {
        ColliderBuilder::convex_decomposition_with_params(
            vertices.iter().map(|v| v.into()).collect().as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            params,
        )
    }

    pub fn round_convex_decomposition_with_params(
        vertices: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
        params: &VHACDParameters,
        border_radius: f32,
    ) -> ColliderBuilder {
        ColliderBuilder::round_convex_decomposition_with_params(
            vertices.iter().map(|v| v.into()).collect().as_slice(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            params,
            border_radius,
        )
    }

    pub fn convex_hull(points: Vec<Vector3>) -> Option<ColliderBuilder> {
        ColliderBuilder::convex_hull(points.iter().map(|v| v.into()).collect().as_slice())
    }

    pub fn round_convex_hull(points: Vec<Vector3>, border_radius: f32) -> Option<ColliderBuilder> {
        ColliderBuilder::round_convex_hull(
            points.iter().map(|v| v.into()).collect().as_slice(),
            border_radius,
        )
    }

    pub fn convex_mesh(
        points: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
    ) -> Option<ColliderBuilder> {
        ColliderBuilder::convex_mesh(
            points.iter().map(|v| v.into()).collect(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
        )
    }

    pub fn round_convex_mesh(
        points: Vec<Vector3>,
        indices: Vec<Triple<u32, u32, u32>>,
        border_radius: f32,
    ) -> Option<ColliderBuilder> {
        ColliderBuilder::round_convex_mesh(
            points.iter().map(|v| v.into()).collect(),
            indices
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<[u32; 3]>>()
                .as_slice(),
            border_radius,
        )
    }
}

pub struct UnitHelper<T, N> {
    _unused: T,
    _unused2: N,
}

impl<T, N> UnitHelper<T, N>
where
    T: Normed<Norm = N> + Clone,
    N: RealField + Clone,
{
    pub fn new_and_get(value: T) -> Tuple<Unit<T>, N> {
        Tuple::from_rust(Unit::new_and_get(value))
    }

    pub fn try_new_and_get(value: T, min_norm: N) -> Option<Tuple<Unit<T>, N>> {
        Unit::try_new_and_get(value, min_norm).map(Tuple::from_rust)
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
