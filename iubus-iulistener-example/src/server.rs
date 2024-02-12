use binder::{BinderFeatures, SpIBinder, Strong};
use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::binder_impls::IUBus::{BnUBus, IUBus};
use aidl_rust_codegen::parcelable_stubs::{ParcelableUMessage, ParcelableUEntity, ParcelableUStatus};
use binder::{Interface, Result as BinderResult,
             binder_impl::{BorrowedParcel, UnstructuredParcelable},
};

pub struct TestCallingClientIUListenerService;

impl Interface for TestCallingClientIUListenerService {}

impl IUBus for TestCallingClientIUListenerService {
    fn registerClient(&self, packageName: &str, entity: &ParcelableUEntity, clientToken: &SpIBinder, flags: i32, listener: &Strong<(dyn IUListener + 'static)>) -> binder::Result<ParcelableUStatus> {
        Ok(ParcelableUStatus::default())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_calling_client_iulistener_service = TestCallingClientIUListenerService;
    let test_calling_client_iulistener_service_binder = BnUBus::new_binder(test_calling_client_iulistener_service, BinderFeatures::default());
    binder::add_service("test-calling-client-iulistener-service", test_calling_client_iulistener_service_binder.as_binder())
        .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
