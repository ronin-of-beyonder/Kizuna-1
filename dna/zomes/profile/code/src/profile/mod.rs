use serde_derive::{Deserialize, Serialize};
use holochain_json_derive::DefaultJson; 
use hdk::{
    self,
    entry,
    from,
    link,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    prelude::*,
    holochain_persistence_api::cas::content::Address
};

pub mod handlers;

const PROFILE_ENTRY_NAME: &str = "profile";
const PROFILE_LINK_TYPE: &str = "profile_link";
const PROFILES_ANCHOR_TYPE: &str = "profiles";
const PROFILES_ANCHOR_TEXT: &str = "profiles";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// #[serde(rename_all = "snake_case")]
pub struct Profile {
    id: Address,
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// #[serde(rename_all = "snake_case")]
pub struct ProfileEntry {
    first_name: String,
    last_name: String,
    email: String,
}

impl Profile {
    pub fn new(id: Address, profile_input: ProfileEntry) ->  ZomeApiResult<Profile> {
        Ok(Profile {
            id,
            first_name: profile_input.first_name,
            last_name: profile_input.last_name,
            email: profile_input.email
        })
    }

    pub fn entry(profile_input: ProfileEntry) -> Entry {
        Entry::App(PROFILE_ENTRY_NAME.into(), profile_input.into())
    }

    // pub fn profile_from_entry(entry_input: Entry) -> ZomeApiResult<Profile> {
    //     Ok(Profile {
    //         id: Entry.address.clone(),
    //         first_name: entry_input.first_name,
    //         second_name: entry_input.last_name,
    //         email: entry_input.email
    //     })
    // }
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: PROFILE_ENTRY_NAME,
        description: "this is the profile spec of the user",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<ProfileEntry>| {
            Ok(())
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: PROFILE_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}