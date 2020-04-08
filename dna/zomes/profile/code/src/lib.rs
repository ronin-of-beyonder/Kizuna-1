#![feature(proc_macro_hygiene)]
// #[allow(dead_code)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
use serde_derive::{Deserialize, Serialize};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address
};

use crate::profile::ProfileEntry;
use crate::profile::Profile;
pub mod profile;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

#[zome]
mod profile_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn profile_def() -> ValidatingEntryType {
        profile::definition()
    }


    #[entry_def]
    fn anchor_def() ->ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn create_profile(profile_input: ProfileEntry) -> ZomeApiResult<Profile> {
        profile::handlers::create_profile(profile_input)
    }


    #[zome_fn("hc_public")]
    fn get_profile(id: Address) -> ZomeApiResult<Option<Entry>>  {
        // profile::handlers::get_profile(id)
        hdk::get_entry(&id)
    }

    
    #[zome_fn("hc_public")]
    fn list_profiles() -> ZomeApiResult<Vec<Profile>> {
        profile::handlers::list_profiles()
    }

}
