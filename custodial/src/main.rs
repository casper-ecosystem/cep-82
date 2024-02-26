#![no_std]
#![no_main]

use common::forward_entrypoints;
#[allow(unused)]
use custodial::entry_point as ep;

forward_entrypoints! {
    ep: [ call ]
}

forward_entrypoints! {
    ep: [
        calculate_royalty,
        can_transfer,
        pay_royalty,
    ]
}
