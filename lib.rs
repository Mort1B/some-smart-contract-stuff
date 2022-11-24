#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

// Strings should be made Vec<u8> in smart contracts and then parse on the UI side when contract is started
#[ink::contract]
mod ticket_event {
    use ink_prelude::string::String;
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use ticket::TicketRef;

    /// A ticket ID.
    pub type EventId = u32;
    /// Defines the storage of all values
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TicketEvent {
        /// Total amount of tickets available
        total_tickets: Balance,
        /// Mapping from ticket ID to owner
        ticket_owner: Mapping<EventId, AccountId>,
        /// Mapping from owner to list of owned tickets
        balance: Mapping<AccountId, Balance>,
        /// Name of event
        name: String,
        /// Location of the event
        location: String,
        /// Symbol of event
        symbol: String,
        /// Date of event
        date: String,
        /// Price of ticket
        price: u32,
        /// TicketRef
        ticket_ref: TicketRef,
    }

    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
    }

    impl TicketEvent {
        /// Constructor that initializes a new `TicketEvent` contract.
        #[ink(constructor)]
        pub fn new(
            total_tickets: Balance,
            version: u32,
            name: String,
            location: String,
            symbol: String,
            date: String,
            price: u32,
            ticket_ref_code_hash: Hash,
        ) -> Self {
            let caller = Self::env().caller();
            let salt = version.to_le_bytes();
            let ticket_ref = TicketRef::new(total_tickets)
                .endowment(15)
                .code_hash(ticket_ref_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!("Cannot instantiate contract: {:?}", error);
                });
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                //hardcoded now, should just use random number generator
                contract.total_tickets = total_tickets;
                contract.name = name;
                contract.location = location;
                contract.symbol = symbol;
                contract.date = date;
                contract.price = price;
                contract.balance.insert(&caller, &total_tickets);
                contract.ticket_owner.insert(&0, &caller);
                contract.ticket_ref = ticket_ref;
            })
        }

        /// Returns the owner of the event
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.env().caller()
        }

        /// Returns the name of the event
        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        /// Returns the location of the event
        #[ink(message)]
        pub fn get_location(&self) -> String {
            self.location.clone()
        }

        /// Returns the total amount of tickets available
        #[ink(message)]
        pub fn get_total_tickets(&self) -> Balance {
            self.total_tickets
        }

        /// Returns the price of the ticket
        #[ink(message)]
        pub fn get_price(&self) -> u32 {
            self.price
        }

        /// Returns the date of the event
        #[ink(message)]
        pub fn get_date(&self) -> String {
            self.date.clone()
        }

        /// Returns the symbol of the event
        #[ink(message)]
        pub fn get_symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Returns the balance of the owner
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            let caller = self.env().caller();
            self.balance.get(&caller).unwrap_or(0)
        }

        /// Returns the balance of the address
        #[ink(message)]
        pub fn get_balance_of(&self, owner: AccountId) -> Balance {
            self.balance.get(&owner).unwrap_or(0)
        }

        /// Mints new tickets
        #[ink(message)]
        pub fn mint(&mut self, event_id: EventId, amount: Balance) -> Result<(), Error> {
            let caller = self.env().caller();

            for _ in 0..amount {
                self.add_token_to(caller, event_id)?;
                self.total_tickets += 1;
            }
            Ok(())
        }

        /// Adds the token id to the AccountId
        #[ink(message)]
        pub fn add_token_to(&mut self, to: AccountId, event_id: EventId) -> Result<(), Error> {
            let balance = self.balance.get(&to).unwrap_or(0);
            self.balance.insert(&to, &(balance + 1));
            self.ticket_owner.insert(&event_id, &to);
            Ok(())
        }

        /// Transfers token id from the sender to the accountID
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            event_id: EventId,
            tickets: Balance,
        ) -> Result<(), Error> {
            // let caller = self.env().caller();
            if !self.exists(event_id) {
                return Err(Error::TokenNotFound);
            }

            for _ in 0..tickets {
                self.remove_token_from(from, event_id)?;
                self.add_token_to(to, event_id)?;
            }
            Ok(())
        }

        /// Removes token id from the owner
        #[ink(message)]
        pub fn remove_token_from(
            &mut self,
            from: AccountId,
            event_id: EventId,
        ) -> Result<(), Error> {
            let balance = self.balance.get(&from).unwrap_or(0);
            self.balance.insert(&from, &(balance - 1));
            self.ticket_owner.remove(event_id);
            Ok(())
        }

        /// Returnt true if the token id exists or false if it doesn't
        #[ink(message)]
        pub fn exists(&self, event_id: EventId) -> bool {
            self.ticket_owner.contains(&event_id)
        }

        /// return info from Ticket type TicketRef
        #[ink(message)]
        pub fn get_bool(&self) -> bool {
            true
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn create_event_works() {
            let contract = TicketEvent::new(
                100,
                1337,
                "Test_Name".to_string(),
                "Test_Location".to_string(),
                "Test_Symbol".to_string(),
                "Test_Date".to_string(),
                55,
                Hash::from([0x42; 32]),
            );
            assert_eq!(contract.get_total_tickets(), 100);
            assert_eq!(contract.get_name(), "Test_Name");
            assert_eq!(contract.get_location(), "Test_Location");
            assert_eq!(contract.owner(), AccountId::from([0x1; 32]));
            assert_eq!(contract.get_symbol(), "Test_Symbol");
            assert_eq!(contract.get_date(), "Test_Date");
            assert_eq!(contract.get_price(), 55);
            assert_eq!(contract.get_balance(), 100);
        }

        /// Testings minting of tickets
        #[ink::test]
        fn minting_tests() {
            let mut contract = TicketEvent::new(
                0,
                1337,
                "Test_Name".to_string(),
                "Test_Location".to_string(),
                "Test_Symbol".to_string(),
                "Test_Date".to_string(),
                55,
                Hash::from([0x42; 32]),
            );
            contract.mint(1, 10).unwrap();
            assert_eq!(contract.get_total_tickets(), 10);
            assert_eq!(contract.get_balance(), 10);
        }

        /// Testing changing of ownership
        #[ink::test]
        fn ownership_tests() {
            let mut contract = TicketEvent::new(
                0,
                1337,
                "Test_Name".to_string(),
                "Test_Location".to_string(),
                "Test_Symbol".to_string(),
                "Test_Date".to_string(),
                55,
                Hash::from([0x42; 32]),
            );
            contract.mint(1, 10).unwrap();
            contract
                .transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 1, 1)
                .unwrap();
            assert_eq!(contract.get_balance(), 9);
            assert_eq!(contract.get_balance_of(AccountId::from([0x2; 32])), 1);
            contract
                .transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 1, 5)
                .unwrap();
            assert_eq!(contract.get_balance(), 4);
            assert_eq!(contract.get_balance_of(AccountId::from([0x2; 32])), 6);
        }

        /// Testing transfering tickets with no ID should panic
        #[ink::test]
        #[should_panic(expected = "TokenNotFound")]
        fn transfering_tests() {
            let mut contract = TicketEvent::new(
                0,
                1337,
                "Test_Name".to_string(),
                "Test_Location".to_string(),
                "Test_Symbol".to_string(),
                "Test_Date".to_string(),
                55,
                Hash::from([0x42; 32]),
            );
            contract.mint(1, 10).unwrap();
            contract
                .transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 2, 1)
                .unwrap();
        }

        /// Test removing token with ID from owner
        #[ink::test]
        fn remove_token_tests() {
            let mut contract = TicketEvent::new(
                0,
                1337,
                "Test_Name".to_string(),
                "Test_Location".to_string(),
                "Test_Symbol".to_string(),
                "Test_Date".to_string(),
                55,
                Hash::from([0x42; 32]),
            );
            contract.mint(1, 10).unwrap();
            contract.mint(2, 10).unwrap();
            contract
                .remove_token_from(AccountId::from([0x1; 32]), 1)
                .unwrap();
            assert_eq!(contract.get_balance(), 19);

            contract
                .transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 2, 5)
                .unwrap();
            assert_eq!(contract.get_balance(), 14);
            assert_eq!(contract.total_tickets, 20);
        }
    }
}
