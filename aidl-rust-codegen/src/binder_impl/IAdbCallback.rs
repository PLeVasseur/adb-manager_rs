/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen/android/debug/IAdbCallback.rs.d -o out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen -Nexternal/rust/adb-manager_rs/aidl-rust-codegen/src/aidl external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android/debug/IAdbCallback.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IAdbCallback["android.debug.IAdbCallback"] {
    native: BnAdbCallback(on_transact),
    proxy: BpAdbCallback {
    },
    async: IAdbCallbackAsync,
  }
}
pub trait IAdbCallback: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.debug.IAdbCallback" }
  fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<()>;
  fn getDefaultImpl() -> IAdbCallbackDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IAdbCallbackDefaultRef) -> IAdbCallbackDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IAdbCallbackAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.debug.IAdbCallback" }
  fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> std::future::Ready<binder::Result<()>>;
}
#[::async_trait::async_trait]
pub trait IAdbCallbackAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.debug.IAdbCallback" }
  async fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<()>;
}
impl BnAdbCallback {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IAdbCallback>
  where
    T: IAdbCallbackAsyncServer + binder::Interface + Send + Sync + 'static,
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
    impl<T, R> IAdbCallback for Wrapper<T, R>
    where
      T: IAdbCallbackAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#onDebuggingChanged(_arg_enabled, _arg_type))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IAdbCallbackDefault: Send + Sync {
  fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#onDebuggingChanged: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
pub type IAdbCallbackDefaultRef = Option<std::sync::Arc<dyn IAdbCallbackDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IAdbCallbackDefaultRef> = std::sync::Mutex::new(None);
impl BpAdbCallback {
  fn build_parcel_onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_enabled)?;
    aidl_data.write(&_arg_type)?;
    Ok(aidl_data)
  }
  fn read_response_onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbCallback>::getDefaultImpl() {
        return _aidl_default_impl.r#onDebuggingChanged(_arg_enabled, _arg_type);
      }
    }
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
}
impl IAdbCallback for BpAdbCallback {
  fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_onDebuggingChanged(_arg_enabled, _arg_type)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#onDebuggingChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_onDebuggingChanged(_arg_enabled, _arg_type, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IAdbCallbackAsync<P> for BpAdbCallback {
  fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> std::future::Ready<binder::Result<()>> {
    let _aidl_data = match self.build_parcel_onDebuggingChanged(_arg_enabled, _arg_type) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return std::future::ready(Err(err)),
    };
    let _aidl_reply = self.binder.submit_transact(transactions::r#onDebuggingChanged, _aidl_data, binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL);
    std::future::ready(self.read_response_onDebuggingChanged(_arg_enabled, _arg_type, _aidl_reply))
  }
}
impl IAdbCallback for binder::binder_impl::Binder<BnAdbCallback> {
  fn r#onDebuggingChanged(&self, _arg_enabled: bool, _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType) -> binder::Result<()> { self.0.r#onDebuggingChanged(_arg_enabled, _arg_type) }
}
fn on_transact(_aidl_service: &dyn IAdbCallback, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#onDebuggingChanged => {
      let _arg_enabled: bool = _aidl_data.read()?;
      let _arg_type: crate::binder_impls::AdbTransportType::r#AdbTransportType = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#onDebuggingChanged(_arg_enabled, _arg_type);
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IAdbCallback as _7_android_5_debug_12_IAdbCallback;
}
