use rs4j::prelude::*;

#[allow(non_camel_case_types)]
pub struct __JNI_Vector3 {
    pub inner: *mut NVector3<f32>,
}

impl __JNI_Vector3 {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> Vector3 {
        Vector3 {
            inner: (&mut *self.inner).clone(),
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_new(x: f32, y: f32, z: f32) -> Self {
        let base = Vector3::new(x, y, z);

        Self {
            inner: Box::leak(Box::new(base.inner)) as *mut NVector3<f32>,
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_default() -> Self {
        let base = Vector3::default();

        Self {
            inner: Box::leak(Box::new(base.inner)) as *mut NVector3<f32>,
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_x(&self, ) -> f32 {
        Vector3::x(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_y(&self, ) -> f32 {
        Vector3::y(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_z(&self, ) -> f32 {
        Vector3::z(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_add(&mut self, other: Vector3) -> Vector3 {
        Vector3::add(&mut self.to_rust(), other).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_sub(&mut self, other: Vector3) -> Vector3 {
        Vector3::sub(&mut self.to_rust(), other).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_clone(&self, ) -> Vector3 {
        Vector3::clone(&self.to_rust()).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1init_1new<'local, >(mut env: JNIEnv<'local>, obj: JObject<'local>, x: jfloat, y: jfloat, z: jfloat) -> jlong {
    
    let it = __JNI_Vector3::__wrapped_new(x, y, z);
    (Box::leak(Box::new(it)) as *mut __JNI_Vector3) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1init_1default<'local, >(mut env: JNIEnv<'local>, obj: JObject<'local>, ) -> jlong {
    
    let it = __JNI_Vector3::__wrapped_default();
    (Box::leak(Box::new(it)) as *mut __JNI_Vector3) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1x<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Vector3);

    it.__wrapped_x()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1y<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Vector3);

    it.__wrapped_y()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1z<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Vector3);

    it.__wrapped_z()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1add<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, other: jlong) -> jlong {
    let it = &mut *(ptr as *mut __JNI_Vector3);
    let other = &*(other as *mut Vector3);

    let val = it.__wrapped_add(other.clone());
    (Box::leak(Box::new(val)) as *mut Vector3) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1sub<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, other: jlong) -> jlong {
    let it = &mut *(ptr as *mut __JNI_Vector3);
    let other = &*(other as *mut Vector3);

    let val = it.__wrapped_sub(other.clone());
    (Box::leak(Box::new(val)) as *mut Vector3) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1clone<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Vector3);

    let val = it.__wrapped_clone();
    (Box::leak(Box::new(val)) as *mut Vector3) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Vector3_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_Vector3);
    let _ = Box::from_raw(it.inner);
}

#[allow(non_camel_case_types)]
pub struct __JNI_ActiveEvents {
    pub __inner: ActiveEvents,
}

impl __JNI_ActiveEvents {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> ActiveEvents {
        self.__inner.clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_empty() -> ActiveEvents {
        ActiveEvents::empty()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_all() -> ActiveEvents {
        ActiveEvents::all()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_default() -> Self {
        let base = ActiveEvents::default();

        Self {
            __inner: base,
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_of(bits: u32) -> Option<ActiveEvents> {
        ActiveEvents::from_bits(bits)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_collision_events() -> u32 {
        BitflagsHelper::active_events_collison_events()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_contact_force_events() -> u32 {
        BitflagsHelper::active_events_contact_force_events()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_valueOf(&self, ) -> u32 {
        ActiveEvents::bits(&self.to_rust()).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1empty<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    

    let val = __JNI_ActiveEvents::__wrapped_empty();
    (Box::leak(Box::new(val)) as *mut ActiveEvents) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1all<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    

    let val = __JNI_ActiveEvents::__wrapped_all();
    (Box::leak(Box::new(val)) as *mut ActiveEvents) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1init_1default<'local, >(mut env: JNIEnv<'local>, obj: JObject<'local>, ) -> jlong {
    
    let it = __JNI_ActiveEvents::__wrapped_default();
    (Box::leak(Box::new(it)) as *mut __JNI_ActiveEvents) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1of<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, bits: jint) -> jlong {
    

    let val = __JNI_ActiveEvents::__wrapped_of(bits as u32).unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut ActiveEvents) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1collision_1events<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    

    __JNI_ActiveEvents::__wrapped_collision_events() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1contact_1force_1events<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    

    __JNI_ActiveEvents::__wrapped_contact_force_events() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1valueOf<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    let it = &*(ptr as *mut __JNI_ActiveEvents);

    it.__wrapped_valueOf() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveEvents_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_ActiveEvents);
    
}

#[allow(non_camel_case_types)]
pub struct __JNI_ActiveHooks {
    pub __inner: ActiveHooks,
}

impl __JNI_ActiveHooks {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> ActiveHooks {
        self.__inner.clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_empty() -> ActiveHooks {
        ActiveHooks::empty()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_all() -> ActiveHooks {
        ActiveHooks::all()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_create() -> ActiveHooks {
        ActiveHooks::default()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_filter_contact_pairs() -> u32 {
        BitflagsHelper::active_hooks_filter_contact_pairs()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_filter_intersection_pair() -> u32 {
        BitflagsHelper::active_hooks_filter_intersection_pair()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_modify_solver_contacts() -> u32 {
        BitflagsHelper::active_hooks_modify_solver_contacts()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_of(bits: u32) -> Option<ActiveHooks> {
        ActiveHooks::from_bits(bits)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_valueOf(&self, ) -> u32 {
        ActiveHooks::bits(&self.to_rust()).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1empty<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    

    let val = __JNI_ActiveHooks::__wrapped_empty();
    (Box::leak(Box::new(val)) as *mut ActiveHooks) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1all<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    

    let val = __JNI_ActiveHooks::__wrapped_all();
    (Box::leak(Box::new(val)) as *mut ActiveHooks) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1create<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    

    let val = __JNI_ActiveHooks::__wrapped_create();
    (Box::leak(Box::new(val)) as *mut ActiveHooks) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1filter_1contact_1pairs<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    

    __JNI_ActiveHooks::__wrapped_filter_contact_pairs() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1filter_1intersection_1pair<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    

    __JNI_ActiveHooks::__wrapped_filter_intersection_pair() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1modify_1solver_1contacts<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    

    __JNI_ActiveHooks::__wrapped_modify_solver_contacts() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1of<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, bits: jint) -> jlong {
    

    let val = __JNI_ActiveHooks::__wrapped_of(bits as u32).unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut ActiveHooks) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1valueOf<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jint {
    let it = &*(ptr as *mut __JNI_ActiveHooks);

    it.__wrapped_valueOf() as i32
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ActiveHooks_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_ActiveHooks);
    
}

#[allow(non_camel_case_types)]
pub struct __JNI_Collider {
    pub __inner: Collider,
}

impl __JNI_Collider {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> Collider {
        self.__inner.clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_is_enabled(&self, ) -> bool {
        Collider::is_enabled(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_is_sensor(&self, ) -> bool {
        Collider::is_sensor(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_active_hooks(&self, ) -> ActiveHooks {
        Collider::active_hooks(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_active_events(&self, ) -> ActiveEvents {
        Collider::active_events(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_active_collision_types(&self, ) -> ActiveCollisionTypes {
        Collider::active_collision_types(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_friction(&self, ) -> f32 {
        Collider::friction(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_friction_combine_rule(&self, ) -> CoefficientCombineRule {
        Collider::friction_combine_rule(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_restitution(&self, ) -> f32 {
        Collider::restitution(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_restitution_combine_rule(&self, ) -> CoefficientCombineRule {
        Collider::restitution_combine_rule(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_position(&self, ) -> Isometry<f32> {
        Collider::position(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_translation(&self, ) -> Vector<f32> {
        Collider::translation(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_rotation(&self, ) -> Rotation<f32> {
        Collider::rotation(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_solver_groups(&self, ) -> InteractionGroups {
        Collider::solver_groups(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_collision_groups(&self, ) -> InteractionGroups {
        Collider::collision_groups(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_material(&self, ) -> ColliderMaterial {
        Collider::material(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_volume(&self, ) -> f32 {
        Collider::volume(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_density(&self, ) -> f32 {
        Collider::density(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_mass(&self, ) -> f32 {
        Collider::mass(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_shared_shape(&self, ) -> SharedShape {
        Collider::shared_shape(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_compute_aabb(&self, ) -> Aabb {
        Collider::compute_aabb(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_compute_swept_aabb(&self, next_pos: &Isometry<f32>) -> Aabb {
        Collider::compute_swept_aabb(&self.to_rust(), next_pos).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_mass_properties(&self, ) -> MassProperties {
        Collider::mass_properties(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_contact_force_event_threshold(&self, ) -> f32 {
        Collider::contact_force_event_threshold(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_active_hooks(&mut self, active_hooks: ActiveHooks) -> () {
        Collider::set_active_hooks(&mut self.to_rust(), active_hooks).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_active_events(&mut self, active_events: ActiveEvents) -> () {
        Collider::set_active_events(&mut self.to_rust(), active_events).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_active_collision_types(&mut self, active_collision_types: ActiveCollisionTypes) -> () {
        Collider::set_active_collision_types(&mut self.to_rust(), active_collision_types).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_friction(&mut self, coef: f32) -> () {
        Collider::set_friction(&mut self.to_rust(), coef).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_friction_combine_rule(&mut self, rule: CoefficientCombineRule) -> () {
        Collider::set_friction_combine_rule(&mut self.to_rust(), rule).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_restitution(&mut self, coef: f32) -> () {
        Collider::set_restitution(&mut self.to_rust(), coef).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_restitution_combine_rule(&mut self, rule: CoefficientCombineRule) -> () {
        Collider::set_restitution_combine_rule(&mut self.to_rust(), rule).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_contact_force_event_threshold(&mut self, threshold: f32) -> () {
        Collider::set_contact_force_event_threshold(&mut self.to_rust(), threshold).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_sensor(&mut self, is_sensor: bool) -> () {
        Collider::set_sensor(&mut self.to_rust(), is_sensor).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_enabled(&mut self, enabled: bool) -> () {
        Collider::set_enabled(&mut self.to_rust(), enabled).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_translation(&mut self, translation: Vector<f32>) -> () {
        Collider::set_translation(&mut self.to_rust(), translation).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_rotation(&mut self, rotation: Rotation<f32>) -> () {
        Collider::set_rotation(&mut self.to_rust(), rotation).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_position(&mut self, position: Isometry<f32>) -> () {
        Collider::set_position(&mut self.to_rust(), position).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_translation_wrt_parent(&mut self, translation: Vector<f32>) -> () {
        Collider::set_translation_wrt_parent(&mut self.to_rust(), translation).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_rotation_wrt_parent(&mut self, rotation: AngVector<f32>) -> () {
        Collider::set_rotation_wrt_parent(&mut self.to_rust(), rotation).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_position_wrt_parent(&mut self, pos_wrt_parent: Isometry<f32>) -> () {
        Collider::set_position_wrt_parent(&mut self.to_rust(), pos_wrt_parent).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_collision_groups(&mut self, groups: InteractionGroups) -> () {
        Collider::set_collision_groups(&mut self.to_rust(), groups).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_solver_groups(&mut self, groups: InteractionGroups) -> () {
        Collider::set_solver_groups(&mut self.to_rust(), groups).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_density(&mut self, density: f32) -> () {
        Collider::set_density(&mut self.to_rust(), density).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_mass(&mut self, mass: f32) -> () {
        Collider::set_mass(&mut self.to_rust(), mass).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_mass_properties(&mut self, mass_properties: MassProperties) -> () {
        Collider::set_mass_properties(&mut self.to_rust(), mass_properties).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_set_shape(&mut self, shape: SharedShape) -> () {
        Collider::set_shape(&mut self.to_rust(), shape).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_parent(&self, ) -> Option<RigidBodyHandle> {
        Collider::parent(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_clone(&self, ) -> Collider {
        Collider::clone(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_clone_from(&mut self, other: &Collider) -> () {
        Collider::clone_from(&mut self.to_rust(), other).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1is_1enabled<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jboolean {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_is_enabled() as u8
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1is_1sensor<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jboolean {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_is_sensor() as u8
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1active_1hooks<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_active_hooks();
    (Box::leak(Box::new(val)) as *mut ActiveHooks) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1active_1events<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_active_events();
    (Box::leak(Box::new(val)) as *mut ActiveEvents) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1active_1collision_1types<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_active_collision_types();
    (Box::leak(Box::new(val)) as *mut ActiveCollisionTypes) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1friction<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_friction()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1friction_1combine_1rule<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_friction_combine_rule();
    (Box::leak(Box::new(val)) as *mut CoefficientCombineRule) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1restitution<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_restitution()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1restitution_1combine_1rule<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_restitution_combine_rule();
    (Box::leak(Box::new(val)) as *mut CoefficientCombineRule) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1position<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_position();
    (Box::leak(Box::new(val)) as *mut Isometry<f32>) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1translation<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_translation();
    (Box::leak(Box::new(val)) as *mut Vector<f32>) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1rotation<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_rotation();
    (Box::leak(Box::new(val)) as *mut Rotation<f32>) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1solver_1groups<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_solver_groups();
    (Box::leak(Box::new(val)) as *mut InteractionGroups) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1collision_1groups<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_collision_groups();
    (Box::leak(Box::new(val)) as *mut InteractionGroups) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1material<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_material();
    (Box::leak(Box::new(val)) as *mut ColliderMaterial) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1volume<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_volume()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1density<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_density()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1mass<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_mass()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1shared_1shape<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_shared_shape();
    (Box::leak(Box::new(val)) as *mut SharedShape) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1compute_1aabb<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_compute_aabb();
    (Box::leak(Box::new(val)) as *mut Aabb) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1compute_1swept_1aabb<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, next_pos: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);
    let next_pos = &*(next_pos as *mut Isometry<f32>);

    let val = it.__wrapped_compute_swept_aabb(&next_pos);
    (Box::leak(Box::new(val)) as *mut Aabb) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1mass_1properties<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_mass_properties();
    (Box::leak(Box::new(val)) as *mut MassProperties) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1contact_1force_1event_1threshold<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    let it = &*(ptr as *mut __JNI_Collider);

    it.__wrapped_contact_force_event_threshold()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1active_1hooks<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, active_hooks: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let active_hooks = &*(active_hooks as *mut ActiveHooks);

    it.__wrapped_set_active_hooks(active_hooks.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1active_1events<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, active_events: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let active_events = &*(active_events as *mut ActiveEvents);

    it.__wrapped_set_active_events(active_events.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1active_1collision_1types<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, active_collision_types: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let active_collision_types = &*(active_collision_types as *mut ActiveCollisionTypes);

    it.__wrapped_set_active_collision_types(active_collision_types.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1friction<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, coef: jfloat) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_friction(coef)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1friction_1combine_1rule<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, rule: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let rule = &*(rule as *mut CoefficientCombineRule);

    it.__wrapped_set_friction_combine_rule(rule.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1restitution<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, coef: jfloat) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_restitution(coef)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1restitution_1combine_1rule<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, rule: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let rule = &*(rule as *mut CoefficientCombineRule);

    it.__wrapped_set_restitution_combine_rule(rule.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1contact_1force_1event_1threshold<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, threshold: jfloat) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_contact_force_event_threshold(threshold)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1sensor<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, is_sensor: jboolean) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_sensor(is_sensor == 1)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1enabled<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, enabled: jboolean) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_enabled(enabled == 1)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1translation<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, translation: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let translation = &*(translation as *mut Vector<f32>);

    it.__wrapped_set_translation(translation.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1rotation<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, rotation: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let rotation = &*(rotation as *mut Rotation<f32>);

    it.__wrapped_set_rotation(rotation.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1position<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, position: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let position = &*(position as *mut Isometry<f32>);

    it.__wrapped_set_position(position.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1translation_1wrt_1parent<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, translation: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let translation = &*(translation as *mut Vector<f32>);

    it.__wrapped_set_translation_wrt_parent(translation.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1rotation_1wrt_1parent<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, rotation: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let rotation = &*(rotation as *mut AngVector<f32>);

    it.__wrapped_set_rotation_wrt_parent(rotation.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1position_1wrt_1parent<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, pos_wrt_parent: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let pos_wrt_parent = &*(pos_wrt_parent as *mut Isometry<f32>);

    it.__wrapped_set_position_wrt_parent(pos_wrt_parent.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1collision_1groups<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, groups: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let groups = &*(groups as *mut InteractionGroups);

    it.__wrapped_set_collision_groups(groups.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1solver_1groups<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, groups: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let groups = &*(groups as *mut InteractionGroups);

    it.__wrapped_set_solver_groups(groups.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1density<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, density: jfloat) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_density(density)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1mass<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, mass: jfloat) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);

    it.__wrapped_set_mass(mass)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1mass_1properties<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, mass_properties: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let mass_properties = &*(mass_properties as *mut MassProperties);

    it.__wrapped_set_mass_properties(mass_properties.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1set_1shape<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, shape: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let shape = &*(shape as *mut SharedShape);

    it.__wrapped_set_shape(shape.clone())
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1parent<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_parent().unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut RigidBodyHandle) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1clone<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Collider);

    let val = it.__wrapped_clone();
    (Box::leak(Box::new(val)) as *mut Collider) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1clone_1from<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, other: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_Collider);
    let other = &*(other as *mut Collider);

    it.__wrapped_clone_from(&other)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Collider_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_Collider);
    
}

#[allow(non_camel_case_types)]
pub struct __JNI_ColliderBuilder {
    pub __inner: ColliderBuilder,
}

impl __JNI_ColliderBuilder {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> ColliderBuilder {
        self.__inner.clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_create(shape: SharedShape) -> ColliderBuilder {
        ColliderBuilder::new(shape)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_compound(shapes: List<Tuple<Vector3, SharedShape>>) -> ColliderBuilder {
        ColliderBuilderCustom::compound(shapes)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_ball(radius: f32) -> ColliderBuilder {
        ColliderBuilder::ball(radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_cylinder(half_height: f32, radius: f32) -> ColliderBuilder {
        ColliderBuilder::cylinder(half_height, radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_cylinder(half_height: f32, radius: f32, border_radius: f32) -> ColliderBuilder {
        ColliderBuilder::round_cylinder(half_height, radius, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_cone(half_height: f32, radius: f32) -> ColliderBuilder {
        ColliderBuilder::cone(half_height, radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_cone(half_height: f32, radius: f32, border_radius: f32) -> ColliderBuilder {
        ColliderBuilder::round_cone(half_height, radius, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_capsule_x(half_height: f32, radius: f32) -> ColliderBuilder {
        ColliderBuilder::capsule_x(half_height, radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_capsule_y(half_height: f32, radius: f32) -> ColliderBuilder {
        ColliderBuilder::capsule_y(half_height, radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_capsule_z(half_height: f32, radius: f32) -> ColliderBuilder {
        ColliderBuilder::capsule_z(half_height, radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_cuboid(hx: f32, hy: f32, hz: f32) -> ColliderBuilder {
        ColliderBuilder::cuboid(hx, hy, hz)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_cuboid(hx: f32, hy: f32, hz: f32, border_radius: f32) -> ColliderBuilder {
        ColliderBuilder::round_cuboid(hx, hy, hz, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_heightfield(heights: DMatrix<f32>, scale: Vector<f32>) -> ColliderBuilder {
        ColliderBuilder::heightfield(heights, scale)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_polyline(vertices: List<Vector3>, indices: Optional<List<Tuple<u32, u32>>>) -> ColliderBuilder {
        ColliderBuilderCustom::polyline(vertices, indices)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_trimesh(vertices: List<Vector3>, indices: List<Triple<u32, u32, u32>>) -> ColliderBuilder {
        ColliderBuilderCustom::trimesh(vertices, indices)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_trimesh_with_flags(vertices: List<Vector3>, indices: List<Triple<u32, u32, u32>>, flags: TriMeshFlags) -> ColliderBuilder {
        ColliderBuilderCustom::trimesh_with_flags(vertices, indices, flags)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_convex_decomposition(vertices: List<Vector3>, indices: List<Triple<u32, u32, u32>>) -> ColliderBuilder {
        ColliderBuilderCustom::convex_decomposition(vertices, indices)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_convex_decomposition(vertices: List<Vector3>, indices: List<Triple<u32, u32, u32>>, border_radius: f32) -> ColliderBuilder {
        ColliderBuilderCustom::round_convex_decomposition(vertices, indices, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_convex_decomposition_with_params(vertices: List<Vector3>, indices: List<Triple<u32, u32, u32>>, params: &VHACDParameters) -> ColliderBuilder {
        ColliderBuilderCustom::convex_decomposition_with_params(vertices, indices, params)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_convex_decomposition_with_params(vertices: List<Vector3>, indices: List<Triple<u32, u32, u32>>, params: &VHACDParameters, border_radius: f32) -> ColliderBuilder {
        ColliderBuilderCustom::round_convex_decomposition_with_params(vertices, indices, params, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_convex_hull(points: List<Vector3>) -> Option<ColliderBuilder> {
        ColliderBuilderCustom::convex_hull(points)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_convex_hull(points: List<Vector3>, border_radius: f32) -> Option<ColliderBuilder> {
        ColliderBuilderCustom::round_convex_hull(points, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_convex_mesh(points: List<Vector3>, indices: List<Triple<u32, u32, u32>>) -> Option<ColliderBuilder> {
        ColliderBuilderCustom::convex_mesh(points, indices)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_round_convex_mesh(points: List<Vector3>, indices: List<Triple<u32, u32, u32>>, border_radius: f32) -> Option<ColliderBuilder> {
        ColliderBuilderCustom::round_convex_mesh(points, indices, border_radius)
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_build(&self, ) -> Collider {
        ColliderBuilder::build(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_default_friction() -> f32 {
        ColliderBuilder::default_friction()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_default_density() -> f32 {
        ColliderBuilder::default_density()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_collision_groups(&self, groups: InteractionGroups) -> ColliderBuilder {
        ColliderBuilder::collision_groups(self.to_rust(), groups).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_solver_groups(&self, groups: InteractionGroups) -> ColliderBuilder {
        ColliderBuilder::solver_groups(self.to_rust(), groups).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_sensor(&self, is_sensor: bool) -> ColliderBuilder {
        ColliderBuilder::sensor(self.to_rust(), is_sensor).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_active_hooks(&self, active_hooks: ActiveHooks) -> ColliderBuilder {
        ColliderBuilder::active_hooks(self.to_rust(), active_hooks).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_active_events(&self, active_events: ActiveEvents) -> ColliderBuilder {
        ColliderBuilder::active_events(self.to_rust(), active_events).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_active_collision_types(&self, active_collision_types: ActiveCollisionTypes) -> ColliderBuilder {
        ColliderBuilder::active_collision_types(self.to_rust(), active_collision_types).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_friction(&self, friction: f32) -> ColliderBuilder {
        ColliderBuilder::friction(self.to_rust(), friction).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_friction_combine_rule(&self, rule: CoefficientCombineRule) -> ColliderBuilder {
        ColliderBuilder::friction_combine_rule(self.to_rust(), rule).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_restitution(&self, restitution: f32) -> ColliderBuilder {
        ColliderBuilder::restitution(self.to_rust(), restitution).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_restitution_combine_rule(&self, rule: CoefficientCombineRule) -> ColliderBuilder {
        ColliderBuilder::restitution_combine_rule(self.to_rust(), rule).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_density(&self, density: f32) -> ColliderBuilder {
        ColliderBuilder::density(self.to_rust(), density).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_mass(&self, mass: f32) -> ColliderBuilder {
        ColliderBuilder::mass(self.to_rust(), mass).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_mass_properties(&self, mass_properties: MassProperties) -> ColliderBuilder {
        ColliderBuilder::mass_properties(self.to_rust(), mass_properties).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_contact_force_event_threshold(&self, threshold: f32) -> ColliderBuilder {
        ColliderBuilder::contact_force_event_threshold(self.to_rust(), threshold).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_enabled(&self, enabled: bool) -> ColliderBuilder {
        ColliderBuilder::enabled(self.to_rust(), enabled).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_clone(&self, ) -> ColliderBuilder {
        ColliderBuilder::clone(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_clone_from(&mut self, other: &ColliderBuilder) -> () {
        ColliderBuilder::clone_from(&mut self.to_rust(), other).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1create<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, shape: jlong) -> jlong {
        let shape = &*(shape as *mut SharedShape);

    let val = __JNI_ColliderBuilder::__wrapped_create(shape.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1compound<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, shapes: jlong) -> jlong {
        let shapes = &*(shapes as *mut List<Tuple<Vector3, SharedShape>>);

    let val = __JNI_ColliderBuilder::__wrapped_compound(shapes.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1ball<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_ball(radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1cylinder<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_cylinder(half_height, radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1cylinder<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat, border_radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_round_cylinder(half_height, radius, border_radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1cone<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_cone(half_height, radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1cone<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat, border_radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_round_cone(half_height, radius, border_radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1capsule_1x<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_capsule_x(half_height, radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1capsule_1y<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_capsule_y(half_height, radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1capsule_1z<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, half_height: jfloat, radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_capsule_z(half_height, radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1cuboid<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, hx: jfloat, hy: jfloat, hz: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_cuboid(hx, hy, hz);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1cuboid<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, hx: jfloat, hy: jfloat, hz: jfloat, border_radius: jfloat) -> jlong {
    

    let val = __JNI_ColliderBuilder::__wrapped_round_cuboid(hx, hy, hz, border_radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1heightfield<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, heights: jlong, scale: jlong) -> jlong {
        let heights = &*(heights as *mut DMatrix<f32>);
    let scale = &*(scale as *mut Vector<f32>);

    let val = __JNI_ColliderBuilder::__wrapped_heightfield(heights.clone(), scale.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1polyline<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut Optional<List<Tuple<u32, u32>>>);

    let val = __JNI_ColliderBuilder::__wrapped_polyline(vertices.clone(), indices.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1trimesh<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);

    let val = __JNI_ColliderBuilder::__wrapped_trimesh(vertices.clone(), indices.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1trimesh_1with_1flags<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong, flags: jlong) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);
    let flags = &*(flags as *mut TriMeshFlags);

    let val = __JNI_ColliderBuilder::__wrapped_trimesh_with_flags(vertices.clone(), indices.clone(), flags.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1convex_1decomposition<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);

    let val = __JNI_ColliderBuilder::__wrapped_convex_decomposition(vertices.clone(), indices.clone());
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1convex_1decomposition<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong, border_radius: jfloat) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);

    let val = __JNI_ColliderBuilder::__wrapped_round_convex_decomposition(vertices.clone(), indices.clone(), border_radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1convex_1decomposition_1with_1params<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong, params: jlong) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);
    let params = &*(params as *mut VHACDParameters);

    let val = __JNI_ColliderBuilder::__wrapped_convex_decomposition_with_params(vertices.clone(), indices.clone(), &params);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1convex_1decomposition_1with_1params<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, vertices: jlong, indices: jlong, params: jlong, border_radius: jfloat) -> jlong {
        let vertices = &*(vertices as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);
    let params = &*(params as *mut VHACDParameters);

    let val = __JNI_ColliderBuilder::__wrapped_round_convex_decomposition_with_params(vertices.clone(), indices.clone(), &params, border_radius);
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1convex_1hull<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, points: jlong) -> jlong {
        let points = &*(points as *mut List<Vector3>);

    let val = __JNI_ColliderBuilder::__wrapped_convex_hull(points.clone()).unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1convex_1hull<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, points: jlong, border_radius: jfloat) -> jlong {
        let points = &*(points as *mut List<Vector3>);

    let val = __JNI_ColliderBuilder::__wrapped_round_convex_hull(points.clone(), border_radius).unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1convex_1mesh<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, points: jlong, indices: jlong) -> jlong {
        let points = &*(points as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);

    let val = __JNI_ColliderBuilder::__wrapped_convex_mesh(points.clone(), indices.clone()).unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1round_1convex_1mesh<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, points: jlong, indices: jlong, border_radius: jfloat) -> jlong {
        let points = &*(points as *mut List<Vector3>);
    let indices = &*(indices as *mut List<Triple<u32, u32, u32>>);

    let val = __JNI_ColliderBuilder::__wrapped_round_convex_mesh(points.clone(), indices.clone(), border_radius).unwrap_or_default();
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1build<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_build();
    (Box::leak(Box::new(val)) as *mut Collider) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1default_1friction<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    

    __JNI_ColliderBuilder::__wrapped_default_friction()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1default_1density<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jfloat {
    

    __JNI_ColliderBuilder::__wrapped_default_density()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1collision_1groups<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, groups: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let groups = &*(groups as *mut InteractionGroups);

    let val = it.__wrapped_collision_groups(groups.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1solver_1groups<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, groups: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let groups = &*(groups as *mut InteractionGroups);

    let val = it.__wrapped_solver_groups(groups.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1sensor<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, is_sensor: jboolean) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_sensor(is_sensor == 1);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1active_1hooks<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, active_hooks: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let active_hooks = &*(active_hooks as *mut ActiveHooks);

    let val = it.__wrapped_active_hooks(active_hooks.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1active_1events<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, active_events: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let active_events = &*(active_events as *mut ActiveEvents);

    let val = it.__wrapped_active_events(active_events.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1active_1collision_1types<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, active_collision_types: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let active_collision_types = &*(active_collision_types as *mut ActiveCollisionTypes);

    let val = it.__wrapped_active_collision_types(active_collision_types.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1friction<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, friction: jfloat) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_friction(friction);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1friction_1combine_1rule<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, rule: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let rule = &*(rule as *mut CoefficientCombineRule);

    let val = it.__wrapped_friction_combine_rule(rule.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1restitution<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, restitution: jfloat) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_restitution(restitution);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1restitution_1combine_1rule<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, rule: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let rule = &*(rule as *mut CoefficientCombineRule);

    let val = it.__wrapped_restitution_combine_rule(rule.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1density<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, density: jfloat) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_density(density);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1mass<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, mass: jfloat) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_mass(mass);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1mass_1properties<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, mass_properties: jlong) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);
    let mass_properties = &*(mass_properties as *mut MassProperties);

    let val = it.__wrapped_mass_properties(mass_properties.clone());
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1contact_1force_1event_1threshold<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, threshold: jfloat) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_contact_force_event_threshold(threshold);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1enabled<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, enabled: jboolean) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_enabled(enabled == 1);
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    

    let val = val;
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1clone<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_ColliderBuilder);

    let val = it.__wrapped_clone();
    (Box::leak(Box::new(val)) as *mut ColliderBuilder) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1clone_1from<'local, >(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, other: jlong) -> () {
    let it = &mut *(ptr as *mut __JNI_ColliderBuilder);
    let other = &*(other as *mut ColliderBuilder);

    it.__wrapped_clone_from(&other)
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_ColliderBuilder_jni_1free<'local, >(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_ColliderBuilder);
    
}

#[allow(non_camel_case_types)]
pub struct __JNI_Triple<A: Clone, B: Clone, C: Clone> {
    pub a: *mut A,
    pub b: *mut B,
    pub c: *mut C,
}

impl<A: Clone, B: Clone, C: Clone> __JNI_Triple<A, B, C> {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> Triple<A, B, C> {
        Triple {
            a: (&mut *self.a).clone(),
            b: (&mut *self.b).clone(),
            c: (&mut *self.c).clone(),
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_of(first: A, second: B, third: C) -> Self {
        let base = Triple::of(first, second, third);

        Self {
            a: Box::leak(Box::new(base.a)) as *mut A,
            b: Box::leak(Box::new(base.b)) as *mut B,
            c: Box::leak(Box::new(base.c)) as *mut C,
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_first(&self, ) -> A {
        Triple::first(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_second(&self, ) -> B {
        Triple::second(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_third(&self, ) -> C {
        Triple::third(&self.to_rust()).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_1init_1of<'local, A: Clone, B: Clone, C: Clone>(mut env: JNIEnv<'local>, obj: JObject<'local>, first: jlong, second: jlong, third: jlong) -> jlong {
        let first = &*(first as *mut A);
    let second = &*(second as *mut B);
    let third = &*(third as *mut C);
    let it = __JNI_Triple::__wrapped_of(first.clone(), second.clone(), third.clone());
    (Box::leak(Box::new(it)) as *mut __JNI_Triple<A, B, C>) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_1first<'local, A: Clone, B: Clone, C: Clone>(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Triple<A, B, C>);

    let val = it.__wrapped_first();
    (Box::leak(Box::new(val)) as *mut A) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_1second<'local, A: Clone, B: Clone, C: Clone>(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Triple<A, B, C>);

    let val = it.__wrapped_second();
    (Box::leak(Box::new(val)) as *mut B) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_1third<'local, A: Clone, B: Clone, C: Clone>(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Triple<A, B, C>);

    let val = it.__wrapped_third();
    (Box::leak(Box::new(val)) as *mut C) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Triple_jni_1free<'local, A: Clone, B: Clone, C: Clone>(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_Triple<A, B, C>);
    let _ = Box::from_raw(it.a);
let _ = Box::from_raw(it.b);
let _ = Box::from_raw(it.c);
}

#[allow(non_camel_case_types)]
pub struct __JNI_Tuple<A: Clone, B: Clone> {
    pub a: *mut A,
    pub b: *mut B,
}

impl<A: Clone, B: Clone> __JNI_Tuple<A, B> {
    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn to_rust(&self) -> Tuple<A, B> {
        Tuple {
            a: (&mut *self.a).clone(),
            b: (&mut *self.b).clone(),
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_of(first: A, second: B) -> Self {
        let base = Tuple::of(first, second);

        Self {
            a: Box::leak(Box::new(base.a)) as *mut A,
            b: Box::leak(Box::new(base.b)) as *mut B,
        }
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_first(&self, ) -> A {
        Tuple::first(&self.to_rust()).clone()
    }

    #[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]
    pub unsafe fn __wrapped_second(&self, ) -> B {
        Tuple::second(&self.to_rust()).clone()
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_1init_1of<'local, A: Clone, B: Clone>(mut env: JNIEnv<'local>, obj: JObject<'local>, first: jlong, second: jlong) -> jlong {
        let first = &*(first as *mut A);
    let second = &*(second as *mut B);
    let it = __JNI_Tuple::__wrapped_of(first.clone(), second.clone());
    (Box::leak(Box::new(it)) as *mut __JNI_Tuple<A, B>) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_1first<'local, A: Clone, B: Clone>(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Tuple<A, B>);

    let val = it.__wrapped_first();
    (Box::leak(Box::new(val)) as *mut A) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_1second<'local, A: Clone, B: Clone>(mut env: JNIEnv<'local>, class: JClass<'local>, ptr: jlong, ) -> jlong {
    let it = &*(ptr as *mut __JNI_Tuple<A, B>);

    let val = it.__wrapped_second();
    (Box::leak(Box::new(val)) as *mut B) as jlong
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
    missing_docs,
)]
pub unsafe extern "system" fn Java_com_dimforge_rapier3d_Tuple_jni_1free<'local, A: Clone, B: Clone>(_env: JNIEnv<'local>, _class: JClass<'local>, ptr: jlong) {
    let it = Box::from_raw(ptr as *mut __JNI_Tuple<A, B>);
    let _ = Box::from_raw(it.a);
let _ = Box::from_raw(it.b);
}

