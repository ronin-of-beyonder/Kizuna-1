use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::{
        Address,
        AddressableContent
    },
    prelude::*,
};
use holochain_anchors::anchor;
use crate::profile::{
    ProfileEntry,
    Profile,
    PROFILE_LINK_TYPE,
    PROFILES_ANCHOR_TYPE, 
    PROFILES_ANCHOR_TEXT
};

fn profiles_anchor() -> ZomeApiResult<Address> {
    anchor(PROFILES_ANCHOR_TYPE.to_string(), PROFILES_ANCHOR_TEXT.to_string())
}

pub fn create_profile(profile_input: ProfileEntry) -> ZomeApiResult<Profile> {
    // let new_profile_entry = Profile::entry(profile_input);
    let new_profile_entry = Profile::entry(profile_input.clone());
    // let new_profile_entry = Entry::App(PROFILE_ENTRY_NAME.into(), profile_input.clone().into());
    let address = hdk::commit_entry(&new_profile_entry)?;
    hdk::link_entries(&profiles_anchor()?, &address, PROFILE_LINK_TYPE, "")?;
    Profile::new(address, profile_input)
}

pub fn get_profile(id: Address) -> ZomeApiResult<Profile> {
    let entry: ProfileEntry = hdk::utils::get_as_type(id.clone())?; //returns type result
    Profile::new(id, entry)
}

pub fn list_profiles() -> ZomeApiResult<Vec<Profile>> {
    // needs to be updated to solve hot spots
    hdk::get_links_and_load(&profiles_anchor()?, LinkMatch::Exactly(PROFILE_LINK_TYPE), LinkMatch::Any)
        .map(|profile_list|{
            profile_list.into_iter()
                .filter_map(Result::ok)
                .flat_map(|entry| {
                    let id = entry.address();
                    hdk::debug(format!("list_entry{:?}", entry)).ok();
                    get_profile(id)
                }).collect()
        })
}