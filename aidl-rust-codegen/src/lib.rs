#[macro_use]
extern crate log;

mod aidl;
mod binder_impl;

pub mod binder_impls {
    pub use crate::binder_impl::{IUBus, IUListener};
}
pub mod parcelable_stubs {
    pub use crate::aidl::parcelable_stubs::*;
}
