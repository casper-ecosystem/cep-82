#![no_std]

extern crate alloc;

pub mod entry_point;
pub mod state;

use common::{
    call_stack::{self, CallStackElementEx},
    ext, o_unwrap,
    prelude::*,
    r_unwrap, store_named_key_incremented,
    token::TokenIdentifier,
};

use state::{get_custodial_package_by_nft_package, set_custodial_package_by_nft_package, unset_target_purse_by_post_id, OrderbookEntry};

use crate::state::{set_target_purse_by_post_id, get_target_purse_by_post_id, Counters};

pub const NK_ACCESS_UREF: &str = "cep82_marketplace_uref";
pub const NK_CONTRACT: &str = "cep82_marketplace";

pub const NAME: &str = "marketplace";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MarketError {
    InvalidMethodAccess = 201,
    InvalidPaymentAmount= 202,
    MustBeApproved= 203,
    UnsupportedNFTContract = 204,
    UnknownPostId = 205,
    UnknownTokenId = 206,
    ArithmeticOverflow = 207,
}

impl From<MarketError> for ApiError {
    fn from(error: MarketError) -> Self {
        ApiError::User(error as u16)
    }
}

pub fn install() {
    let named_keys = state::all_named_keys().into_iter().collect::<_>();
    let entry_points = entry_point::all_entrypoints().into();

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    store_named_key_incremented(access_uref.into(), NK_ACCESS_UREF);
    store_named_key_incremented(contract_hash.into(), NK_CONTRACT);
}

pub fn bid(
    source_purse: URef,
    post_id: u64,
    amount: U512
) {   
    let entry = OrderbookEntry::by_id(post_id);

    if amount < entry.price {
        revert(MarketError::InvalidPaymentAmount);
    }
    
    let source_key = entry.owner;
    let target_key = call_stack::caller().key();

    let custodial_package = get_custodial_package_by_nft_package(entry.token_contract);
    if let Some(custodial_package) = custodial_package {
        let royalty_amount = ext::cep82::custodial::calculate_royalty(
            custodial_package,
            entry.token_contract,
            &entry.token_id,
            entry.price,
        );

        let owned_purse = casper_contract::contract_api::system::create_purse();

        let remaining_amount = amount
            .checked_sub(royalty_amount)
            .unwrap_or_revert_with(MarketError::ArithmeticOverflow);
        
        r_unwrap!(
            casper_contract::contract_api::system::transfer_from_purse_to_purse(
                source_purse,
                owned_purse,
                royalty_amount,
                None,
            )
        );

        let target_purse = o_unwrap!(get_target_purse_by_post_id(post_id), MarketError::UnknownPostId);
        r_unwrap!(
            casper_contract::contract_api::system::transfer_from_purse_to_purse(
                source_purse,
                target_purse,
                remaining_amount,
                None,
            )
        );

        ext::cep82::custodial::pay_royalty(
            custodial_package,
            entry.token_contract,
            &entry.token_id,
            owned_purse,
            target_key,
            source_key,
            target_key,
            entry.price,
        );
    }

    ext::cep78::transfer(
        entry.token_contract,
        &entry.token_id,
        source_key,
        target_key
    );

    unset_target_purse_by_post_id(post_id);
    OrderbookEntry::remove(post_id);
}

pub fn post(
    token_contract: ContractPackageHash,
    token_id: TokenIdentifier,
    target_purse: URef,
    price: U512,
) -> u64 {
    let approved = o_unwrap!(
        ext::cep78::get_approved(token_contract, &token_id),
        MarketError::MustBeApproved
    );

    let this: Key = call_stack::current_contract().into();
    ensure_eq!(approved, this, MarketError::MustBeApproved);

    let caller = call_stack::caller().key();
    let owner = ext::cep78::owner_of(token_contract, &token_id);
    ensure_eq!(owner, caller, MarketError::InvalidMethodAccess);

    let mut counters = Counters::read();
    let post_id = counters.post_id;
    counters.post_id += 1;
    counters.write();

    let entry = OrderbookEntry {
        owner,
        token_contract,
        token_id,
        price,
    };

    set_target_purse_by_post_id(post_id, target_purse);

    entry.write(post_id);

    post_id
}

pub fn cancel_posting(post_id: u64) {
    let caller = call_stack::caller().key();
    let entry = OrderbookEntry::by_id(post_id);

    if entry.owner != caller {
        revert(MarketError::InvalidMethodAccess);
    }

    OrderbookEntry::remove(post_id);
}

pub fn register_custodial_contract(
    nft_package: ContractPackageHash,
    custodial_package: ContractPackageHash
) {
    set_custodial_package_by_nft_package(nft_package, custodial_package);
}