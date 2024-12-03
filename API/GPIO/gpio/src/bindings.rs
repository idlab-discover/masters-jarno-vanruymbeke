#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod gpio {
        #[allow(dead_code, clippy::all)]
        pub mod gpio {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// Embedded HAL mapping
            /// Error kind.
            #[derive(Clone, Copy)]
            pub enum ErrorKind {
                /// A different error occurred. The original error may contain more information.
                Other,
            }
            impl ::core::fmt::Debug for ErrorKind {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        ErrorKind::Other => f.debug_tuple("ErrorKind::Other").finish(),
                    }
                }
            }
            impl ::core::fmt::Display for ErrorKind {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for ErrorKind {}
            /// Embedded HAL mapping
            /// Digital output pin state.
            #[derive(Clone, Copy)]
            pub enum PinState {
                /// Low pin state.
                Low,
                /// High pin state.
                High,
            }
            impl ::core::fmt::Debug for PinState {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        PinState::Low => f.debug_tuple("PinState::Low").finish(),
                        PinState::High => f.debug_tuple("PinState::High").finish(),
                    }
                }
            }
            /// Embedded HAL mapping
            /// Single digital push-pull output pin.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutputPin {
                handle: _rt::Resource<OutputPin>,
            }
            impl OutputPin {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutputPin {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[resource-drop]output-pin"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Embedded HAL mapping
            /// Push-pull output pin that can read its output state.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct StatefulOutputPin {
                handle: _rt::Resource<StatefulOutputPin>,
            }
            impl StatefulOutputPin {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for StatefulOutputPin {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[resource-drop]stateful-output-pin"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Embedded HAL mapping
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct InputPin {
                handle: _rt::Resource<InputPin>,
            }
            impl InputPin {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for InputPin {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[resource-drop]input-pin"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl OutputPin {
                #[allow(unused_unsafe, clippy::all)]
                /// Drives the pin low.
                ///
                /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
                /// electrical sources.
                pub fn set_low(&self) -> Result<(), ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]output-pin.set-low"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v3 = match l2 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v3
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputPin {
                #[allow(unused_unsafe, clippy::all)]
                /// Drives the pin high.
                ///
                /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
                /// electrical sources.
                pub fn set_high(&self) -> Result<(), ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]output-pin.set-high"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v3 = match l2 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v3
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputPin {
                #[allow(unused_unsafe, clippy::all)]
                /// Drives the pin high or low depending on the provided value.
                ///
                /// *NOTE* the actual electrical state of the pin may not actually be high or low, e.g. due to external
                /// electrical sources.
                pub fn set_state(&self, state: PinState) -> Result<(), ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let result0 = match state {
                            PinState::Low => 0i32,
                            PinState::High => 1i32,
                        };
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]output-pin.set-state"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, result0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl StatefulOutputPin {
                #[allow(unused_unsafe, clippy::all)]
                /// Have the same function available as output-pin
                /// use output-pin.{set-low, set-high, set-state};
                /// Is the pin in drive high mode?
                ///
                /// *NOTE* this does *not* read the electrical state of the pin.
                pub fn is_set_high(&self) -> Result<bool, ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]stateful-output-pin.is-set-high"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl StatefulOutputPin {
                #[allow(unused_unsafe, clippy::all)]
                /// Is the pin in drive low mode?
                ///
                /// *NOTE* this does *not* read the electrical state of the pin.
                pub fn is_set_low(&self) -> Result<bool, ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]stateful-output-pin.is-set-low"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl StatefulOutputPin {
                #[allow(unused_unsafe, clippy::all)]
                /// Toggle pin output.
                pub fn toggle(&self) -> Result<(), ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]stateful-output-pin.toggle"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v3 = match l2 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v3
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputPin {
                #[allow(unused_unsafe, clippy::all)]
                pub fn is_high(&self) -> Result<bool, ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]input-pin.is-high"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputPin {
                #[allow(unused_unsafe, clippy::all)]
                pub fn is_low(&self) -> Result<bool, ErrorKind> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:gpio/gpio")]
                        extern "C" {
                            #[link_name = "[method]input-pin.is-low"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        n => {
                                            debug_assert_eq!(n, 0, "invalid enum discriminant");
                                            ErrorKind::Other
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:wasi:gpio:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 650] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x8c\x04\x01A\x02\x01\
A\x02\x01B\x19\x01q\x01\x05other\0\0\x04\0\x0aerror-kind\x03\0\0\x01q\x02\x03low\
\0\0\x04high\0\0\x04\0\x09pin-state\x03\0\x02\x04\0\x0aoutput-pin\x03\x01\x04\0\x13\
stateful-output-pin\x03\x01\x04\0\x09input-pin\x03\x01\x01h\x04\x01j\0\x01\x01\x01\
@\x01\x04self\x07\0\x08\x04\0\x1a[method]output-pin.set-low\x01\x09\x04\0\x1b[me\
thod]output-pin.set-high\x01\x09\x01@\x02\x04self\x07\x05state\x03\0\x08\x04\0\x1c\
[method]output-pin.set-state\x01\x0a\x01h\x05\x01j\x01\x7f\x01\x01\x01@\x01\x04s\
elf\x0b\0\x0c\x04\0'[method]stateful-output-pin.is-set-high\x01\x0d\x04\0&[metho\
d]stateful-output-pin.is-set-low\x01\x0d\x01@\x01\x04self\x0b\0\x08\x04\0\"[meth\
od]stateful-output-pin.toggle\x01\x0e\x01h\x06\x01@\x01\x04self\x0f\0\x0c\x04\0\x19\
[method]input-pin.is-high\x01\x10\x04\0\x18[method]input-pin.is-low\x01\x10\x03\x01\
\x0ewasi:gpio/gpio\x05\0\x04\x01\x11wasi:gpio/example\x04\0\x0b\x0d\x01\0\x07exa\
mple\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\
\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
