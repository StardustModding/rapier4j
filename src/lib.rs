pub mod patches;
pub mod tuple;
pub mod util;
pub mod wrappers;

use nalgebra::{
    ArrayStorage, Const, Matrix, OPoint, Translation as RTranslation, Vector3 as NVector3,
};
use parry3d::transformation::vhacd::VHACDParameters;
use patches::*;
use rapier3d::prelude::{
    Aabb, ActiveCollisionTypes, ActiveEvents, ActiveHooks, AngVector, CoefficientCombineRule,
    Collider, ColliderBuilder, ColliderMaterial, DMatrix, InteractionGroups, Isometry,
    MassProperties, RigidBodyHandle, Rotation, Shape as RShape, SharedShape, TriMeshFlags, Vector,
};
use tuple::*;
use wrappers::*;

pub type Optional<T> = Option<T>;
pub type List<T> = Vec<T>;
pub type Translation<T> = RTranslation<T, 3>;
pub type OPoint3<T> = OPoint<T, Const<3>>;
pub type Matrix3<T> = Matrix<T, Const<3>, Const<1>, ArrayStorage<T, 3, 1>>;
pub type Shape = Box<dyn RShape>;

include!("bindings.rs");
