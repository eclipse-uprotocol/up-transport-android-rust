use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;
use binder::BinderFeatures;
use binder::Interface;

pub struct TestIUListenerService;

impl Interface for TestIUListenerService {}

impl IUListener for TestIUListenerService {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        println!("received ParcelableUMessage: {:?}", event);

        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_iulistener_service = TestIUListenerService;
    let test_iulistener_service_binder =
        BnUListener::new_binder(test_iulistener_service, BinderFeatures::default());
    binder::add_service(
        "test-iulistener-service",
        test_iulistener_service_binder.as_binder(),
    )
    .expect("Failed to register service?");
    println!("Running!");
    binder::ProcessState::join_thread_pool();
    anyhow::Ok(())
}
