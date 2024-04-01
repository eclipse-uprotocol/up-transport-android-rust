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

use parcelable_stubs::ParcelableUMessage;

/// Implementation of the `IUListener` AIDL interface.
use org_eclipse_uprotocol_core_ubus_iulistener::aidl::org::eclipse::uprotocol::core::ubus::IUListener::IUListener;
use org_eclipse_uprotocol_core_ubus_iulistener::binder;

/// The `MyStubIUListener` implementation.
pub struct MyStubIUListener;

impl binder::Interface for MyStubIUListener {}

impl IUListener for MyStubIUListener {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        Ok(())
    }
}
