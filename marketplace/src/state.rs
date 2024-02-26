use alloc::{format, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{ContractPackageHash, Key, URef, U512};
use common::{o_unwrap, token::TokenIdentifier, ToStrKey};

use crate::{named_keys, serializable_structs, MarketError};

serializable_structs! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OrderbookEntry {
        pub owner: Key,
        pub token_contract: ContractPackageHash,
        pub token_id: TokenIdentifier,
        pub price: U512,
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct Counters {
        pub post_id: u64,
    }
}

// TODO: Remove counters and replace it with a singular usize
named_keys! {
    all_named_keys():
    
    // Common named keys
    val counters: Counters = Counters::default();
    
    // Order book specificic named keys
    dict orderbook_entry_by_id: OrderbookEntry;

    // Page table
    dict custodial_package_by_nft_package: ContractPackageHash;
}

impl Counters {
    pub fn read() -> Self {
        counters::read()
    }

    pub fn write(self) {
        counters::write(self);
    }
}

impl OrderbookEntry {
    pub fn by_id(id: u64) -> Self {
        o_unwrap!(
            orderbook_entry_by_id::try_read(&id.to_key()),
            MarketError::UnknownPostId
        )
    }

    pub fn write(self, id: u64) {
        orderbook_entry_by_id::write(&id.to_key(), self);
    }

    pub fn remove(id: u64) {
        orderbook_entry_by_id::remove(&id.to_key());
    }
}

pub fn set_custodial_package_by_nft_package(nft_package: ContractPackageHash, custodial_package: ContractPackageHash) {
    custodial_package_by_nft_package::write(&nft_package.to_key(), custodial_package);
}

pub fn get_custodial_package_by_nft_package(nft_package: ContractPackageHash) -> Option<ContractPackageHash> {
    custodial_package_by_nft_package::try_read(&nft_package.to_key())
}

pub fn set_target_purse_by_post_id(post_id: u64, purse: URef) {
    runtime::put_key(&format!("target_purse_{post_id}"), purse.into());
}

pub fn unset_target_purse_by_post_id(post_id: u64) {
    runtime::remove_key(&format!("target_purse_{post_id}"));
}

pub fn get_target_purse_by_post_id(post_id: u64) -> Option<URef> {
    runtime::get_key(&format!("target_purse_{post_id}")).and_then(|k| k.into_uref())
}
