#![no_std]
#![no_main]

use common::forward_entrypoints;
#[allow(unused)]
use marketplace::entry_point as ep;

forward_entrypoints! {
    ep: [ call ]
}

forward_entrypoints! {
    ep: [
        bid,
        post,
        cancel_posting,
        register_custodial_contract
    ]
}
