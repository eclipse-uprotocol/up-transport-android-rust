use binder::{BinderFeatures, Interface, Strong,
    binder_impl::Binder
};
use binder::unstable_api::new_spibinder;
use aidl_rust_codegen::binder_impls::IUListener::{IUListener, BnUListener};
use aidl_rust_codegen::binder_impls::IUBus::{IUBus};
use aidl_rust_codegen::parcelable_stubs::{ParcelableUMessage, ParcelableUEntity};

use up_rust::uprotocol::{UAttributes, UAuthority, UEntity, UMessage, UPayload, UResource, UUri};

pub struct MyIUListener;

impl Interface for MyIUListener {}

impl IUListener for MyIUListener {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        println!("received ParcelableUMessage: {:?}", event);
        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_calling_client_iulistener_service = binder::get_interface("adb");
//     let test_calling_client_iulistener_service = binder::get_interface("org.eclipse.uprotocol.ubus.MyIUBusService");

    if test_calling_client_iulistener_service.is_err() {
        println!("get_interface failed: {:?}", test_calling_client_iulistener_service);
        return anyhow::Ok(());
    }

    let test_calling_client_iulistener_service: Strong<dyn IUBus> = test_calling_client_iulistener_service.unwrap();

//     let test_calling_client_iulistener_service: Strong<dyn IUBus> = binder::get_interface("org.eclipse.uprotocol.action.BIND_MY_IUBUS_SERVICE").unwrap();

    let my_package_name = "super_cool_client";
    let my_entity = UEntity {
        name: "super_cool_name".to_owned(),
        ..Default::default()
    };

    println!("my_entity we're sending over with registerClient():\n{:?}", my_entity);

    let my_flags: i32 = 0;

    let my_iulistener = MyIUListener;
    let my_iulistener_binder = BnUListener::new_binder(my_iulistener, BinderFeatures::default());
    println!("Running!");

    let client_token = Binder::new(()).as_binder();

    println!("Call registerClient()");
    let parcelable_ustatus = test_calling_client_iulistener_service.registerClient(&my_package_name, &my_entity.into(), &client_token, my_flags, &my_iulistener_binder);
    binder::ProcessState::join_thread_pool();

    anyhow::Ok(())
}
