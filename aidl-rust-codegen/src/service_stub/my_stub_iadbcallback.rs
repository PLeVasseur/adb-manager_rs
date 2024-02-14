/// Implementation of the `IUListener` AIDL interface.
use android_debug_iadbmanager::aidl::android::debug::IAdbCallback::IAdbCallback;
use android_debug_iadbmanager::aidl::android::debug::AdbTransportType::AdbTransportType;
use android_debug_iadbmanager::binder;

/// The `MyStubIAdbCallback` implementation.
pub struct MyStubIAdbCallback;

impl binder::Interface for MyStubIAdbCallback {}

// oneway interface IAdbCallback {
//     /**
//      * On debugging enabled, service providing IAdbManager calls this function.
//      */
//     void onDebuggingChanged(boolean enabled, AdbTransportType type);
// }

impl IAdbCallback for MyStubIAdbCallback {
    fn onDebuggingChanged(&self, enabled: bool, r#type: AdbTransportType) -> binder::Result<()> {
        Ok(())
    }
}
