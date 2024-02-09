pub mod patches;
pub mod tuple;
pub mod util;

use nalgebra::{
    ArrayStorage, Const, Matrix, OPoint, Quaternion, RealField, Scalar,
    Translation as RTranslation, Unit,
};
use parry3d::transformation::vhacd::VHACDParameters;
use patches::*;
use rapier3d::prelude::{
    ActiveCollisionTypes, ActiveEvents, ActiveHooks, AngVector, CoefficientCombineRule, Collider,
    ColliderBuilder, DMatrix, InteractionGroups, Isometry, MassProperties, Point, Rotation,
    SharedShape, TriMeshFlags, Vector,
};
use tuple::*;

pub type Optional<T> = Option<T>;
pub type List<T> = Vec<T>;
pub type Translation<T> = RTranslation<T, 3>;
pub type OPoint3<T> = OPoint<T, Const<3>>;
pub type Matrix3<T> = Matrix<T, Const<3>, Const<1>, ArrayStorage<T, 3, 1>>;

include!("bindings.rs");
