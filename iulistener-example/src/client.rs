use binder::Strong;
use aidl_rust_codegen::binder_impls::IUListener::{IUListener};
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;

use up_rust::uprotocol::{UAttributes, UAuthority, UEntity, UMessage, UPayload, UResource, UUri};

pub fn run() -> anyhow::Result<()> {
    let test_iulistener_service: Strong<dyn IUListener> = binder::get_interface("test-iulistener-service").unwrap();
    println!("Do onReceive()");
    let umessage = UMessage {
        attributes: Some(UAttributes {
            source: Some(UUri {
                authority: Some(UAuthority {
                    name: Some("my_vin".to_owned()),
                    ..Default::default()
                }).into(),
                entity: Some(UEntity {
                    name: "door".to_owned(),
                    ..Default::default()
                }).into(),
                resource: Some(UResource {
                    name: "front_left".to_owned(),
                    ..Default::default()
                }).into(),
                ..Default::default()
            }).into(),
            ..Default::default()
        }).into(),
        ..Default::default()
    };
    let parcelable_umessage = ParcelableUMessage::from(umessage);
    println!("parcelable_umessage prior to onReceive: {:?}", parcelable_umessage);
    let res = test_iulistener_service.onReceive(&parcelable_umessage).expect("Failed to trigger onReceive");
    println!("Got result: {:?}", res);
    println!("Done!");
    Ok(())
}
