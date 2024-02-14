/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen/android/debug/IAdbManager.rs.d -o out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen -Nexternal/rust/adb-manager_rs/aidl-rust-codegen/src/aidl external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android/debug/IAdbManager.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IAdbManager["android.debug.IAdbManager"] {
    native: BnAdbManager(on_transact),
    proxy: BpAdbManager {
    },
    async: IAdbManagerAsync,
  }
}
pub trait IAdbManager: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.debug.IAdbManager" }
  fn r#allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<()>;
  fn r#denyDebugging(&self) -> binder::Result<()>;
  fn r#clearDebuggingKeys(&self) -> binder::Result<()>;
  fn r#allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<()>;
  fn r#denyWirelessDebugging(&self) -> binder::Result<()>;
  fn r#getPairedDevices(&self) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>>;
  fn r#unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<()>;
  fn r#enablePairingByPairingCode(&self) -> binder::Result<()>;
  fn r#enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<()>;
  fn r#getAdbWirelessPort(&self) -> binder::Result<i32>;
  fn r#disablePairing(&self) -> binder::Result<()>;
  fn r#isAdbWifiSupported(&self) -> binder::Result<bool>;
  fn r#isAdbWifiQrSupported(&self) -> binder::Result<bool>;
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()>;
  fn r#unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()>;
  fn getDefaultImpl() -> IAdbManagerDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IAdbManagerDefaultRef) -> IAdbManagerDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IAdbManagerAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.debug.IAdbManager" }
  fn r#allowDebugging<'a>(&'a self, _arg_alwaysAllow: bool, _arg_publicKey: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#denyDebugging<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#clearDebuggingKeys<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#allowWirelessDebugging<'a>(&'a self, _arg_alwaysAllow: bool, _arg_bssid: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#denyWirelessDebugging<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getPairedDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>>>;
  fn r#unpairDevice<'a>(&'a self, _arg_fingerprint: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#enablePairingByPairingCode<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#enablePairingByQrCode<'a>(&'a self, _arg_serviceName: &'a str, _arg_password: &'a str) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#getAdbWirelessPort<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>>;
  fn r#disablePairing<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#isAdbWifiSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#isAdbWifiQrSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>>;
  fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::BoxFuture<'a, binder::Result<()>>;
  fn r#unregisterCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::BoxFuture<'a, binder::Result<()>>;
}
#[::async_trait::async_trait]
pub trait IAdbManagerAsyncServer: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.debug.IAdbManager" }
  async fn r#allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<()>;
  async fn r#denyDebugging(&self) -> binder::Result<()>;
  async fn r#clearDebuggingKeys(&self) -> binder::Result<()>;
  async fn r#allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<()>;
  async fn r#denyWirelessDebugging(&self) -> binder::Result<()>;
  async fn r#getPairedDevices(&self) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>>;
  async fn r#unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<()>;
  async fn r#enablePairingByPairingCode(&self) -> binder::Result<()>;
  async fn r#enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<()>;
  async fn r#getAdbWirelessPort(&self) -> binder::Result<i32>;
  async fn r#disablePairing(&self) -> binder::Result<()>;
  async fn r#isAdbWifiSupported(&self) -> binder::Result<bool>;
  async fn r#isAdbWifiQrSupported(&self) -> binder::Result<bool>;
  async fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()>;
  async fn r#unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()>;
}
impl BnAdbManager {
  /// Create a new async binder service.
  pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IAdbManager>
  where
    T: IAdbManagerAsyncServer + binder::Interface + Send + Sync + 'static,
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
    impl<T, R> IAdbManager for Wrapper<T, R>
    where
      T: IAdbManagerAsyncServer + Send + Sync + 'static,
      R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
    {
      fn r#allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#allowDebugging(_arg_alwaysAllow, _arg_publicKey))
      }
      fn r#denyDebugging(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#denyDebugging())
      }
      fn r#clearDebuggingKeys(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#clearDebuggingKeys())
      }
      fn r#allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid))
      }
      fn r#denyWirelessDebugging(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#denyWirelessDebugging())
      }
      fn r#getPairedDevices(&self) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>> {
        self._rt.block_on(self._inner.r#getPairedDevices())
      }
      fn r#unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#unpairDevice(_arg_fingerprint))
      }
      fn r#enablePairingByPairingCode(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#enablePairingByPairingCode())
      }
      fn r#enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#enablePairingByQrCode(_arg_serviceName, _arg_password))
      }
      fn r#getAdbWirelessPort(&self) -> binder::Result<i32> {
        self._rt.block_on(self._inner.r#getAdbWirelessPort())
      }
      fn r#disablePairing(&self) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#disablePairing())
      }
      fn r#isAdbWifiSupported(&self) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#isAdbWifiSupported())
      }
      fn r#isAdbWifiQrSupported(&self) -> binder::Result<bool> {
        self._rt.block_on(self._inner.r#isAdbWifiQrSupported())
      }
      fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#registerCallback(_arg_callback))
      }
      fn r#unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> {
        self._rt.block_on(self._inner.r#unregisterCallback(_arg_callback))
      }
    }
    let wrapped = Wrapper { _inner: inner, _rt: rt };
    Self::new_binder(wrapped, features)
  }
}
pub trait IAdbManagerDefault: Send + Sync {
  fn r#allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#denyDebugging(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#clearDebuggingKeys(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#denyWirelessDebugging(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getPairedDevices(&self) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enablePairingByPairingCode(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#getAdbWirelessPort(&self) -> binder::Result<i32> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#disablePairing(&self) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#isAdbWifiSupported(&self) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#isAdbWifiQrSupported(&self) -> binder::Result<bool> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn r#unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const r#allowDebugging: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#denyDebugging: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#clearDebuggingKeys: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#allowWirelessDebugging: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#denyWirelessDebugging: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#getPairedDevices: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
  pub const r#unpairDevice: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
  pub const r#enablePairingByPairingCode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
  pub const r#enablePairingByQrCode: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
  pub const r#getAdbWirelessPort: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
  pub const r#disablePairing: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
  pub const r#isAdbWifiSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
  pub const r#isAdbWifiQrSupported: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
  pub const r#registerCallback: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
  pub const r#unregisterCallback: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
}
pub type IAdbManagerDefaultRef = Option<std::sync::Arc<dyn IAdbManagerDefault>>;
static DEFAULT_IMPL: std::sync::Mutex<IAdbManagerDefaultRef> = std::sync::Mutex::new(None);
impl BpAdbManager {
  fn build_parcel_allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_alwaysAllow)?;
    aidl_data.write(_arg_publicKey)?;
    Ok(aidl_data)
  }
  fn read_response_allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#allowDebugging(_arg_alwaysAllow, _arg_publicKey);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_denyDebugging(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_denyDebugging(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#denyDebugging();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_clearDebuggingKeys(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_clearDebuggingKeys(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#clearDebuggingKeys();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_alwaysAllow)?;
    aidl_data.write(_arg_bssid)?;
    Ok(aidl_data)
  }
  fn read_response_allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_denyWirelessDebugging(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_denyWirelessDebugging(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#denyWirelessDebugging();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_getPairedDevices(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getPairedDevices(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#getPairedDevices();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_fingerprint)?;
    Ok(aidl_data)
  }
  fn read_response_unpairDevice(&self, _arg_fingerprint: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#unpairDevice(_arg_fingerprint);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_enablePairingByPairingCode(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_enablePairingByPairingCode(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#enablePairingByPairingCode();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_serviceName)?;
    aidl_data.write(_arg_password)?;
    Ok(aidl_data)
  }
  fn read_response_enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#enablePairingByQrCode(_arg_serviceName, _arg_password);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_getAdbWirelessPort(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getAdbWirelessPort(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#getAdbWirelessPort();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: i32 = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_disablePairing(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_disablePairing(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#disablePairing();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_isAdbWifiSupported(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_isAdbWifiSupported(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#isAdbWifiSupported();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_isAdbWifiQrSupported(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_isAdbWifiQrSupported(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<bool> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#isAdbWifiQrSupported();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: bool = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_callback)?;
    Ok(aidl_data)
  }
  fn read_response_registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#registerCallback(_arg_callback);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
  fn build_parcel_unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_callback)?;
    Ok(aidl_data)
  }
  fn read_response_unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IAdbManager>::getDefaultImpl() {
        return _aidl_default_impl.r#unregisterCallback(_arg_callback);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    Ok(())
  }
}
impl IAdbManager for BpAdbManager {
  fn r#allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_allowDebugging(_arg_alwaysAllow, _arg_publicKey)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#allowDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_allowDebugging(_arg_alwaysAllow, _arg_publicKey, _aidl_reply)
  }
  fn r#denyDebugging(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_denyDebugging()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#denyDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_denyDebugging(_aidl_reply)
  }
  fn r#clearDebuggingKeys(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_clearDebuggingKeys()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#clearDebuggingKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_clearDebuggingKeys(_aidl_reply)
  }
  fn r#allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#allowWirelessDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid, _aidl_reply)
  }
  fn r#denyWirelessDebugging(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_denyWirelessDebugging()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#denyWirelessDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_denyWirelessDebugging(_aidl_reply)
  }
  fn r#getPairedDevices(&self) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>> {
    let _aidl_data = self.build_parcel_getPairedDevices()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getPairedDevices, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getPairedDevices(_aidl_reply)
  }
  fn r#unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_unpairDevice(_arg_fingerprint)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#unpairDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_unpairDevice(_arg_fingerprint, _aidl_reply)
  }
  fn r#enablePairingByPairingCode(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_enablePairingByPairingCode()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enablePairingByPairingCode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enablePairingByPairingCode(_aidl_reply)
  }
  fn r#enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_enablePairingByQrCode(_arg_serviceName, _arg_password)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#enablePairingByQrCode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_enablePairingByQrCode(_arg_serviceName, _arg_password, _aidl_reply)
  }
  fn r#getAdbWirelessPort(&self) -> binder::Result<i32> {
    let _aidl_data = self.build_parcel_getAdbWirelessPort()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getAdbWirelessPort, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getAdbWirelessPort(_aidl_reply)
  }
  fn r#disablePairing(&self) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_disablePairing()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#disablePairing, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_disablePairing(_aidl_reply)
  }
  fn r#isAdbWifiSupported(&self) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_isAdbWifiSupported()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#isAdbWifiSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_isAdbWifiSupported(_aidl_reply)
  }
  fn r#isAdbWifiQrSupported(&self) -> binder::Result<bool> {
    let _aidl_data = self.build_parcel_isAdbWifiQrSupported()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#isAdbWifiQrSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_isAdbWifiQrSupported(_aidl_reply)
  }
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_registerCallback(_arg_callback)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_registerCallback(_arg_callback, _aidl_reply)
  }
  fn r#unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_unregisterCallback(_arg_callback)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#unregisterCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_unregisterCallback(_arg_callback, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IAdbManagerAsync<P> for BpAdbManager {
  fn r#allowDebugging<'a>(&'a self, _arg_alwaysAllow: bool, _arg_publicKey: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_allowDebugging(_arg_alwaysAllow, _arg_publicKey) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#allowDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_allowDebugging(_arg_alwaysAllow, _arg_publicKey, _aidl_reply)
      }
    )
  }
  fn r#denyDebugging<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_denyDebugging() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#denyDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_denyDebugging(_aidl_reply)
      }
    )
  }
  fn r#clearDebuggingKeys<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_clearDebuggingKeys() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#clearDebuggingKeys, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_clearDebuggingKeys(_aidl_reply)
      }
    )
  }
  fn r#allowWirelessDebugging<'a>(&'a self, _arg_alwaysAllow: bool, _arg_bssid: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#allowWirelessDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid, _aidl_reply)
      }
    )
  }
  fn r#denyWirelessDebugging<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_denyWirelessDebugging() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#denyWirelessDebugging, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_denyWirelessDebugging(_aidl_reply)
      }
    )
  }
  fn r#getPairedDevices<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>>> {
    let _aidl_data = match self.build_parcel_getPairedDevices() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getPairedDevices, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getPairedDevices(_aidl_reply)
      }
    )
  }
  fn r#unpairDevice<'a>(&'a self, _arg_fingerprint: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_unpairDevice(_arg_fingerprint) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#unpairDevice, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_unpairDevice(_arg_fingerprint, _aidl_reply)
      }
    )
  }
  fn r#enablePairingByPairingCode<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_enablePairingByPairingCode() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enablePairingByPairingCode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enablePairingByPairingCode(_aidl_reply)
      }
    )
  }
  fn r#enablePairingByQrCode<'a>(&'a self, _arg_serviceName: &'a str, _arg_password: &'a str) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_enablePairingByQrCode(_arg_serviceName, _arg_password) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#enablePairingByQrCode, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_enablePairingByQrCode(_arg_serviceName, _arg_password, _aidl_reply)
      }
    )
  }
  fn r#getAdbWirelessPort<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
    let _aidl_data = match self.build_parcel_getAdbWirelessPort() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#getAdbWirelessPort, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getAdbWirelessPort(_aidl_reply)
      }
    )
  }
  fn r#disablePairing<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_disablePairing() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#disablePairing, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_disablePairing(_aidl_reply)
      }
    )
  }
  fn r#isAdbWifiSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_isAdbWifiSupported() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#isAdbWifiSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_isAdbWifiSupported(_aidl_reply)
      }
    )
  }
  fn r#isAdbWifiQrSupported<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<bool>> {
    let _aidl_data = match self.build_parcel_isAdbWifiQrSupported() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#isAdbWifiQrSupported, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_isAdbWifiQrSupported(_aidl_reply)
      }
    )
  }
  fn r#registerCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_registerCallback(_arg_callback) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#registerCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_registerCallback(_arg_callback, _aidl_reply)
      }
    )
  }
  fn r#unregisterCallback<'a>(&'a self, _arg_callback: &'a binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::BoxFuture<'a, binder::Result<()>> {
    let _aidl_data = match self.build_parcel_unregisterCallback(_arg_callback) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::r#unregisterCallback, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_unregisterCallback(_arg_callback, _aidl_reply)
      }
    )
  }
}
impl IAdbManager for binder::binder_impl::Binder<BnAdbManager> {
  fn r#allowDebugging(&self, _arg_alwaysAllow: bool, _arg_publicKey: &str) -> binder::Result<()> { self.0.r#allowDebugging(_arg_alwaysAllow, _arg_publicKey) }
  fn r#denyDebugging(&self) -> binder::Result<()> { self.0.r#denyDebugging() }
  fn r#clearDebuggingKeys(&self) -> binder::Result<()> { self.0.r#clearDebuggingKeys() }
  fn r#allowWirelessDebugging(&self, _arg_alwaysAllow: bool, _arg_bssid: &str) -> binder::Result<()> { self.0.r#allowWirelessDebugging(_arg_alwaysAllow, _arg_bssid) }
  fn r#denyWirelessDebugging(&self) -> binder::Result<()> { self.0.r#denyWirelessDebugging() }
  fn r#getPairedDevices(&self) -> binder::Result<Vec<crate::binder_impls::FingerprintAndPairDevice::r#FingerprintAndPairDevice>> { self.0.r#getPairedDevices() }
  fn r#unpairDevice(&self, _arg_fingerprint: &str) -> binder::Result<()> { self.0.r#unpairDevice(_arg_fingerprint) }
  fn r#enablePairingByPairingCode(&self) -> binder::Result<()> { self.0.r#enablePairingByPairingCode() }
  fn r#enablePairingByQrCode(&self, _arg_serviceName: &str, _arg_password: &str) -> binder::Result<()> { self.0.r#enablePairingByQrCode(_arg_serviceName, _arg_password) }
  fn r#getAdbWirelessPort(&self) -> binder::Result<i32> { self.0.r#getAdbWirelessPort() }
  fn r#disablePairing(&self) -> binder::Result<()> { self.0.r#disablePairing() }
  fn r#isAdbWifiSupported(&self) -> binder::Result<bool> { self.0.r#isAdbWifiSupported() }
  fn r#isAdbWifiQrSupported(&self) -> binder::Result<bool> { self.0.r#isAdbWifiQrSupported() }
  fn r#registerCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> { self.0.r#registerCallback(_arg_callback) }
  fn r#unregisterCallback(&self, _arg_callback: &binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback>) -> binder::Result<()> { self.0.r#unregisterCallback(_arg_callback) }
}
fn on_transact(_aidl_service: &dyn IAdbManager, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#allowDebugging => {
      let _arg_alwaysAllow: bool = _aidl_data.read()?;
      let _arg_publicKey: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#allowDebugging(_arg_alwaysAllow, &_arg_publicKey);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#denyDebugging => {
      let _aidl_return = _aidl_service.r#denyDebugging();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#clearDebuggingKeys => {
      let _aidl_return = _aidl_service.r#clearDebuggingKeys();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#allowWirelessDebugging => {
      let _arg_alwaysAllow: bool = _aidl_data.read()?;
      let _arg_bssid: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#allowWirelessDebugging(_arg_alwaysAllow, &_arg_bssid);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#denyWirelessDebugging => {
      let _aidl_return = _aidl_service.r#denyWirelessDebugging();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getPairedDevices => {
      let _aidl_return = _aidl_service.r#getPairedDevices();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#unpairDevice => {
      let _arg_fingerprint: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#unpairDevice(&_arg_fingerprint);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enablePairingByPairingCode => {
      let _aidl_return = _aidl_service.r#enablePairingByPairingCode();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#enablePairingByQrCode => {
      let _arg_serviceName: String = _aidl_data.read()?;
      let _arg_password: String = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#enablePairingByQrCode(&_arg_serviceName, &_arg_password);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getAdbWirelessPort => {
      let _aidl_return = _aidl_service.r#getAdbWirelessPort();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#disablePairing => {
      let _aidl_return = _aidl_service.r#disablePairing();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#isAdbWifiSupported => {
      let _aidl_return = _aidl_service.r#isAdbWifiSupported();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#isAdbWifiQrSupported => {
      let _aidl_return = _aidl_service.r#isAdbWifiQrSupported();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#registerCallback => {
      let _arg_callback: binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#registerCallback(&_arg_callback);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#unregisterCallback => {
      let _arg_callback: binder::Strong<dyn crate::binder_impls::IAdbCallback::IAdbCallback> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#unregisterCallback(&_arg_callback);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IAdbManager as _7_android_5_debug_11_IAdbManager;
}
