#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// #[openbrush::contract]
#[ink::contract]
pub mod cert_token {

    use openbrush::contracts::psp34::extensions::burnable::*;
    use openbrush::contracts::psp34::extensions::enumerable::*;
    use openbrush::contracts::psp34::extensions::metadata::*;
    use openbrush::contracts::psp34::extensions::mintable::*;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CertToken {
        #[storage_field]
        psp34: psp34::Data<Balances>,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34 for CertToken {}
    impl PSP34Burnable for CertToken {}
    impl PSP34Mintable for CertToken {}
    impl PSP34Enumerable for CertToken {}
    impl PSP34Metadata for CertToken {}

    impl CertToken {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let mut _instance = Self::default();
            _instance
                ._mint_to(_instance.env().caller(), Id::U8(1))
                .expect("First token can be minted");
            let collection_id = _instance.collection_id();
            _instance._set_attribute(
                collection_id.clone(),
                String::from(name),
                String::from(symbol),
            );

            _instance
        }
        #[ink(message)]
        pub fn getTokenOwner(&mut self, id: Id) -> Result<(Option<AccountId>), PSP34Error> {
            let owner = self._owner_of(&id);
            Ok(owner)
        }
    }
}
