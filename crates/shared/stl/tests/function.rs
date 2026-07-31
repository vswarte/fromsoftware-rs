use fromsoftware_shared_stl::{FnCallable, FnTarget, Function, Ref};
use std::sync::atomic::{AtomicUsize, Ordering};
struct Tracked<'a>(&'a AtomicUsize);

impl Clone for Tracked<'_> {
    fn clone(&self) -> Self {
        Tracked(self.0)
    }
}

impl Drop for Tracked<'_> {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn function_size_and_align() {
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    assert_eq!(std::mem::size_of::<Function<fn()>>(), 64);
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    assert_eq!(std::mem::size_of::<Function<fn()>>(), 32);
    assert_eq!(std::mem::align_of::<Function<fn()>>(), 8);
}

#[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
#[test]
fn function_impl_ptr_at_offset_56() {
    let f = Function::<fn()>::new(|| {});
    let base = &f as *const Function<fn()>;
    let slot7 = unsafe { base.cast::<*mut std::ffi::c_void>().add(7) };
    assert_eq!(slot7 as usize - base as usize, 56);
    assert!(
        !unsafe { *slot7 }.is_null(),
        "non-empty function must have a non-null impl pointer"
    );
}

#[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
#[test]
fn function_impl_ptr_at_offset_24() {
    let f = Function::<fn()>::new(|| {});
    let base = &f as *const Function<fn()>;
    let impl_ptr_slot = unsafe { base.cast::<*mut std::ffi::c_void>().add(3) };
    assert_eq!(impl_ptr_slot as usize - base as usize, 24);
    assert!(
        !unsafe { *impl_ptr_slot }.is_null(),
        "non-empty function must have a non-null impl pointer"
    );
}

#[test]
fn function_call_arity0() {
    let mut f = Function::<fn() -> i32>::new(|| 42);
    assert_eq!(f.call(()), 42);
}

#[test]
fn function_call_arity1_primitive() {
    let mut f = Function::<fn(i32) -> i32>::new(|x: i32| x * 2);
    assert_eq!(f.call((21,)), 42);
}

#[test]
fn function_call_arity2_primitive() {
    let mut f = Function::<fn(i32, i32) -> i32>::new(|a: i32, b: i32| a + b);
    assert_eq!(f.call((10, 32)), 42);
}

#[test]
fn function_call_pointer_arg_and_return() {
    let mut value = 7i32;
    let ptr: *mut i32 = &mut value;
    let mut f = Function::<fn(*mut i32) -> *mut i32>::new(|p: *mut i32| p);
    assert_eq!(f.call((ptr,)), ptr);
}

#[test]
fn function_call_float_args_and_return() {
    let mut f = Function::<fn(f32, f64) -> f64>::new(|a: f32, b: f64| a as f64 + b);
    assert_eq!(f.call((1.5f32, 2.5f64)), 4.0);
}

#[test]
fn function_call_ref_arg_mutates_referent() {
    let mut value = 10i32;
    let mut f = Function::<fn(Ref<i32>)>::new(|r: Ref<i32>| unsafe {
        *r.0.as_ptr() += 5;
    });
    f.call((Ref(std::ptr::NonNull::from(&mut value)),));
    assert_eq!(value, 15);
}

#[test]
fn function_call_stateful_closure_across_calls() {
    let count = std::cell::Cell::new(0i32);
    let mut f = Function::<fn() -> i32>::new(move || {
        count.set(count.get() + 1);
        count.get()
    });
    assert_eq!(f.call(()), 1);
    assert_eq!(f.call(()), 2);
    assert_eq!(f.call(()), 3);
}

#[test]
fn function_call_many_args() {
    let mut f = Function::<fn(i32, i32, i32, i32, i32, i32, i32, i32) -> i32>::new(
        |a, b, c, d, e, g, h, i| a + b + c + d + e + g + h + i,
    );
    assert_eq!(f.call((1, 2, 3, 4, 5, 6, 7, 8)), 36);
}

#[test]
fn function_drop_runs_closure_destructor_once() {
    static C: AtomicUsize = AtomicUsize::new(0);
    {
        let held = Tracked(&C);
        let f = Function::<fn()>::new(move || {
            let _ = &held;
        });
        drop(f);
    }
    assert_eq!(C.load(Ordering::Relaxed), 1);
}

#[test]
fn function_empty_drop_is_noop() {
    let f = Function::<fn() -> i32>::empty();
    drop(f);
}

#[test]
fn function_default_is_empty() {
    let f: Function<fn()> = Default::default();
    assert!(f.is_empty());
}

#[test]
#[allow(unused_assignments)]
fn function_overwrite_through_mut_ref_drops_old_value() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let held = Tracked(&C);
    let mut f = Function::<fn()>::new(move || {
        let _ = &held;
    });
    assert_eq!(C.load(Ordering::Relaxed), 0);
    f = Function::<fn()>::new(|| {});
    assert_eq!(
        C.load(Ordering::Relaxed),
        1,
        "old closure must be dropped on overwrite"
    );
    drop(f);
}

#[test]
fn function_is_empty() {
    let empty = Function::<fn()>::empty();
    let full = Function::<fn()>::new(|| {});
    assert!(empty.is_empty());
    assert!(!full.is_empty());
}

#[test]
#[should_panic(expected = "bad function call")]
fn function_call_on_empty_panics() {
    let mut f = Function::<fn() -> i32>::empty();
    f.call(());
}

#[test]
fn function_target_ptr_points_at_captured_state() {
    let f = Function::<fn() -> i32>::new(|| 99);
    f.target_ptr().expect("non-empty function has a target");
}

#[test]
fn function_target_ptr_none_when_empty() {
    let f = Function::<fn() -> i32>::empty();
    assert!(f.target_ptr().is_none());
}

#[repr(C)]
#[derive(Clone)]
struct CountingCallable {
    n: i32,
}

impl FnCallable<fn() -> i32> for CountingCallable {
    fn call_callee(&self, _: ()) -> i32 {
        self.n
    }
}

unsafe impl FnTarget for CountingCallable {}

#[test]
fn function_typed_target_reads_captured_state() {
    let mut f: Function<fn() -> i32, CountingCallable> =
        Function::new_with_target(CountingCallable { n: 7 });
    assert_eq!(f.target().unwrap().n, 7);
    assert_eq!(f.call(()), 7);
}

#[test]
fn function_typed_target_none_when_empty() {
    let f: Function<fn() -> i32, CountingCallable> = Function::empty();
    assert!(f.target().is_none());
}

#[test]
fn function_target_ptr_reads_captured_value() {
    let n = 4242i32;
    let f = Function::<fn() -> i32>::new(move || n);
    let ptr = f.target_ptr().unwrap();
    let read_back = unsafe { *ptr.cast::<i32>().as_ref() };
    assert_eq!(read_back, 4242);
}

#[repr(C)]
struct RawFuncBaseVmt<Ret> {
    copy: unsafe extern "C" fn(*const RawImplHeader, *mut std::ffi::c_void) -> *mut RawImplHeader,
    move_: unsafe extern "C" fn(*mut RawImplHeader, *mut std::ffi::c_void) -> *mut RawImplHeader,
    do_call: unsafe extern "C" fn(*mut RawImplHeader) -> Ret,
    target_type: unsafe extern "C" fn(*const RawImplHeader) -> *const std::ffi::c_void,
    delete_this: unsafe extern "C" fn(*mut RawImplHeader, bool),
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    scalar_deleting_destructor: unsafe extern "C" fn(*mut RawImplHeader, u32) -> *mut RawImplHeader,
    get: unsafe extern "C" fn(*const RawImplHeader) -> *const std::ffi::c_void,
}

#[repr(C)]
struct RawImplHeader {
    vtable: *const std::ffi::c_void,
}

fn raw_impl_ptr<S: fromsoftware_shared_stl::FnSig>(f: &Function<S>) -> *mut RawImplHeader {
    let raw = f as *const Function<S> as *const *mut RawImplHeader;
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    let slot = 7;
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    let slot = 3;
    unsafe { *raw.add(slot) }
}

fn leak<S: fromsoftware_shared_stl::FnSig>(f: Function<S>) {
    std::mem::forget(f);
}

#[test]
fn simulated_cpp_copy_and_delete_this() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    let f = Function::<fn() -> i32>::new({
        let held = Tracked(&DROPS);
        move || {
            let _ = &held;
            42
        }
    });

    let impl_ptr = raw_impl_ptr(&f);
    let vtable = unsafe { *(impl_ptr as *const *const RawFuncBaseVmt<i32>) };
    let vtable = unsafe { &*vtable };

    let mut dest_storage = [0u8; 64];
    let copied =
        unsafe { (vtable.copy)(impl_ptr as *const _, dest_storage.as_mut_ptr() as *mut _) };
    assert!(
        !std::ptr::eq(copied as *const u8, dest_storage.as_ptr()),
        "rust impl is not heap-allocated"
    );

    let copied_result = unsafe { (vtable.do_call)(copied) };
    assert_eq!(copied_result, 42);
    assert_eq!(DROPS.load(Ordering::Relaxed), 0, "no drop yet");

    unsafe { (vtable.delete_this)(copied, true) };
    assert_eq!(
        DROPS.load(Ordering::Relaxed),
        1,
        "copy's callee must be dropped"
    );

    leak(f);
    unsafe { (vtable.delete_this)(impl_ptr, true) };
    assert_eq!(
        DROPS.load(Ordering::Relaxed),
        2,
        "original's callee must be dropped independently of the copy"
    );
}

#[test]
fn simulated_cpp_move_steals_pointer() {
    let f = Function::<fn() -> i32>::new(|| 7);
    let impl_ptr = raw_impl_ptr(&f);

    let vtable = unsafe { *(impl_ptr as *const *const RawFuncBaseVmt<i32>) };
    let vtable = unsafe { &*vtable };
    let result = unsafe { (vtable.do_call)(impl_ptr) };
    assert_eq!(result, 7);

    unsafe { (vtable.delete_this)(impl_ptr, true) };
    leak(f);
}

#[test]
fn simulated_cpp_get_returns_callee_pointer() {
    let f = Function::<fn() -> i32>::new(|| 99);
    let impl_ptr = raw_impl_ptr(&f);
    let vtable = unsafe { *(impl_ptr as *const *const RawFuncBaseVmt<i32>) };
    let vtable = unsafe { &*vtable };

    let callee_ptr = unsafe { (vtable.get)(impl_ptr as *const _) };
    assert!(!callee_ptr.is_null());
}

#[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
#[test]
fn simulated_cpp_scalar_deleting_destructor() {
    static DROPS: AtomicUsize = AtomicUsize::new(0);

    let f = Function::<fn() -> i32>::new({
        let held = Tracked(&DROPS);
        move || {
            let _ = &held;
            7
        }
    });
    let impl_ptr = raw_impl_ptr(&f);
    let vtable = unsafe { *(impl_ptr as *const *const RawFuncBaseVmt<i32>) };
    let vtable = unsafe { &*vtable };

    leak(f);
    let returned = unsafe { (vtable.scalar_deleting_destructor)(impl_ptr, 1) };
    assert!(std::ptr::eq(returned, impl_ptr), "must return `this`");
    assert_eq!(DROPS.load(Ordering::Relaxed), 1);
}

#[test]
fn simulated_game_local_impl_call_and_drop() {
    static DEALLOC_FLAG: AtomicUsize = AtomicUsize::new(2);

    #[repr(C)]
    struct LocalImpl {
        vtable: *const LocalVmt,
        value: i32,
    }
    #[repr(C)]
    struct LocalVmt {
        copy: unsafe extern "C" fn(*const LocalImpl, *mut std::ffi::c_void) -> *mut LocalImpl,
        move_: unsafe extern "C" fn(*mut LocalImpl, *mut std::ffi::c_void) -> *mut LocalImpl,
        do_call: unsafe extern "C" fn(*mut LocalImpl) -> i32,
        target_type: unsafe extern "C" fn(*const LocalImpl) -> *const std::ffi::c_void,
        delete_this: unsafe extern "C" fn(*mut LocalImpl, bool),
        #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
        scalar_deleting_destructor: unsafe extern "C" fn(*mut LocalImpl, u32) -> *mut LocalImpl,
        get: unsafe extern "C" fn(*const LocalImpl) -> *const std::ffi::c_void,
    }
    unsafe extern "C" fn local_do_call(this: *mut LocalImpl) -> i32 {
        unsafe { (*this).value }
    }
    unsafe extern "C" fn local_delete_this(_this: *mut LocalImpl, dealloc: bool) {
        DEALLOC_FLAG.store(dealloc as usize, Ordering::Relaxed);
    }
    unsafe extern "C" fn local_unreachable_copy(
        _this: *const LocalImpl,
        _where_: *mut std::ffi::c_void,
    ) -> *mut LocalImpl {
        unreachable!()
    }
    unsafe extern "C" fn local_unreachable_move(
        _this: *mut LocalImpl,
        _where_: *mut std::ffi::c_void,
    ) -> *mut LocalImpl {
        unreachable!()
    }
    unsafe extern "C" fn local_unreachable_target_type(
        _this: *const LocalImpl,
    ) -> *const std::ffi::c_void {
        unreachable!()
    }
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    unsafe extern "C" fn local_unreachable_scalar_deleting_destructor(
        _this: *mut LocalImpl,
        _flags: u32,
    ) -> *mut LocalImpl {
        unreachable!()
    }
    unsafe extern "C" fn local_unreachable_get(_this: *const LocalImpl) -> *const std::ffi::c_void {
        unreachable!()
    }

    static LOCAL_VMT: LocalVmt = LocalVmt {
        copy: local_unreachable_copy,
        move_: local_unreachable_move,
        do_call: local_do_call,
        target_type: local_unreachable_target_type,
        delete_this: local_delete_this,
        #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
        scalar_deleting_destructor: local_unreachable_scalar_deleting_destructor,
        get: local_unreachable_get,
    };

    {
        let mut f = Function::<fn() -> i32>::empty();
        let storage_addr = &mut f as *mut _ as *mut u8;

        let f: &mut Function<fn() -> i32> = unsafe {
            let impl_ptr = storage_addr as *mut LocalImpl;
            std::ptr::write(
                impl_ptr,
                LocalImpl {
                    vtable: &LOCAL_VMT,
                    value: 1234,
                },
            );
            #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
            let impl_ptr_slot = (storage_addr as *mut *mut std::ffi::c_void).add(7);
            #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
            let impl_ptr_slot = (storage_addr as *mut *mut std::ffi::c_void).add(3);
            std::ptr::write(impl_ptr_slot, storage_addr as *mut std::ffi::c_void);
            &mut *(storage_addr as *mut Function<fn() -> i32>)
        };

        assert_eq!(f.call(()), 1234);
    }
    assert_eq!(
        DEALLOC_FLAG.load(Ordering::Relaxed),
        0,
        "dropping a local impl must call _Delete_this(false)"
    );
}
