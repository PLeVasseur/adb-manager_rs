use binder::Strong;
use aidl_rust_codegen::binder_impls::IAdbManager::{IAdbManager};
use aidl_rust_codegen::binder_impls::IAdbCallback::{IAdbCallback};

pub fn run() -> anyhow::Result<()> {
    let iadb_manager: Strong<dyn IAdbManager> = binder::get_interface("adb").unwrap();

    let devices = iadb_manager.r#getPairedDevices()?;

    println!("getPairedDevices(): {:?}", devices);

//     println!("Do onReceive()");
//     let umessage = UMessage {
//         source: Some(UUri {
//             authority: Some(UAuthority {
//                 name: Some("my_vin".to_owned()),
//                 ..Default::default()
//             }).into(),
//             entity: Some(UEntity {
//                 name: "door".to_owned(),
//                 ..Default::default()
//             }).into(),
//             resource: Some(UResource {
//                 name: "front_left".to_owned(),
//                 ..Default::default()
//             }).into(),
//             ..Default::default()
//         }).into(),
//         ..Default::default()
//     };
//     let parcelable_umessage = ParcelableUMessage::from(umessage);
//     println!("parcelable_umessage prior to onReceive: {:?}", parcelable_umessage);
//     let res = test_iulistener_service.onReceive(&parcelable_umessage).expect("Failed to trigger onReceive");
//     println!("Got result: {:?}", res);
//     println!("Done!");
    Ok(())
}
