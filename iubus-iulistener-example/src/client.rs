/********************************************************************************
 * Copyright (c) 2024 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

use binder::{binder_impl::Binder, BinderFeatures, Interface, Strong};

use aidl_rust_codegen::binder_impls::IUBus::IUBus;
use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::parcelable_stubs::ParcelableUMessage;

use up_rust::uprotocol::UEntity;

pub struct MyIUListener;

impl Interface for MyIUListener {}

impl IUListener for MyIUListener {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        println!("received ParcelableUMessage: {:?}", event);
        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let test_calling_client_iulistener_service: Strong<dyn IUBus> =
        binder::get_interface("test-calling-client-iulistener-service").unwrap();

    let my_package_name = "super_cool_client";
    let uentity = UEntity {
        name: "ustreamer_glue".to_string(),
        version_major: Some(1),
        ..Default::default()
    };

    println!(
        "my_entity we're sending over with registerClient():\n{:?}",
        uentity
    );

    let my_flags: i32 = 0;

    let my_iulistener = MyIUListener;
    let my_iulistener_binder = BnUListener::new_binder(my_iulistener, BinderFeatures::default());
    println!("Running!");

    let client_token = Binder::new(()).as_binder();

    println!("Call registerClient()");
    let _parcelable_ustatus = test_calling_client_iulistener_service.registerClient(
        my_package_name,
        &uentity.into(),
        &client_token,
        my_flags,
        &my_iulistener_binder,
    );
    binder::ProcessState::join_thread_pool();

    anyhow::Ok(())
}
