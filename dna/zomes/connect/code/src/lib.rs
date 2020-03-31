#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

const FIRST_NAME_ENTRY_NAME: &str = "first_name";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FirstName {
    name: String,
}

impl FirstName {
    pub fn new(first_name: String) -> Self {
        FirstName {
            name: first_name
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App(FIRST_NAME_ENTRY_NAME.into(), self.into())
    }
}

#[zome]
mod kizuna_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn definition() -> ValidatingEntryType {
        entry!(
            name: FIRST_NAME_ENTRY_NAME,
            description: "this is the first name of the user",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<FirstName>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_first_name(first_name: String) -> ZomeApiResult<Address> {
        let new_first_name = FirstName::new(first_name);
        let new_first_name_entry = new_first_name.entry();
        let address = hdk::commit_entry(&new_first_name_entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_first_name(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }
}
