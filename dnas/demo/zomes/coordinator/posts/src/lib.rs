use hdk::prelude::*;

/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn get_timestamp(_: ()) -> ExternResult<Timestamp> {
  Ok(Timestamp::now())
}
