//! Entry points of the contract.
//!
//! Some methods have entry points that are not listed in the returned `EntryPoints` object.
//! These are either optional or only contextually available. See the documentation of the
//! individual methods for more information.

use casper_types::{ContractPackageHash, URef, U512};
use common::{entrypoint, entrypoints, token::TokenIdentifier};

entrypoint! {
    [install] fn call() -> () = crate::install
}

entrypoints! {
    [public contract] fn bid(
        source_purse: URef,
        post_id: u64,
        amount: U512
    ) -> () = crate::bid;

    [public contract] fn post(
        token_contract: ContractPackageHash,
        token_id: TokenIdentifier,
        target_purse: URef,
        price: U512,
    ) -> u64 = crate::post;

    [public contract] fn cancel_posting(
        post_id: u64
    ) -> () = crate::cancel_posting;

    [public contract] fn register_custodial_contract(
        nft_package: ContractPackageHash,
        custodial_package: ContractPackageHash
    ) -> () = crate::register_custodial_contract;
}
