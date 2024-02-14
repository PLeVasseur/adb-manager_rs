/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen/android/debug/PairDevice.rs.d -o out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen -Nexternal/rust/adb-manager_rs/aidl-rust-codegen/src/aidl external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android/debug/PairDevice.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#PairDevice {
  pub r#name: String,
  pub r#guid: String,
  pub r#connected: bool,
}
impl Default for r#PairDevice {
  fn default() -> Self {
    Self {
      r#name: Default::default(),
      r#guid: Default::default(),
      r#connected: false,
    }
  }
}
impl binder::Parcelable for r#PairDevice {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#name)?;
      subparcel.write(&self.r#guid)?;
      subparcel.write(&self.r#connected)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#name = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#guid = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#connected = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PairDevice);
binder::impl_deserialize_for_parcelable!(r#PairDevice);
impl binder::binder_impl::ParcelableMetadata for r#PairDevice {
  fn get_descriptor() -> &'static str { "android.debug.PairDevice" }
}
pub(crate) mod mangled {
 pub use super::r#PairDevice as _7_android_5_debug_10_PairDevice;
}
