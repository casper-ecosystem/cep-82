#![no_main]
#![no_std]

use casper_contract::contract_api::{account, runtime, system};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{runtime_args, ContractHash, RuntimeArgs, U512};

pub const AMOUNT: &str = "amount";
pub const MARKETPLACE_CONTRACT_HASH: &str = "marketplace_hash";
pub const POST_ID: &str = "post_id";
pub const BID_ENTRYPOINT: &str = "bid";
pub const SOURCE_PURSE: &str = "source_purse";

#[no_mangle]
pub extern "C" fn call() {
    let marketplace_contract_hash: ContractHash = runtime::get_named_arg(MARKETPLACE_CONTRACT_HASH);
    let post_id: u64 = runtime::get_named_arg(POST_ID);
    let amount: U512 = runtime::get_named_arg(AMOUNT);

    let owned_purse = system::create_purse();

    system::transfer_from_purse_to_purse(
        account::get_main_purse(),
        owned_purse,
        amount,
        None
    ).unwrap_or_revert();

    let _: () = runtime::call_contract(
        marketplace_contract_hash,
        BID_ENTRYPOINT,
        runtime_args! {
            POST_ID => post_id,
            SOURCE_PURSE => owned_purse,
            AMOUNT => amount
        },
    );
}