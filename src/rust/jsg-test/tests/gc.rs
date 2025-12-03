//! GC (garbage collection) tests for Rust resources.
//!
//! These tests verify that Rust resources are properly cleaned up when:
//! 1. All Rust `Ref` handles are dropped and no JavaScript wrapper exists
//! 2. All Rust `Ref` handles are dropped and V8 garbage collects the JS wrapper

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use jsg::Lock;
use jsg::Resource;
use jsg::Type;
use jsg_macros::jsg_method;
use jsg_macros::jsg_resource;

use crate::ffi;

/// Counter to track how many `SimpleResource` instances have been dropped.
static SIMPLE_RESOURCE_DROPS: AtomicUsize = AtomicUsize::new(0);

#[jsg_resource]
struct SimpleResource {
    pub name: String,
}

impl Drop for SimpleResource {
    fn drop(&mut self) {
        SIMPLE_RESOURCE_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

#[jsg_resource]
#[expect(clippy::unnecessary_wraps)]
impl SimpleResource {
    #[jsg_method]
    fn get_name(&self) -> Result<String, String> {
        Ok(self.name.clone())
    }
}

/// Tests that resources are dropped when all Rust Refs are dropped and no JS wrapper exists.
///
/// When a resource is allocated but never wrapped for JavaScript, dropping all `Ref` handles
/// should immediately deallocate the resource.
#[test]
fn supports_gc_via_realm_drop() {
    SIMPLE_RESOURCE_DROPS.store(0, Ordering::SeqCst);

    let harness = crate::Harness::new();
    harness.run_in_context(|isolate, _ctx| unsafe {
        let mut lock = Lock::from_isolate_ptr(isolate);
        let resource = SimpleResource::alloc(
            &mut lock,
            SimpleResource {
                name: "test".to_owned(),
            },
        );
        assert_eq!(SIMPLE_RESOURCE_DROPS.load(Ordering::SeqCst), 0);
        std::mem::drop(resource);
        assert_eq!(SIMPLE_RESOURCE_DROPS.load(Ordering::SeqCst), 1);
    });
}

/// Tests that resources are dropped via V8 GC weak callback when JS wrapper is collected.
///
/// When a resource is wrapped for JavaScript:
/// 1. Dropping all Rust `Ref` handles makes the V8 Global weak
/// 2. V8 GC can then collect the wrapper and trigger the weak callback
/// 3. The weak callback deallocates the resource
#[test]
fn supports_gc_via_weak_callback() {
    SIMPLE_RESOURCE_DROPS.store(0, Ordering::SeqCst);

    let harness = crate::Harness::new();
    harness.run_in_context(|isolate, _ctx| unsafe {
        let mut lock = Lock::from_isolate_ptr(isolate);
        let resource = SimpleResource::alloc(
            &mut lock,
            SimpleResource {
                name: "test".to_owned(),
            },
        );
        let _wrapped = SimpleResource::wrap(resource.clone(), &mut lock);
        assert_eq!(SIMPLE_RESOURCE_DROPS.load(Ordering::SeqCst), 0);
        std::mem::drop(resource);
        // There is a JS object that holds a reference to the resource
        assert_eq!(SIMPLE_RESOURCE_DROPS.load(Ordering::SeqCst), 0);
    });

    harness.run_in_context(|isolate, _ctx| unsafe {
        assert_eq!(SIMPLE_RESOURCE_DROPS.load(Ordering::SeqCst), 0);
        ffi::request_gc(isolate);
        assert_eq!(SIMPLE_RESOURCE_DROPS.load(Ordering::SeqCst), 1);
    });
}
