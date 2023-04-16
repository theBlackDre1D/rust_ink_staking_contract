#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod staking_contract {
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;
    use ink::prelude::vec::Vec;
    use openbrush::test_utils::*;
    

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct StakingContract {

        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for StakingContract {}

    impl psp22::Internal for StakingContract {
        
        fn _do_safe_transfer_check(
            &mut self,
            _from: &AccountId,
            _to: &AccountId,
            _value: &Balance,
            _data: &Vec<u8>,
        ) -> Result<(), PSP22Error> {
            Ok(())
        }
    }

    impl StakingContract {
        
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut contract = Self::default();
            contract
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Could not mint");
            return contract;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;

            let staking_contract = StakingContract::new(mint_amount);

            let alice_balance = staking_contract.balance_of(accounts.alice);

            assert_eq!(alice_balance, mint_amount);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;
            let transfer_amount = 1_000;

            let mut staking_contract = StakingContract::new(mint_amount);
            let result = staking_contract.transfer(accounts.bob, transfer_amount, Vec::<u8>::new());

            let alice_balance = staking_contract.balance_of(accounts.alice);
            let bob_balance = staking_contract.balance_of(accounts.bob);

            assert!(result.is_ok());
            assert_eq!(alice_balance, mint_amount - transfer_amount);
            assert_eq!(bob_balance, transfer_amount);
        }
    }
}
