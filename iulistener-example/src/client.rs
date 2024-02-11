use binder::Strong;
use aidl_rust_codegen::binder_impls::IUListener::{IUListener};
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;

pub fn run() -> anyhow::Result<()> {
    let test_iulistener_service: Strong<dyn IUListener> = binder::get_interface("test-iulistener-service").unwrap();
    println!("Do onReceive()");
    let parcelable_umessage = ParcelableUMessage::default();
    let res = test_iulistener_service.onReceive(&parcelable_umessage).expect("Failed to trigger onReceive");
    println!("Got result: {:?}", res);
    println!("Done!");
    Ok(())
}
