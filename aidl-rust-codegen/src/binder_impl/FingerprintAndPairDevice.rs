/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: out/host/linux-x86/bin/aidl --lang=rust -Weverything -Wno-missing-permission-annotation --min_sdk_version current --ninja -d out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen/android/debug/FingerprintAndPairDevice.rs.d -o out/soong/.intermediates/external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android.debug.iadbmanager-rust-source/gen -Nexternal/rust/adb-manager_rs/aidl-rust-codegen/src/aidl external/rust/adb-manager_rs/aidl-rust-codegen/src/aidl/android/debug/FingerprintAndPairDevice.aidl
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct r#FingerprintAndPairDevice {
  pub r#keyFingerprint: String,
  pub r#device: crate::binder_impls::PairDevice::r#PairDevice,
}
impl Default for r#FingerprintAndPairDevice {
  fn default() -> Self {
    Self {
      r#keyFingerprint: Default::default(),
      r#device: Default::default(),
    }
  }
}
impl binder::Parcelable for r#FingerprintAndPairDevice {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#keyFingerprint)?;
      subparcel.write(&self.r#device)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#keyFingerprint = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#device = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#FingerprintAndPairDevice);
binder::impl_deserialize_for_parcelable!(r#FingerprintAndPairDevice);
impl binder::binder_impl::ParcelableMetadata for r#FingerprintAndPairDevice {
  fn get_descriptor() -> &'static str { "android.debug.FingerprintAndPairDevice" }
}
pub(crate) mod mangled {
 pub use super::r#FingerprintAndPairDevice as _7_android_5_debug_24_FingerprintAndPairDevice;
}
