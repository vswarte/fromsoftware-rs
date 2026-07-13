#![allow(non_snake_case)]

use std::{ffi::c_void, fmt, marker::PhantomData, mem::size_of, ptr::NonNull};

/// Opaque `_Func_base`-derived impl object.
#[repr(C)]
pub struct UnknownFuncImpl {
    #[allow(dead_code)]
    vftable: *const c_void,
}

#[repr(C)]
struct MsvcTypeInfo {
    vftable: *const c_void,
    decorated_name_cache: *mut c_void,
    decorated_name: [u8; 18],
}

unsafe impl Sync for MsvcTypeInfo {}

static RUST_CLOSURE_TYPE_INFO: MsvcTypeInfo = MsvcTypeInfo {
    vftable: std::ptr::null(),
    decorated_name_cache: std::ptr::null_mut(),
    decorated_name: *b".?AVRustClosure@@\0",
};

/// Marshals a single C++ parameter across `_Do_call`.
///
/// # Safety
///
/// `from_wire` must only be called with a pointer `to_wire` produced for this
/// exact type, valid for the duration of the call.
pub unsafe trait FnArg: Sized {
    fn to_wire(&mut self) -> *mut Self;

    /// # Safety
    ///
    /// `wire` must point at a live `Self`.
    unsafe fn from_wire(wire: *mut Self) -> Self;
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Ref<T>(pub NonNull<T>);

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Ref<T> {}

unsafe impl<T> FnArg for Ref<T> {
    fn to_wire(&mut self) -> *mut Self {
        self.0.as_ptr().cast()
    }
    unsafe fn from_wire(wire: *mut Self) -> Self {
        Ref(unsafe { NonNull::new_unchecked(wire.cast::<T>()) })
    }
}

macro_rules! impl_fn_arg_by_value {
    ($($t:ty),* $(,)?) => {$(
        unsafe impl FnArg for $t {
            fn to_wire(&mut self) -> *mut Self {
                self
            }
            unsafe fn from_wire(wire: *mut Self) -> Self {
                unsafe { wire.read() }
            }
        }
    )*};
}

impl_fn_arg_by_value!(
    (),
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    isize,
    usize,
    f32,
    f64,
    char,
);

unsafe impl<T> FnArg for *const T {
    fn to_wire(&mut self) -> *mut Self {
        self
    }
    unsafe fn from_wire(wire: *mut Self) -> Self {
        unsafe { wire.read() }
    }
}

unsafe impl<T> FnArg for *mut T {
    fn to_wire(&mut self) -> *mut Self {
        self
    }
    unsafe fn from_wire(wire: *mut Self) -> Self {
        unsafe { wire.read() }
    }
}

unsafe impl<T> FnArg for NonNull<T> {
    fn to_wire(&mut self) -> *mut Self {
        self
    }
    unsafe fn from_wire(wire: *mut Self) -> Self {
        unsafe { wire.read() }
    }
}

unsafe impl<T> FnArg for Option<NonNull<T>> {
    fn to_wire(&mut self) -> *mut Self {
        self
    }
    unsafe fn from_wire(wire: *mut Self) -> Self {
        unsafe { wire.read() }
    }
}

/// Types MSVC returns from `_Do_call` in a register instead of hidden
/// `sret` pointer: should be copyable, 0/1/2/4/8 bytes.
///
/// # Safety
///
/// Implementing this asserts the type is copyable with size in
/// `{0, 1, 2, 4, 8}`.
pub unsafe trait FnRet: Copy + 'static {}

macro_rules! impl_fn_ret {
    ($($t:ty),* $(,)?) => {$(
        unsafe impl FnRet for $t {}
    )*};
}

impl_fn_ret!(
    (),
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    isize,
    usize,
    f32,
    f64,
    char,
);
unsafe impl<T: 'static> FnRet for *const T {}
unsafe impl<T: 'static> FnRet for *mut T {}
unsafe impl<T: 'static> FnRet for NonNull<T> {}
unsafe impl<T: 'static> FnRet for Option<NonNull<T>> {}

#[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
const SMALL_OBJECT_NUM_PTRS: usize = 6 + 16 / size_of::<*const c_void>();
#[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
const SMALL_OBJECT_NUM_PTRS: usize = 3;

const _: () = assert!(size_of::<usize>() == 8);
#[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
const _: () = assert!(SMALL_OBJECT_NUM_PTRS == 8);

/// Trait that Rust closures must satisfy to back a `Function<S>`.
pub trait FnCallable<S: FnSig>: 'static {
    fn call_callee(&self, args: S::Args) -> S::Ret;
}

#[repr(C)]
struct FuncVtable<DoCall> {
    pub copy: unsafe extern "C" fn(*const UnknownFuncImpl, *mut c_void) -> *mut UnknownFuncImpl,
    pub move_: unsafe extern "C" fn(*mut UnknownFuncImpl, *mut c_void) -> *mut UnknownFuncImpl,
    pub do_call: DoCall,
    pub target_type: unsafe extern "C" fn(*const UnknownFuncImpl) -> *const MsvcTypeInfo,
    pub delete_this: unsafe extern "C" fn(*mut UnknownFuncImpl, bool),
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    pub scalar_deleting_destructor: unsafe extern "C" fn(*mut UnknownFuncImpl, u32) -> *mut UnknownFuncImpl,
    pub get: unsafe extern "C" fn(*const UnknownFuncImpl) -> *const c_void,
}

/// Ties a Rust fn-pointer type `fn(Args...) -> Ret` to its vtable shape.
/// Implemented per arity (0-8) by [`impl_function_arity!`].
///
/// # Safety
///
/// `DoCall` must match `FuncVtable`'s `do_call` slot for this signature.
pub unsafe trait FnSig: 'static {
    type Ret: FnRet;
    type Args;
    type DoCall: Copy;

    /// # Safety
    /// `this` must be non-null and live.
    unsafe fn invoke(this: *mut UnknownFuncImpl, args: Self::Args) -> Self::Ret;

    /// # Safety
    /// `this` must be non-null and live.
    unsafe fn delete_this(this: *mut UnknownFuncImpl, dealloc: bool);

    fn new_impl<F: FnCallable<Self> + Clone + 'static>(f: F) -> *mut UnknownFuncImpl
    where
        Self: Sized;
}

/// # Safety
/// `this` must be non-null and live, first field a vtable pointer matching
/// `FuncVtable<DoCall>`.
unsafe fn read_vtable<DoCall: Copy>(this: *mut UnknownFuncImpl) -> &'static FuncVtable<DoCall> {
    unsafe { &*(*this.cast::<*const FuncVtable<DoCall>>()) }
}

/// Implementation of MSVC C++ `std::function<R(Args...)>`.
///
/// `S` is a concrete Rust fn-pointer type describing the C++ signature, e.g.
/// `Function<fn(u32) -> bool>`. Use [`Ref<T>`] for C++ reference parameters.
///
/// Under the `msvc2012` feature (VS2012 ABI), the impl
/// pointer is a separate field rather than the last slot of the inline
/// buffer, and the impl object's vtable has an extra slot for VS2012's
/// `_Func_base` destructor (MSVC's "scalar deleting destructor").
///
/// # References
///
/// - [cppreference - `std::function`]
/// - [MSVC STL source - `functional`]
/// - [Raymond Chen - Inside std::function, part 1]
/// - [Raymond Chen - Inside std::function, part 2]
///
/// [cppreference - `std::function`]: https://en.cppreference.com/w/cpp/utility/functional/function.html
/// [MSVC STL source - `functional`]: https://github.com/microsoft/STL/blob/main/stl/inc/functional
/// [Raymond Chen - Inside std::function, part 1]: https://devblogs.microsoft.com/oldnewthing/20200513-00/?p=103745
/// [Raymond Chen - Inside std::function, part 2]: https://devblogs.microsoft.com/oldnewthing/20200514-00/?p=103749
#[repr(C)]
pub struct Function<S: FnSig> {
    storage: [*mut c_void; SMALL_OBJECT_NUM_PTRS],
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    impl_ptr: *mut c_void,
    _marker: PhantomData<S>,
}

impl<S: FnSig> Function<S> {
    /// Creates a function from a Rust closure. Always heap-allocated.
    pub fn new<F: FnCallable<S> + Clone + 'static>(f: F) -> Self {
        #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
        {
            let mut storage = [std::ptr::null_mut(); SMALL_OBJECT_NUM_PTRS];
            storage[SMALL_OBJECT_NUM_PTRS - 1] = S::new_impl(f).cast();
            Self {
                storage,
                _marker: PhantomData,
            }
        }
        #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
        {
            Self {
                storage: [std::ptr::null_mut(); SMALL_OBJECT_NUM_PTRS],
                impl_ptr: S::new_impl(f).cast(),
                _marker: PhantomData,
            }
        }
    }

    /// Creates an empty function, equivalent to a default-constructed
    /// `std::function`. Calling it panics, matching `std::bad_function_call`.
    pub fn empty() -> Self {
        Self {
            storage: [std::ptr::null_mut(); SMALL_OBJECT_NUM_PTRS],
            #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
            impl_ptr: std::ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn impl_ptr(&self) -> *mut UnknownFuncImpl {
        #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
        {
            self.storage[SMALL_OBJECT_NUM_PTRS - 1].cast()
        }
        #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
        {
            self.impl_ptr.cast()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.impl_ptr().is_null()
    }

    #[inline]
    fn is_local(&self) -> bool {
        std::ptr::eq(self.impl_ptr().cast(), self.storage.as_ptr())
    }

    #[inline]
    fn impl_ptr_for_access(&mut self) -> *mut UnknownFuncImpl {
        if self.is_local() {
            self.storage.as_mut_ptr().cast()
        } else {
            self.impl_ptr()
        }
    }

    /// Calls the wrapped callable, equivalent to `std::function::operator()`.
    ///
    /// # Panics
    ///
    /// If this function is empty, matching `std::bad_function_call`.
    pub fn call(&mut self, args: S::Args) -> S::Ret {
        assert!(!self.is_empty(), "Function::call: bad function call");
        let this = self.impl_ptr_for_access();
        unsafe { S::invoke(this, args) }
    }
}

impl<S: FnSig> Drop for Function<S> {
    fn drop(&mut self) {
        if self.is_empty() {
            return;
        }
        let dealloc = !self.is_local();
        let this = self.impl_ptr_for_access();
        unsafe { S::delete_this(this, dealloc) };
    }
}

impl<S: FnSig> Default for Function<S> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<S: FnSig> fmt::Debug for Function<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "Function(empty)")
        } else if self.is_local() {
            write!(f, "Function(local @ {:p})", self.impl_ptr())
        } else {
            write!(f, "Function(heap @ {:p})", self.impl_ptr())
        }
    }
}

/// Generates, per arity, the `FnSig`/`FnCallable` impls and the `FuncImpl`
/// type backing Rust closures.
macro_rules! impl_function_arity {
    ($modname:ident, [$($a:ident),*]) => {
        mod $modname {
            use super::*;

            type DoCall<Ret, $($a,)*> =
                unsafe extern "C" fn(*mut UnknownFuncImpl $(, *mut $a)*) -> Ret;

            #[repr(C)]
            struct FuncImpl<Ret: FnRet, $($a: FnArg + 'static,)* F: 'static> {
                vftable: &'static FuncVtable<DoCall<Ret, $($a,)*>>,
                callee: F,
            }

            unsafe extern "C" fn copy<Ret: FnRet, $($a: FnArg + 'static,)* F: FnCallable<fn($($a),*) -> Ret> + Clone + 'static>(
                this: *const UnknownFuncImpl,
                _where_: *mut c_void,
            ) -> *mut UnknownFuncImpl {
                let this = this.cast::<FuncImpl<Ret, $($a,)* F>>();
                let boxed = Box::new(FuncImpl {
                    vftable: &Vtables::<Ret, $($a,)* F>::VTABLE,
                    callee: unsafe { (*this).callee.clone() },
                });
                Box::into_raw(boxed).cast()
            }

            unsafe extern "C" fn move_(
                _this: *mut UnknownFuncImpl,
                _where_: *mut c_void,
            ) -> *mut UnknownFuncImpl {
                std::ptr::null_mut()
            }

            unsafe extern "C" fn do_call<Ret: FnRet, $($a: FnArg + 'static,)* F: FnCallable<fn($($a),*) -> Ret> + 'static>(
                this: *mut UnknownFuncImpl,
                $($a: *mut $a,)*
            ) -> Ret {
                let this = this.cast::<FuncImpl<Ret, $($a,)* F>>();
                $(let $a = unsafe { $a::from_wire($a) };)*
                unsafe { (*this).callee.call_callee(($($a,)*)) }
            }

            unsafe extern "C" fn target_type(_this: *const UnknownFuncImpl) -> *const MsvcTypeInfo {
                &RUST_CLOSURE_TYPE_INFO
            }

            unsafe extern "C" fn delete_this<Ret: FnRet, $($a: FnArg + 'static,)* F: 'static>(
                this: *mut UnknownFuncImpl,
                dealloc: bool,
            ) {
                let this = this.cast::<FuncImpl<Ret, $($a,)* F>>();
                if dealloc {
                    drop(unsafe { Box::from_raw(this) });
                } else {
                    unsafe { std::ptr::drop_in_place(this) };
                }
            }

            #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
            unsafe extern "C" fn scalar_deleting_destructor<Ret: FnRet, $($a: FnArg + 'static,)* F: 'static>(
                this: *mut UnknownFuncImpl,
                flags: u32,
            ) -> *mut UnknownFuncImpl {
                unsafe { delete_this::<Ret, $($a,)* F>(this, flags & 1 != 0) };
                this
            }

            unsafe extern "C" fn get<Ret: FnRet, $($a: FnArg + 'static,)* F: 'static>(
                this: *const UnknownFuncImpl,
            ) -> *const c_void {
                let this = this.cast::<FuncImpl<Ret, $($a,)* F>>();
                unsafe { &(*this).callee as *const F as *const c_void }
            }

            struct Vtables<Ret: FnRet, $($a: FnArg + 'static,)* F: 'static>(
                std::marker::PhantomData<(Ret, $($a,)* F)>,
            );
            impl<Ret: FnRet, $($a: FnArg + 'static,)* F: FnCallable<fn($($a),*) -> Ret> + Clone + 'static>
                Vtables<Ret, $($a,)* F>
            {
                const VTABLE: FuncVtable<DoCall<Ret, $($a,)*>> = FuncVtable {
                    copy: copy::<Ret, $($a,)* F>,
                    move_,
                    do_call: do_call::<Ret, $($a,)* F>,
                    target_type,
                    delete_this: delete_this::<Ret, $($a,)* F>,
                    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
                    scalar_deleting_destructor: scalar_deleting_destructor::<Ret, $($a,)* F>,
                    get: get::<Ret, $($a,)* F>,
                };
            }

            unsafe impl<Ret: FnRet, $($a: FnArg + 'static,)*> FnSig for fn($($a),*) -> Ret {
                type Ret = Ret;
                type Args = ($($a,)*);
                type DoCall = DoCall<Ret, $($a,)*>;

                #[allow(unused_variables, unused_mut)]
                unsafe fn invoke(this: *mut UnknownFuncImpl, args: Self::Args) -> Self::Ret {
                    let vtable = unsafe { read_vtable::<Self::DoCall>(this) };
                    let ($($a,)*) = args;
                    $(let mut $a = $a;)*
                    unsafe { (vtable.do_call)(this $(, $a.to_wire())*) }
                }

                unsafe fn delete_this(this: *mut UnknownFuncImpl, dealloc: bool) {
                    let vtable = unsafe { read_vtable::<Self::DoCall>(this) };
                    unsafe { (vtable.delete_this)(this, dealloc) }
                }

                fn new_impl<Fun: FnCallable<Self> + Clone + 'static>(
                    f: Fun,
                ) -> *mut UnknownFuncImpl {
                    let boxed = Box::new(FuncImpl::<Ret, $($a,)* Fun> {
                        vftable: &Vtables::<Ret, $($a,)* Fun>::VTABLE,
                        callee: f,
                    });
                    Box::into_raw(boxed).cast()
                }
            }

            impl<Ret: FnRet, $($a: FnArg + 'static,)* F: Fn($($a),*) -> Ret + 'static>
                FnCallable<fn($($a),*) -> Ret> for F
            {
                fn call_callee(&self, args: ($($a,)*)) -> Ret {
                    let ($($a,)*) = args;
                    self($($a),*)
                }
            }
        }
    };
}

impl_function_arity!(arity0, []);
impl_function_arity!(arity1, [A1]);
impl_function_arity!(arity2, [A1, A2]);
impl_function_arity!(arity3, [A1, A2, A3]);
impl_function_arity!(arity4, [A1, A2, A3, A4]);
impl_function_arity!(arity5, [A1, A2, A3, A4, A5]);
impl_function_arity!(arity6, [A1, A2, A3, A4, A5, A6]);
impl_function_arity!(arity7, [A1, A2, A3, A4, A5, A6, A7]);
impl_function_arity!(arity8, [A1, A2, A3, A4, A5, A6, A7, A8]);
