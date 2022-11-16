#![cfg_attr(not(feature = "std"), no_std)]




#[ink::contract]
mod testERC {

    use erc20::Erc20Ref;

    #[ink(storage)]
    pub struct TestErc {
        erc20: Erc20Ref,
    }

    impl TestErc {

        #[ink(constructor, payable)]
        pub fn new(
            init_value: Balance,
            version: u32,
            erc20_code_hash: Hash,
        ) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let erc20 = Erc20Ref::new(init_value)
                .endowment(total_balance / 4)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the erc20 contract: {:?}",
                        error
                    )
                });
            Self {
                erc20,
            }
        }

        #[ink(message)]
        pub fn get(&self) ->  Balance{
            self.erc20.total_supply()        
        }

    }



}
