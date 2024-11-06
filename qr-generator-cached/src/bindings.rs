#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod keyvalue {
        #[allow(dead_code, clippy::all)]
        pub mod store {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// The set of errors which may be raised by functions in this package
            #[derive(Clone)]
            pub enum Error {
                /// The host does not recognize the store identifier requested.
                NoSuchStore,
                /// The requesting component does not have access to the specified store
                /// (which may or may not exist).
                AccessDenied,
                /// Some implementation-specific error has occurred (e.g. I/O)
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Error {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Error::NoSuchStore => {
                            f.debug_tuple("Error::NoSuchStore").finish()
                        }
                        Error::AccessDenied => {
                            f.debug_tuple("Error::AccessDenied").finish()
                        }
                        Error::Other(e) => {
                            f.debug_tuple("Error::Other").field(e).finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for Error {}
            /// A response to a `list-keys` operation.
            #[derive(Clone)]
            pub struct KeyResponse {
                /// The list of keys returned by the query.
                pub keys: _rt::Vec<_rt::String>,
                /// The continuation token to use to fetch the next page of keys. If this is `null`, then
                /// there are no more keys to fetch.
                pub cursor: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for KeyResponse {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("KeyResponse")
                        .field("keys", &self.keys)
                        .field("cursor", &self.cursor)
                        .finish()
                }
            }
            /// A bucket is a collection of key-value pairs. Each key-value pair is stored as a entry in the
            /// bucket, and the bucket itself acts as a collection of all these entries.
            ///
            /// It is worth noting that the exact terminology for bucket in key-value stores can very
            /// depending on the specific implementation. For example:
            ///
            /// 1. Amazon DynamoDB calls a collection of key-value pairs a table
            /// 2. Redis has hashes, sets, and sorted sets as different types of collections
            /// 3. Cassandra calls a collection of key-value pairs a column family
            /// 4. MongoDB calls a collection of key-value pairs a collection
            /// 5. Riak calls a collection of key-value pairs a bucket
            /// 6. Memcached calls a collection of key-value pairs a slab
            /// 7. Azure Cosmos DB calls a collection of key-value pairs a container
            ///
            /// In this interface, we use the term `bucket` to refer to a collection of key-value pairs
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Bucket {
                handle: _rt::Resource<Bucket>,
            }
            impl Bucket {
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
            unsafe impl _rt::WasmResource for Bucket {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                        extern "C" {
                            #[link_name = "[resource-drop]bucket"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the bucket with the specified identifier.
            ///
            /// `identifier` must refer to a bucket provided by the host.
            ///
            /// `error::no-such-store` will be raised if the `identifier` is not recognized.
            pub fn open(identifier: &str) -> Result<Bucket, Error> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let vec0 = identifier;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                    extern "C" {
                        #[link_name = "open"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<i32>();
                                Bucket::from_handle(l3 as u32)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                let v8 = match l4 {
                                    0 => Error::NoSuchStore,
                                    1 => Error::AccessDenied,
                                    n => {
                                        debug_assert_eq!(n, 2, "invalid enum discriminant");
                                        let e8 = {
                                            let l5 = *ptr1.add(8).cast::<*mut u8>();
                                            let l6 = *ptr1.add(12).cast::<usize>();
                                            let len7 = l6;
                                            let bytes7 = _rt::Vec::from_raw_parts(
                                                l5.cast(),
                                                len7,
                                                len7,
                                            );
                                            _rt::string_lift(bytes7)
                                        };
                                        Error::Other(e8)
                                    }
                                };
                                v8
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            impl Bucket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the value associated with the specified `key`
                ///
                /// The value is returned as an option. If the key-value pair exists in the
                /// store, it returns `Ok(value)`. If the key does not exist in the
                /// store, it returns `Ok(none)`.
                ///
                /// If any other error occurs, it returns an `Err(error)`.
                pub fn get(&self, key: &str) -> Result<Option<_rt::Vec<u8>>, Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                        extern "C" {
                            #[link_name = "[method]bucket.get"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    match l3 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l4 = *ptr1.add(8).cast::<*mut u8>();
                                                let l5 = *ptr1.add(12).cast::<usize>();
                                                let len6 = l5;
                                                _rt::Vec::from_raw_parts(l4.cast(), len6, len6)
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v11 = match l7 {
                                        0 => Error::NoSuchStore,
                                        1 => Error::AccessDenied,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e11 = {
                                                let l8 = *ptr1.add(8).cast::<*mut u8>();
                                                let l9 = *ptr1.add(12).cast::<usize>();
                                                let len10 = l9;
                                                let bytes10 = _rt::Vec::from_raw_parts(
                                                    l8.cast(),
                                                    len10,
                                                    len10,
                                                );
                                                _rt::string_lift(bytes10)
                                            };
                                            Error::Other(e11)
                                        }
                                    };
                                    v11
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Bucket {
                #[allow(unused_unsafe, clippy::all)]
                /// Set the value associated with the key in the store. If the key already
                /// exists in the store, it overwrites the value.
                ///
                /// If the key does not exist in the store, it creates a new key-value pair.
                ///
                /// If any other error occurs, it returns an `Err(error)`.
                pub fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = value;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                        extern "C" {
                            #[link_name = "[method]bucket.set"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(4).cast::<u8>());
                                    let v8 = match l4 {
                                        0 => Error::NoSuchStore,
                                        1 => Error::AccessDenied,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e8 = {
                                                let l5 = *ptr2.add(8).cast::<*mut u8>();
                                                let l6 = *ptr2.add(12).cast::<usize>();
                                                let len7 = l6;
                                                let bytes7 = _rt::Vec::from_raw_parts(
                                                    l5.cast(),
                                                    len7,
                                                    len7,
                                                );
                                                _rt::string_lift(bytes7)
                                            };
                                            Error::Other(e8)
                                        }
                                    };
                                    v8
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Bucket {
                #[allow(unused_unsafe, clippy::all)]
                /// Delete the key-value pair associated with the key in the store.
                ///
                /// If the key does not exist in the store, it does nothing.
                ///
                /// If any other error occurs, it returns an `Err(error)`.
                pub fn delete(&self, key: &str) -> Result<(), Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                        extern "C" {
                            #[link_name = "[method]bucket.delete"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v7 = match l3 {
                                        0 => Error::NoSuchStore,
                                        1 => Error::AccessDenied,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e7 = {
                                                let l4 = *ptr1.add(8).cast::<*mut u8>();
                                                let l5 = *ptr1.add(12).cast::<usize>();
                                                let len6 = l5;
                                                let bytes6 = _rt::Vec::from_raw_parts(
                                                    l4.cast(),
                                                    len6,
                                                    len6,
                                                );
                                                _rt::string_lift(bytes6)
                                            };
                                            Error::Other(e7)
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Bucket {
                #[allow(unused_unsafe, clippy::all)]
                /// Check if the key exists in the store.
                ///
                /// If the key exists in the store, it returns `Ok(true)`. If the key does
                /// not exist in the store, it returns `Ok(false)`.
                ///
                /// If any other error occurs, it returns an `Err(error)`.
                pub fn exists(&self, key: &str) -> Result<bool, Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = key;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                        extern "C" {
                            #[link_name = "[method]bucket.exists"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    _rt::bool_lift(l3 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v8 = match l4 {
                                        0 => Error::NoSuchStore,
                                        1 => Error::AccessDenied,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e8 = {
                                                let l5 = *ptr1.add(8).cast::<*mut u8>();
                                                let l6 = *ptr1.add(12).cast::<usize>();
                                                let len7 = l6;
                                                let bytes7 = _rt::Vec::from_raw_parts(
                                                    l5.cast(),
                                                    len7,
                                                    len7,
                                                );
                                                _rt::string_lift(bytes7)
                                            };
                                            Error::Other(e8)
                                        }
                                    };
                                    v8
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Bucket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get all the keys in the store with an optional cursor (for use in pagination). It
                /// returns a list of keys. Please note that for most KeyValue implementations, this is a
                /// can be a very expensive operation and so it should be used judiciously. Implementations
                /// can return any number of keys in a single response, but they should never attempt to
                /// send more data than is reasonable (i.e. on a small edge device, this may only be a few
                /// KB, while on a large machine this could be several MB). Any response should also return
                /// a cursor that can be used to fetch the next page of keys. See the `key-response` record
                /// for more information.
                ///
                /// Note that the keys are not guaranteed to be returned in any particular order.
                ///
                /// If the store is empty, it returns an empty list.
                ///
                /// MAY show an out-of-date list of keys if there are concurrent writes to the store.
                ///
                /// If any error occurs, it returns an `Err(error)`.
                pub fn list_keys(
                    &self,
                    cursor: Option<&str>,
                ) -> Result<KeyResponse, Error> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 24],
                        );
                        let (result1_0, result1_1, result1_2) = match cursor {
                            Some(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (1i32, ptr0.cast_mut(), len0)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:keyvalue/store@0.2.0-draft2")]
                        extern "C" {
                            #[link_name = "[method]bucket.list-keys"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = *ptr2.add(4).cast::<*mut u8>();
                                    let l5 = *ptr2.add(8).cast::<usize>();
                                    let base9 = l4;
                                    let len9 = l5;
                                    let mut result9 = _rt::Vec::with_capacity(len9);
                                    for i in 0..len9 {
                                        let base = base9.add(i * 8);
                                        let e9 = {
                                            let l6 = *base.add(0).cast::<*mut u8>();
                                            let l7 = *base.add(4).cast::<usize>();
                                            let len8 = l7;
                                            let bytes8 = _rt::Vec::from_raw_parts(
                                                l6.cast(),
                                                len8,
                                                len8,
                                            );
                                            _rt::string_lift(bytes8)
                                        };
                                        result9.push(e9);
                                    }
                                    _rt::cabi_dealloc(base9, len9 * 8, 4);
                                    let l10 = i32::from(*ptr2.add(12).cast::<u8>());
                                    KeyResponse {
                                        keys: result9,
                                        cursor: match l10 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l11 = *ptr2.add(16).cast::<*mut u8>();
                                                    let l12 = *ptr2.add(20).cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l14 = i32::from(*ptr2.add(4).cast::<u8>());
                                    let v18 = match l14 {
                                        0 => Error::NoSuchStore,
                                        1 => Error::AccessDenied,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e18 = {
                                                let l15 = *ptr2.add(8).cast::<*mut u8>();
                                                let l16 = *ptr2.add(12).cast::<usize>();
                                                let len17 = l16;
                                                let bytes17 = _rt::Vec::from_raw_parts(
                                                    l15.cast(),
                                                    len17,
                                                    len17,
                                                );
                                                _rt::string_lift(bytes17)
                                            };
                                            Error::Other(e18)
                                        }
                                    };
                                    v18
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
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod example {
        #[allow(dead_code)]
        pub mod qr {
            #[allow(dead_code, clippy::all)]
            pub mod qr {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Bucket = super::super::super::super::wasi::keyvalue::store::Bucket;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_qr_code_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::get_qr_code(
                        super::super::super::super::wasi::keyvalue::store::Bucket::from_handle(
                            arg0 as u32,
                        ),
                        _rt::string_lift(bytes0),
                        arg3 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_qr_code<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn get_qr_code(
                        bucket: Bucket,
                        url: _rt::String,
                        size: u32,
                    ) -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_example_qr_qr_1_0_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "example:qr/qr@1.0.0#get-qr-code"] unsafe extern "C" fn
                        export_get_qr_code(arg0 : i32, arg1 : * mut u8, arg2 : usize,
                        arg3 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_get_qr_code_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name = "cabi_post_example:qr/qr@1.0.0#get-qr-code"]
                        unsafe extern "C" fn _post_return_get_qr_code(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_qr_code::<$ty > (arg0) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_example_qr_qr_1_0_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
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
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
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
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_example_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::example::qr::qr::__export_example_qr_qr_1_0_0_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::example::qr::qr);
    };
}
#[doc(inline)]
pub(crate) use __export_example_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:qr-generator-cached:example:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 703] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc1\x04\x01A\x02\x01\
A\x05\x01B\x1c\x01q\x03\x0dno-such-store\0\0\x0daccess-denied\0\0\x05other\x01s\0\
\x04\0\x05error\x03\0\0\x01ps\x01ks\x01r\x02\x04keys\x02\x06cursor\x03\x04\0\x0c\
key-response\x03\0\x04\x04\0\x06bucket\x03\x01\x01h\x06\x01p}\x01k\x08\x01j\x01\x09\
\x01\x01\x01@\x02\x04self\x07\x03keys\0\x0a\x04\0\x12[method]bucket.get\x01\x0b\x01\
j\0\x01\x01\x01@\x03\x04self\x07\x03keys\x05value\x08\0\x0c\x04\0\x12[method]buc\
ket.set\x01\x0d\x01@\x02\x04self\x07\x03keys\0\x0c\x04\0\x15[method]bucket.delet\
e\x01\x0e\x01j\x01\x7f\x01\x01\x01@\x02\x04self\x07\x03keys\0\x0f\x04\0\x15[meth\
od]bucket.exists\x01\x10\x01j\x01\x05\x01\x01\x01@\x02\x04self\x07\x06cursor\x03\
\0\x11\x04\0\x18[method]bucket.list-keys\x01\x12\x01i\x06\x01j\x01\x13\x01\x01\x01\
@\x01\x0aidentifiers\0\x14\x04\0\x04open\x01\x15\x03\x01\x20wasi:keyvalue/store@\
0.2.0-draft2\x05\0\x02\x03\0\0\x06bucket\x01B\x05\x02\x03\x02\x01\x01\x04\0\x06b\
ucket\x03\0\0\x01i\x01\x01@\x03\x06bucket\x02\x03urls\x04sizey\0s\x04\0\x0bget-q\
r-code\x01\x03\x04\x01\x13example:qr/qr@1.0.0\x05\x02\x04\x01%component:qr-gener\
ator-cached/example\x04\0\x0b\x0d\x01\0\x07example\x03\0\0\0G\x09producers\x01\x0c\
processed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
