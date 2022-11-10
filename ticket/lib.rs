#![cfg_attr(not(feature = "std"), no_std)]

pub use self::ticket::{Ticket, TicketRef};
use ink_lang as ink;

#[ink::contract]
mod ticket {
    use ink_env::call::FromAccountId;
    use ink_storage::traits::SpreadAllocate;

    impl SpreadAllocate for TicketRef {
        fn allocate_spread(_ptr: &mut ink_primitives::KeyPtr) -> Self {
            FromAccountId::from_account_id([0; 32].into())
        }
    }
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Ticket {
        /// Stores a single `Balance` value on the storage.
        pub value: Balance,
    }

    impl Ticket {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: Balance) -> Self {
            Self { value: init_value }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn increase(&mut self) {
            self.value += 1;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> Balance {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let ticket = Ticket::new(1);
            assert_eq!(ticket.get(), 1);
        }
    }
}
