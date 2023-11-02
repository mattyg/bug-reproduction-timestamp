use hdk::prelude::*;
use holochain::{conductor::{config::ConductorConfig}, sweettest::*};
use chrono::Utc;

#[tokio::test(flavor = "multi_thread")]
async fn can_get_timestamp() {
    // Use prebuilt dna file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../../workdir/demo.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductor: SweetConductor= SweetConductor::from_config(ConductorConfig::default()).await;
    let app = conductor.setup_app("demo", &[dna]).await.unwrap();
    let (alice,) = app.into_tuple();

    // Alice gets a u32 response
    let timestamp_start = Utc::now().timestamp_micros();
    let value: Timestamp = conductor
        .call(&alice.zome("posts"), "get_timestamp", ())
        .await;
    let timestamp_end = Utc::now().timestamp_micros();

    assert!(value.0 > timestamp_start && value.0 < timestamp_end);
}
