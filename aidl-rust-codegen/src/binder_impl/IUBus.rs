/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org.eclipse.uprotocol.core.ubus.iubus-rust-source/gen/org/eclipse/uprotocol/core/ubus/IUBus.rs.d -o out/soong/.intermediates/external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org.eclipse.uprotocol.core.ubus.iubus-rust-source/gen -Nexternal/rust/up-client-android-rust/aidl-rust-codegen/src/aidl external/rust/up-client-android-rust/aidl-rust-codegen/src/aidl/org/eclipse/uprotocol/core/ubus/IUBus.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
use binder::StatusCode;

use crate::parcelable_stubs;
use protobuf::Message;

declare_binder_interface! {
  IUBus["org.eclipse.uprotocol.core.ubus.IUBus"] {
    native: BnUBus(on_transact),
    proxy: BpUBus {
    },
    async: IUBusAsync,
  }
}
pub trait IUBus: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUBus" }
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
  fn getDefaultImpl() -> IUBusDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IUBusDefaultRef) -> IUBusDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IUBusAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUBus" }
  fn r#registerClient<'a>(&'a self, _arg_packageName: &'a str, _arg_entity: &'a parcelable_stubs::ParcelableUEntity, _arg_clientToken: &'a binder::SpIBinder, _arg_flags: i32, _arg_listener: &'a binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>>;
}
#[::async_trait::async_trait]
pub trait IUBusAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "org.eclipse.uprotocol.core.ubus.IUBus" }
  async fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus>;
}
impl BnUBus {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IUBus>
  where
    T: IUBusAsyncServer + binder::Interface + Send + Sync + 'static,
    R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
  {
    struct Wrapper<T, R> {
      _inner: T,
      _rt: R,
    }
    impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
      fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
      fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
    }
    impl<T, R> IUBus for Wrapper<T, R>
    where
      T: IUBusAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
        self._rt.block_on(self._inner.r#registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IUBusDefault: Send + Sync {
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#registerClient: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
pub type IUBusDefaultRef = Option<std::sync::Arc<dyn IUBusDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IUBusDefaultRef> = std::sync::Mutex::new(None);
impl BpUBus {
  fn build_parcel_registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_packageName)?;
    // Pack ParcelableUEntity using Protobuf - Start
    let uentity = _arg_entity.as_ref();
    let bytes = uentity.write_to_bytes().map_err(|_e| { StatusCode::BAD_VALUE })?;
    aidl_data.write(&(bytes.len() as i32))?;
    aidl_data.write(&bytes)?;
    // Pack ParcelableUEntity using Protobuf - End
    aidl_data.write(_arg_clientToken)?;
    aidl_data.write(&_arg_flags)?;
    aidl_data.write(_arg_listener)?;
    Ok(aidl_data)
  }
  fn read_response_registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IUBus>::getDefaultImpl() {
        return _aidl_default_impl.r#registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: parcelable_stubs::ParcelableUStatus = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IUBus for BpUBus {
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> {
    let _aidl_data = self.build_parcel_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#registerClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IUBusAsync<P> for BpUBus {
  fn r#registerClient<'a>(&'a self, _arg_packageName: &'a str, _arg_entity: &'a parcelable_stubs::ParcelableUEntity, _arg_clientToken: &'a binder::SpIBinder, _arg_flags: i32, _arg_listener: &'a binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::BoxFuture<'a, binder::Result<parcelable_stubs::ParcelableUStatus>> {
    let _aidl_data = match self.build_parcel_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#registerClient, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener, _aidl_reply)
      }
    )
  }
}
impl IUBus for binder::binder_impl::Binder<BnUBus> {
  fn r#registerClient(&self, _arg_packageName: &str, _arg_entity: &parcelable_stubs::ParcelableUEntity, _arg_clientToken: &binder::SpIBinder, _arg_flags: i32, _arg_listener: &binder::Strong<dyn crate::binder_impls::IUListener::IUListener>) -> binder::Result<parcelable_stubs::ParcelableUStatus> { self.0.r#registerClient(_arg_packageName, _arg_entity, _arg_clientToken, _arg_flags, _arg_listener) }
}
fn on_transact(_aidl_service: &dyn IUBus, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#registerClient => {
      let _arg_packageName: String = _aidl_data.read()?;
      // Unpack ParcelableUEntity using Protobuf - Start
      let _size = _aidl_data.read::<i32>()?;
      let bytes = _aidl_data.read::<Vec<u8>>()?;
      let uentity = up_rust::uprotocol::UEntity::parse_from_bytes(&bytes).map_err(|_e| { StatusCode::BAD_VALUE })?;
      let _arg_entity = parcelable_stubs::ParcelableUEntity::from(uentity);
      // Unpack ParcelableUEntity using Protobuf - End
      let _arg_clientToken: binder::SpIBinder = _aidl_data.read()?;
      let _arg_flags: i32 = _aidl_data.read()?;
      let _arg_listener: binder::Strong<dyn crate::binder_impls::IUListener::IUListener> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#registerClient(&_arg_packageName, &_arg_entity, &_arg_clientToken, _arg_flags, &_arg_listener);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IUBus as _3_org_7_eclipse_9_uprotocol_4_core_4_ubus_5_IUBus;
}
