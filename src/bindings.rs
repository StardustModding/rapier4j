#[allow(unused_imports)]
use jni::sys::{
    jarray, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jobject, jshort, jstring,
    jvalue,
};

use jni::{
    objects::{JObject, JValueGen},
    JNIEnv,
};

#[cfg(target_pointer_width = "32")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}

#[allow(dead_code)]
fn object_to_jobject<T>(env: *mut JNIEnv, obj: T, jcls: String) -> jobject {
    let jobj: JObject = unsafe { (*env).alloc_object(jcls).unwrap() };

    assert!(!jobj.is_null(), "object_to_jobject: AllocObject failed");

    let ret: jlong = Box::into_raw(Box::new(obj)) as jlong;

    unsafe {
        let res = (*env).set_field(&jobj, "__pointer", "Long", JValueGen::Long(ret));

        if (*env).exception_check().unwrap() || res.is_err() {
            panic!("object_to_jobject: Can not set mNativeObj field: catch exception");
        }
    }

    jobj.as_raw()
}


pub trait IntoJavaType {
    fn into_java_type(&self) -> String;
}

macro_rules! from_type {
    ($ty: ident, $name: ident) => {
        impl From<$ty> for RustTypes {
            fn from(val: $ty) -> RustTypes {
                RustTypes::$name(val)
            }
        }

        impl Into<$ty> for RustTypes {
            fn into(self) -> $ty {
                if let Self::$name(val) = self {
                    val
                } else {
                    panic!("Expected RustTypes::{}, got {:?}", stringify!($name), self)
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub enum RustTypes {
    String(String),
    Bool(bool),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    Other(String),

    #[default]
    Void,
}

from_type!(String, String);
from_type!(bool, Bool);
from_type!(u8, Uint8);
from_type!(u16, Uint16);
from_type!(u32, Uint32);
from_type!(u64, Uint64);
from_type!(u128, Uint128);
from_type!(i8, Int8);
from_type!(i16, Int16);
from_type!(i32, Int32);
from_type!(i64, Int64);
from_type!(i128, Int128);
from_type!(f32, Float32);
from_type!(f64, Float64);

impl From<&str> for RustTypes {
    fn from(val: &str) -> Self {
        match val {
            "String" | "str" => Self::String(String::new()),
            "bool" => Self::Bool(false),
            "i8" => Self::Int8(0),
            "i16" => Self::Int16(0),
            "i32" => Self::Int32(0),
            "i64" => Self::Int64(0),
            "i128" => Self::Int128(0),
            "u8" => Self::Uint8(0),
            "u16" => Self::Uint16(0),
            "u32" => Self::Uint32(0),
            "u64" => Self::Uint64(0),
            "u128" => Self::Uint128(0),
            "f32" => Self::Float32(0.0),
            "f64" => Self::Float64(0.0),
            v => Self::Other(v.to_string()),
        }
    }
}

impl From<()> for RustTypes {
    fn from(_: ()) -> Self {
        Self::Void
    }
}

impl Into<()> for RustTypes {
    fn into(self) -> () {
        if let Self::Void = self {
            ()
        } else {
            panic!("Expected RustTypes::Void, got {:?}", self)
        }
    }
}

impl IntoJavaType for RustTypes {
    fn into_java_type(&self) -> String {
        match self.clone() {
            Self::String(_) => "String".to_string(),
            Self::Bool(_) => "Boolean".to_string(),
            Self::Uint8(_) | Self::Int8(_) => "Byte".to_string(),
            Self::Uint16(_) | Self::Int16(_) => "Short".to_string(),
            Self::Uint32(_) | Self::Int32(_) => "Integer".to_string(),
            Self::Uint64(_) | Self::Int64(_) => "Long".to_string(),
            Self::Uint128(_) | Self::Int128(_) => "java.math.BigInteger".to_string(),
            Self::Float32(_) => "Float".to_string(),
            Self::Float64(_) => "Double".to_string(),
            Self::Other(val) => val,
            Self::Void => "Void".to_string(),
        }
    }
}





#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_new_unchecked<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    value: T
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    object_to_jobject(env, Unit::new_unchecked(value), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_from_ref_unchecked<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    value: &T
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    object_to_jobject(env, Unit::from_ref_unchecked(value), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_new_normalize<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    value: T
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    object_to_jobject(env, Unit::new_normalize(value), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_new_and_get<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    value: T
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    object_to_jobject(env, UnitHelper::new_and_get(value), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_try_new<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    value: T,
    min_norm: N
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    object_to_jobject(env, Unit::try_new(value, min_norm), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_try_new_and_get<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    value: T,
    min_norm: N
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    object_to_jobject(env, Unit::try_new_and_get(value, min_norm), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_into_inner<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    let this: &Unit<T> = jlong_to_pointer::<Unit<T>>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, Unit::into_inner(this), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_unwrap<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    let this: &Unit<T> = jlong_to_pointer::<Unit<T>>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, Unit::unwrap(this), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_renormalize<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    let this: &mut Unit<T> = jlong_to_pointer::<Unit<T>>(this).as_mut().unwrap();
    object_to_jobject(env, Unit::renormalize(this), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_renormalize_fast<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    let this: &mut Unit<T> = jlong_to_pointer::<Unit<T>>(this).as_mut().unwrap();
    object_to_jobject(env, Unit::renormalize_fast(this), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Unit_jni_as_mut_unchecked<'local, T, N>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    T: Normed<Norm = N> + Clone,
    N: SimdRealField + RealField + Clone,
 {
    let this: &mut Unit<T> = jlong_to_pointer::<Unit<T>>(this).as_mut().unwrap();
    object_to_jobject(env, Unit::as_mut_unchecked(this), "com/dimforge/rapier3d/Unit".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_of<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    x: f32,
    y: f32,
    z: f32
) -> jobject {
    object_to_jobject(env, Vector3::new(x, y, z), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_create<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, Vector3::default(), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_x<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Vector3 = jlong_to_pointer::<Vector3>(this).as_mut().unwrap();
    object_to_jobject(env, Vector3::x(this), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_y<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Vector3 = jlong_to_pointer::<Vector3>(this).as_mut().unwrap();
    object_to_jobject(env, Vector3::y(this), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_z<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Vector3 = jlong_to_pointer::<Vector3>(this).as_mut().unwrap();
    object_to_jobject(env, Vector3::z(this), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_add<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    other: Vector3
) -> jobject {
    let this: &mut Vector3 = jlong_to_pointer::<Vector3>(this).as_mut().unwrap();
    object_to_jobject(env, Vector3::add(this, other), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_sub<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    other: Vector3
) -> jobject {
    let this: &mut Vector3 = jlong_to_pointer::<Vector3>(this).as_mut().unwrap();
    object_to_jobject(env, Vector3::sub(this, other), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_clone<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Vector3 = jlong_to_pointer::<Vector3>(this).as_mut().unwrap();
    object_to_jobject(env, Vector3::clone(this), "com/dimforge/rapier3d/Vector3".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_empty<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveCollisionTypes::empty(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_all<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveCollisionTypes::all(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_create<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveCollisionTypes::default(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_dynamic_dynamic<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_collision_types_dynamic_dynamic(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_dynamic_kinematic<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_collision_types_dynamic_kinematic(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_dynamic_fixed<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_collision_types_dynamic_fixed(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_kinematic_kinematic<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_collision_types_kinematic_kinematic(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_kinematic_fixed<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_collision_types_kinematic_fixed(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_fixed_fixed<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_collision_types_fixed_fixed(), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_of<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    bits: u16
) -> jobject {
    object_to_jobject(env, ActiveCollisionTypes::from_bits(bits), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveCollisionTypes_jni_valueOf<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &ActiveCollisionTypes = jlong_to_pointer::<ActiveCollisionTypes>(this).as_mut().unwrap();
    object_to_jobject(env, ActiveCollisionTypes::bits(this), "com/dimforge/rapier3d/ActiveCollisionTypes".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_empty<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveEvents::empty(), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_all<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveEvents::all(), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_create<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveEvents::default(), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_collision_events<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_events_collison_events(), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_contact_force_events<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_events_contact_force_events(), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_of<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    bits: u32
) -> jobject {
    object_to_jobject(env, ActiveEvents::from_bits(bits), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_valueOf<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &ActiveEvents = jlong_to_pointer::<ActiveEvents>(this).as_mut().unwrap();
    object_to_jobject(env, ActiveEvents::bits(this), "com/dimforge/rapier3d/ActiveEvents".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_empty<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveHooks::empty(), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_all<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveHooks::all(), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_create<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ActiveHooks::default(), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_filter_contact_pairs<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_hooks_filter_contact_pairs(), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_filter_intersection_pair<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_hooks_filter_intersection_pair(), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_modify_solver_contacts<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, BitflagsHelper::active_hooks_modify_solver_contacts(), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_of<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    bits: u32
) -> jobject {
    object_to_jobject(env, ActiveHooks::from_bits(bits), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_valueOf<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &ActiveHooks = jlong_to_pointer::<ActiveHooks>(this).as_mut().unwrap();
    object_to_jobject(env, ActiveHooks::bits(this), "com/dimforge/rapier3d/ActiveHooks".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_is_enabled<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::is_enabled(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_is_sensor<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::is_sensor(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_active_hooks<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::active_hooks(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_active_events<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::active_events(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_active_collision_types<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::active_collision_types(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_friction<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::friction(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_friction_combine_rule<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::friction_combine_rule(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_restitution<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::restitution(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_restitution_combine_rule<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::restitution_combine_rule(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_position<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::position(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_translation<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::translation(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_rotation<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::rotation(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_solver_groups<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::solver_groups(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_collision_groups<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::collision_groups(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_material<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::material(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_volume<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::volume(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_density<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::density(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_mass<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::mass(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_shape<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::shape(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_shared_shape<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::shared_shape(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_compute_aabb<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::compute_aabb(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_compute_swept_aabb<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    next_pos: &Isometry<f32>
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::compute_swept_aabb(this, next_pos), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_mass_properties<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::mass_properties(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_contact_force_event_threshold<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::contact_force_event_threshold(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_active_hooks<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    active_hooks: ActiveHooks
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_active_hooks(this, active_hooks), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_active_events<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    active_events: ActiveEvents
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_active_events(this, active_events), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_active_collision_types<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    active_collision_types: ActiveCollisionTypes
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_active_collision_types(this, active_collision_types), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_friction<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    coef: f32
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_friction(this, coef), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_friction_combine_rule<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rule: CoefficientCombineRule
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_friction_combine_rule(this, rule), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_restitution<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    coef: f32
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_restitution(this, coef), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_restitution_combine_rule<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rule: CoefficientCombineRule
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_restitution_combine_rule(this, rule), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_contact_force_event_threshold<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    threshold: f32
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_contact_force_event_threshold(this, threshold), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_sensor<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    is_sensor: bool
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_sensor(this, is_sensor), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_enabled<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    enabled: bool
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_enabled(this, enabled), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_translation<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    translation: Vector<f32>
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_translation(this, translation), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_rotation<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rotation: Rotation<f32>
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_rotation(this, rotation), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_position<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    position: Isometry<f32>
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_position(this, position), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_translation_wrt_parent<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    translation: Vector<f32>
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_translation_wrt_parent(this, translation), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_rotation_wrt_parent<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rotation: AngVector<f32>
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_rotation_wrt_parent(this, rotation), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_position_wrt_parent<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    pos_wrt_parent: Isometry<f32>
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_position_wrt_parent(this, pos_wrt_parent), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_collision_groups<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    groups: InteractionGroups
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_collision_groups(this, groups), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_solver_groups<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    groups: InteractionGroups
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_solver_groups(this, groups), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_density<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    density: f32
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_density(this, density), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_mass<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    mass: f32
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_mass(this, mass), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_mass_properties<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    mass_properties: MassProperties
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_mass_properties(this, mass_properties), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_shape_mut<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::shape_mut(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_set_shape<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    shape: SharedShape
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::set_shape(this, shape), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_parent<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::parent(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_position_wrt_parent<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::position_wrt_parent(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_clone<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::clone(this), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_clone_from<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    other: &Collider
) -> jobject {
    let this: &mut Collider = jlong_to_pointer::<Collider>(this).as_mut().unwrap();
    object_to_jobject(env, Collider::clone_from(this, other), "com/dimforge/rapier3d/Collider".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_create<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    shape: SharedShape
) -> jobject {
    object_to_jobject(env, ColliderBuilder::new(shape), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_compound<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    shapes: List<Tuple<Vector3, SharedShape>>
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::compound(shapes), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_ball<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::ball(radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_halfspace<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    outward_normal: Vector3
) -> jobject {
    object_to_jobject(env, ColliderBuilder::halfspace(outward_normal.into()), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_cylinder<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::cylinder(half_height, radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_cylinder<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::round_cylinder(half_height, radius, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_cone<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::cone(half_height, radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_cone<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::round_cone(half_height, radius, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_capsule_x<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::capsule_x(half_height, radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_capsule_y<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::capsule_y(half_height, radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_capsule_z<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    half_height: f32,
    radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::capsule_z(half_height, radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_cuboid<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    hx: f32,
    hy: f32,
    hz: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::cuboid(hx, hy, hz), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_cuboid<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    hx: f32,
    hy: f32,
    hz: f32,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::round_cuboid(hx, hy, hz, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_segment<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    a: Vector3,
    b: Vector3
) -> jobject {
    object_to_jobject(env, ColliderBuilder::segment(a.into(), b), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_triangle<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    a: Vector3,
    b: Vector3,
    c: Vector3
) -> jobject {
    object_to_jobject(env, ColliderBuilder::triangle(a.into(), b, c), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_triangle<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    a: Vector3,
    b: Vector3,
    c: Vector3,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilder::round_triangle(a.into(), b, c, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_heightfield<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    heights: DMatrix<f32>,
    scale: Vector<f32>
) -> jobject {
    object_to_jobject(env, ColliderBuilder::heightfield(heights, scale), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_polyline<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: Optional<List<Tuple<u32, u32>>>
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::polyline(vertices, indices), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_trimesh<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::trimesh(vertices, indices), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_trimesh_with_flags<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>,
    flags: TriMeshFlags
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::trimesh_with_flags(vertices, indices, flags), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_convex_decomposition<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::convex_decomposition(vertices, indices), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_convex_decomposition<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::round_convex_decomposition(vertices, indices, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_convex_decomposition_with_params<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>,
    params: &VHACDParameters
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::convex_decomposition_with_params(vertices, indices, params), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_convex_decomposition_with_params<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    vertices: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>,
    params: &VHACDParameters,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::round_convex_decomposition_with_params(vertices, indices, params, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_convex_hull<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    points: List<Vector3>
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::convex_hull(points), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_convex_hull<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    points: List<Vector3>,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::round_convex_hull(points, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_convex_mesh<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    points: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::convex_mesh(points, indices), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_round_convex_mesh<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    points: List<Vector3>,
    indices: List<Triple<u32, u32, u32>>,
    border_radius: f32
) -> jobject {
    object_to_jobject(env, ColliderBuilderCustom::round_convex_mesh(points, indices, border_radius), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_build<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    object_to_jobject(env, ColliderBuilder::build(this), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_default_friction<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ColliderBuilder::default_friction(), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_default_density<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>
) -> jobject {
    object_to_jobject(env, ColliderBuilder::default_density(), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_user_data<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    data: u128
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::user_data(this, data), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_collision_groups<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    groups: InteractionGroups
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::collision_groups(this, groups), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_solver_groups<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    groups: InteractionGroups
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::solver_groups(this, groups), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_sensor<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    is_sensor: bool
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::sensor(this, is_sensor), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_active_hooks<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    active_hooks: ActiveHooks
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::active_hooks(this, active_hooks), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_active_events<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    active_events: ActiveEvents
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::active_events(this, active_events), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_active_collision_types<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    active_collision_types: ActiveCollisionTypes
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::active_collision_types(this, active_collision_types), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_friction<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    friction: f32
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::friction(this, friction), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_friction_combine_rule<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rule: CoefficientCombineRule
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::friction_combine_rule(this, rule), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_restitution<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    restitution: f32
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::restitution(this, restitution), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_restitution_combine_rule<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rule: CoefficientCombineRule
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::restitution_combine_rule(this, rule), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_density<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    density: f32
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::density(this, density), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_mass<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    mass: f32
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::mass(this, mass), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_mass_properties<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    mass_properties: MassProperties
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::mass_properties(this, mass_properties), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_contact_force_event_threshold<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    threshold: f32
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::contact_force_event_threshold(this, threshold), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_translation<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    translation: Vector3
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::translation(this, translation.into()), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_rotation<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    rotation: Vector3
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::rotation(this, rotation.into()), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_position<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    pos: Vector3
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::position(this, pos.into()), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_position_wrt_parent<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    pos: Vector3
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::position_wrt_parent(this, pos.into()), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_delta<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    delta: Vector3
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::delta(this, delta.into()), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_enabled<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    enabled: bool
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, ColliderBuilder::enabled(this, enabled), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_clone<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject {
    let this: &ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    object_to_jobject(env, ColliderBuilder::clone(this), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_clone_from<'local>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong,
    other: &ColliderBuilder
) -> jobject {
    let this: &mut ColliderBuilder = jlong_to_pointer::<ColliderBuilder>(this).as_mut().unwrap();
    object_to_jobject(env, ColliderBuilder::clone_from(this, other), "com/dimforge/rapier3d/ColliderBuilder".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_of<'local, A, B, C>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    first: A,
    second: B,
    third: C
) -> jobject where
    A: Clone,
    B: Clone,
    C: Clone,
 {
    object_to_jobject(env, Triple::of(first, second, third), "com/dimforge/rapier3d/Triple".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_first<'local, A, B, C>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    A: Clone,
    B: Clone,
    C: Clone,
 {
    let this: &Triple<A, B, C> = jlong_to_pointer::<Triple<A, B, C>>(this).as_mut().unwrap();
    object_to_jobject(env, Triple::first(this), "com/dimforge/rapier3d/Triple".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_second<'local, A, B, C>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    A: Clone,
    B: Clone,
    C: Clone,
 {
    let this: &Triple<A, B, C> = jlong_to_pointer::<Triple<A, B, C>>(this).as_mut().unwrap();
    object_to_jobject(env, Triple::second(this), "com/dimforge/rapier3d/Triple".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_third<'local, A, B, C>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    A: Clone,
    B: Clone,
    C: Clone,
 {
    let this: &Triple<A, B, C> = jlong_to_pointer::<Triple<A, B, C>>(this).as_mut().unwrap();
    object_to_jobject(env, Triple::third(this), "com/dimforge/rapier3d/Triple".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_of<'local, A, B>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    first: A,
    second: B
) -> jobject where
    A: Clone,
    B: Clone,
 {
    object_to_jobject(env, Tuple::of(first, second), "com/dimforge/rapier3d/Tuple".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_first<'local, A, B>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    A: Clone,
    B: Clone,
 {
    let this: &Tuple<A, B> = jlong_to_pointer::<Tuple<A, B>>(this).as_mut().unwrap();
    object_to_jobject(env, Tuple::first(this), "com/dimforge/rapier3d/Tuple".to_string())
}

#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_second<'local, A, B>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong
) -> jobject where
    A: Clone,
    B: Clone,
 {
    let this: &Tuple<A, B> = jlong_to_pointer::<Tuple<A, B>>(this).as_mut().unwrap();
    object_to_jobject(env, Tuple::second(this), "com/dimforge/rapier3d/Tuple".to_string())
}

