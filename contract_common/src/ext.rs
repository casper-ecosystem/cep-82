pub mod cep78 {
    use crate::token::TokenIdentifier;
    use crate::{named_arg, trace_block};
    use alloc::{string::String, vec};
    use casper_contract::contract_api::runtime;
    use casper_types::Key;

    pub fn transfer(
        token_id: &TokenIdentifier,
        source_key: Key,
        target_key: Key,
    ) {
        let package = token_id.package;
        trace_block! {{
            runtime::call_versioned_contract::<(String, Key)>(
                package,
                None,
                "transfer",
                vec![
                    token_id.to_named_arg(),
                    named_arg!(source_key),
                    named_arg!(target_key),
                ]
                .into(),
            );
        }}
    }

    pub fn metadata(token_id: TokenIdentifier) -> String {
        let package = token_id.package;
        trace_block! {{
            runtime::call_versioned_contract::<String>(
                package,
                None,
                "metadata",
                vec![token_id.to_named_arg()].into(),
            )
        }}
    }

    pub fn owner_of(token_id: &TokenIdentifier) -> Key {
        let package = token_id.package;
        trace_block! {{
            runtime::call_versioned_contract::<Key>(
                package,
                None,
                "owner_of",
                vec![token_id.to_named_arg()].into(),
            )
        }}
    }

    pub fn get_approved(token_id: &TokenIdentifier) -> Option<Key> {
        let package = token_id.package;
        trace_block! {{
            runtime::call_versioned_contract::<Option<Key>>(
                package,
                None,
                "get_approved",
                vec![token_id.to_named_arg()].into(),
            )
        }}
    }
}

pub mod erc20 {
    use alloc::vec;
    use casper_contract::contract_api::runtime;
    use casper_types::{ContractPackageHash, Key, U256};

    use crate::{named_arg, trace_block};

    pub fn approve(package: ContractPackageHash, spender: Key, amount: U256) {
        trace_block! {{
            runtime::call_versioned_contract::<()>(
                package,
                None,
                "approve",
                vec![named_arg!(spender), named_arg!(amount)].into(),
            );
        }}
    }

    pub fn transfer_from(package: ContractPackageHash, owner: Key, recipient: Key, amount: U256) {
        trace_block! {{
            runtime::call_versioned_contract::<()>(
                package,
                None,
                "transfer_from",
                vec![named_arg!(owner), named_arg!(recipient), named_arg!(amount)].into(),
            );
        }}
    }

    pub fn transfer(package: ContractPackageHash, recipient: Key, amount: U256) {
        trace_block! {{
            runtime::call_versioned_contract::<()>(
                package,
                None,
                "transfer",
                vec![named_arg!(recipient), named_arg!(amount)].into(),
            );
        }}
    }

    pub fn balance_of(package: ContractPackageHash, address: Key) -> U256 {
        trace_block! {{
            runtime::call_versioned_contract::<U256>(
                package,
                None,
                "balance_of",
                vec![named_arg!(address)].into(),
            )
        }}
    }
}

pub mod cep82 {
    pub mod custodial {
        use alloc::vec;
        use casper_contract::contract_api::runtime;
        use casper_types::{ContractPackageHash, Key, URef, U512};

        use crate::{named_arg, token::TokenIdentifier, trace_block};

        pub fn calculate_royalty(
            package: ContractPackageHash,
            token_id: &TokenIdentifier,
            payment_amount: U512,
        ) -> U512 {
            let token_contract = token_id.package;
            trace_block! {{
                runtime::call_versioned_contract::<U512>(
                    package,
                    None,
                    "calculate_royalty",
                    vec![
                        named_arg!(token_contract),
                        token_id.to_named_arg(),
                        named_arg!(payment_amount),
                    ].into(),
                )
            }}
        }

        #[allow(clippy::too_many_arguments)]
        pub fn pay_royalty(
            package: ContractPackageHash,
            token_id: &TokenIdentifier,
            source_purse: URef,
            payer: Key,
            source_key: Key,
            target_key: Key,
            payment_amount: U512,
        ) {
            let token_contract = token_id.package;
            trace_block! {{
                runtime::call_versioned_contract::<()>(
                    package,
                    None,
                    "pay_royalty",
                    vec![
                        named_arg!(token_contract),
                        token_id.to_named_arg(),
                        named_arg!(source_purse),
                        named_arg!(payer),
                        named_arg!(source_key),
                        named_arg!(target_key),
                        named_arg!(payment_amount),
                    ].into(),
                )
            }}
        }
    }
}
