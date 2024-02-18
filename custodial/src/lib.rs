#![no_std]

use alloc::{collections::BTreeMap, string::ToString};
use common::{
    call_stack::CallStackElementEx, o_unwrap, prelude::*, store_named_key_incremented, token::TokenIdentifier, ToStrKey
};
use state::{RoyaltyPaymentState, RoyaltyStructure};

extern crate alloc;

mod bytes;
pub mod entry_point;
pub mod state;

pub const NK_ACCESS_UREF: &str = "cep82_custodial_uref";
pub const NK_CONTRACT: &str = "cep82_custodial";
pub const NK_ROYALTY_PURSE: &str = "royalty_purse";
pub const NAME: &str = "custodial";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CustodialError {
    MarketplaceNotWhitelisted = 100,
    CallerMustBeContract = 101,
    CallerMustBeApproved = 102,
    SelfTransferForbidden = 103,
    SourceMustBeOwner = 104,
    AlreadyPaid = 105,
    MustPayRoyalties = 106,
    Overflow = 107,
    InvalidRoyaltyScheme = 108,
}

impl From<CustodialError> for ApiError {
    fn from(error: CustodialError) -> Self {
        ApiError::User(error as u16)
    }
}

fn install(
    whitelisted_marketplaces: Vec<ContractPackageHash>,
    royalty_structure: RoyaltyStructure,
    manager: Key,
) {
    let royalty_purse = casper_contract::contract_api::system::create_purse();
    let mut named_keys: BTreeMap<_, _> = state::init_all(manager, royalty_structure)
        .into_iter()
        .collect::<_>();

    named_keys.insert(NK_ROYALTY_PURSE.to_string(), royalty_purse.into());

    let entry_points = entry_point::all_entrypoints().into();

    init(whitelisted_marketplaces);

    let (contract_package_hash, access_uref) = storage::create_contract_package_at_hash();
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    store_named_key_incremented(access_uref.into(), NK_ACCESS_UREF);
    store_named_key_incremented(contract_hash.into(), NK_CONTRACT);
}

fn init(whitelisted_marketplaces: Vec<ContractPackageHash>) {
    if whitelisted_marketplaces.is_empty() {
        state::marketplace_whitelist_enabled::write(false);
    } else {
        state::marketplace_whitelist_enabled::write(true);

        for marketplace in whitelisted_marketplaces {
            let marketplace_key = marketplace.to_key();
            state::whitelisted_marketplaces::write(&marketplace_key, true);
        }
    }
}

fn pay_royalty(
    token_contract: ContractPackageHash,
    token_id: TokenIdentifier,
    source_purse: URef,
    payer: Key,
    source_key: Key,
    target_key: Key,
    payment_amount: U512,
) {
    ensure_neq!(
        source_key,
        target_key,
        CustodialError::SelfTransferForbidden
    );

    let caller_contract_hash: Key = o_unwrap!(
        common::call_stack::caller()
            .contract_hash()
            .cloned(),
        CustodialError::CallerMustBeContract
    )
    .into();

    let caller_contract_package = o_unwrap!(
        common::call_stack::caller().contract_package(),
        CustodialError::CallerMustBeContract
    );

    let is_whitelisted = (!state::marketplace_whitelist_enabled::read())
    || state::is_marketplace_whitelisted(caller_contract_package);
    ensure!(is_whitelisted, CustodialError::MarketplaceNotWhitelisted);

    let approved = o_unwrap!(
        common::ext::cep78::get_approved(token_contract, &token_id),
        CustodialError::CallerMustBeApproved
    );
    
    ensure_eq!(
        caller_contract_hash,
        approved,
        CustodialError::CallerMustBeApproved
    );

    let current_owner = common::ext::cep78::owner_of(token_contract, &token_id);
    ensure_eq!(current_owner, source_key, CustodialError::SourceMustBeOwner);

    let royalty_purse = runtime::get_key(NK_ROYALTY_PURSE)
        .unwrap_or_revert()
        .into_uref()
        .unwrap_or_revert();

    let token_key = token_id.to_key();
    let total_royalty = calculate_royalty(token_contract, token_id, payment_amount);

    let old_payment_state = state::royalty_payments::try_read(&token_key);
    if let Some(RoyaltyPaymentState::Paid {
        source_key: paid_source_key,
        ..
    }) = old_payment_state
    {
        ensure_neq!(source_key, paid_source_key, CustodialError::AlreadyPaid)
    }

    contract_api::system::transfer_from_purse_to_purse(
        source_purse,
        royalty_purse,
        total_royalty,
        None,
    )
    .unwrap_or_revert();

    let payment_state = RoyaltyPaymentState::Paid {
        payer,
        source_key,
        amount: total_royalty,
    };

    state::royalty_payments::write(&token_key, payment_state);
}

// This sample custodial implementation applies the same royalty regardless
// of the token. The interface is left open for those who wish to implement
// a more sophisticated royalty scheme.
fn calculate_royalty(
    _token_contract: ContractPackageHash,
    _token_id: TokenIdentifier,
    payment_amount: U512,
) -> U512 {
    let royalty_structure = o_unwrap!(
        state::royalty_structure::try_read(),
        CustodialError::InvalidRoyaltyScheme
    );
    royalty_structure.calculate_total_royalty(payment_amount)
}

fn transfer(token_contract: ContractPackageHash, token_id: TokenIdentifier, source_key: Key, target_key: Key) {
    o_unwrap!(
        common::call_stack::caller().contract_package(),
        CustodialError::CallerMustBeContract
    );

    let key = token_id.to_key();
    let payment_state = state::royalty_payments::read(&key);

    let RoyaltyPaymentState::Paid { source_key: payment_key, .. } = payment_state else {
        casper_contract::contract_api::runtime::revert(CustodialError::MustPayRoyalties);
    };

    let current_owner = common::ext::cep78::owner_of(token_contract, &token_id);

    if source_key == payment_key && source_key == current_owner {
        common::ext::cep78::transfer(
            token_contract,
            &token_id,
            source_key,
            target_key
        );

        state::royalty_payments::write(&key, RoyaltyPaymentState::Unpaid);
    }
}