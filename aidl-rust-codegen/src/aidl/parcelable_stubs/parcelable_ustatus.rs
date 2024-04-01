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

use binder::{
    binder_impl::{BorrowedParcel, UnstructuredParcelable},
    impl_deserialize_for_unstructured_parcelable, impl_serialize_for_unstructured_parcelable,
    StatusCode,
};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "use_proto")]
use protobuf::Message;

#[cfg(feature = "use_proto")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParcelableUStatus(up_rust::uprotocol::UStatus);

#[cfg(feature = "use_proto")]
impl From<up_rust::uprotocol::UStatus> for ParcelableUStatus {
    fn from(item: up_rust::uprotocol::UStatus) -> Self {
        ParcelableUStatus(item)
    }
}

#[cfg(feature = "use_proto")]
impl Deref for ParcelableUStatus {
    type Target = up_rust::uprotocol::UStatus;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "use_proto")]
impl DerefMut for ParcelableUStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "use_proto")]
impl AsRef<up_rust::uprotocol::UStatus> for ParcelableUStatus {
    fn as_ref(&self) -> &up_rust::uprotocol::UStatus {
        &self.0
    }
}

#[cfg(not(feature = "use_proto"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParcelableUStatus;

/// Rust `UnstructuredParcelable` stub for `ParcelableUStatus`
/// True implementation is down below
/// This exists just to allow the Android aidl_interface build process to succeed
#[cfg(not(feature = "use_proto"))]
impl UnstructuredParcelable for ParcelableUStatus {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel) -> Result<(), StatusCode> {
        Ok(())
    }

    fn from_parcel(parcel: &BorrowedParcel) -> Result<Self, StatusCode> {
        Ok(Self {})
    }
}

/// Rust `UnstructuredParcelable` implementation for `ParcelableUStatus` using Protobuf serialization/deserialization
#[cfg(feature = "use_proto")]
impl UnstructuredParcelable for ParcelableUStatus {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel) -> Result<(), StatusCode> {
        let ustatus = &self.0;
        let bytes = ustatus
            .write_to_bytes()
            .map_err(|_e| StatusCode::BAD_VALUE)?;
        parcel.write(&(bytes.len() as i32))?;
        parcel.write(&bytes)?;
        Ok(())
    }

    fn from_parcel(parcel: &BorrowedParcel) -> Result<Self, StatusCode> {
        let _size = parcel.read::<i32>()?;
        let bytes = parcel.read::<Vec<u8>>()?;
        let ustatus = up_rust::uprotocol::UStatus::parse_from_bytes(&bytes)
            .map_err(|_e| StatusCode::BAD_VALUE)?;
        let parcelable_ustatus = ParcelableUStatus(ustatus);
        Ok(parcelable_ustatus)
    }
}

impl_deserialize_for_unstructured_parcelable!(ParcelableUStatus);
impl_serialize_for_unstructured_parcelable!(ParcelableUStatus);
