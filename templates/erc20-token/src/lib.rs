#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    prelude::*,
    evm,
};

// Define ERC20 events
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// Storage structure for ERC20
sol_storage! {
    #[entrypoint]
    pub struct Erc20Token {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
    }
}

// ERC20 implementation
#[public]
impl Erc20Token {
    // Get token name
    pub fn name() -> String {
        String::from("Stylus Token")
    }

    // Get token symbol
    pub fn symbol() -> String {
        String::from("STY")
    }

    // Get decimals
    pub fn decimals() -> u8 {
        18
    }

    // Get total supply
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    // Get balance of an account
    pub fn balance_of(&self, account: Address) -> U256 {
        self.balances.get(account)
    }

    // Transfer tokens
    pub fn transfer(&mut self, to: Address, amount: U256) -> bool {
        let sender = self.vm().msg_sender();
        self.transfer_impl(sender, to, amount);
        true
    }

    // Approve spender
    pub fn approve(&mut self, spender: Address, amount: U256) -> bool {
        let owner = self.vm().msg_sender();
        
        self.allowances.setter(owner).insert(spender, amount);
        
        evm::log(Approval {
            owner,
            spender,
            value: amount,
        });
        
        true
    }

    // Transfer from (using allowance)
    pub fn transfer_from(&mut self, from: Address, to: Address, amount: U256) -> bool {
        let spender = self.vm().msg_sender();
        
        // Check and decrease allowance
        let allowed = self.allowances.get(from).get(spender);
        assert!(allowed >= amount, "Insufficient allowance");
        
        self.allowances.setter(from).insert(spender, allowed - amount);
        
        // Transfer
        self.transfer_impl(from, to, amount);
        
        true
    }

    // Get allowance
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.get(owner).get(spender)
    }

    // Mint tokens (for initialization)
    pub fn mint(&mut self, to: Address, amount: U256) {
        assert!(to != Address::ZERO, "Invalid recipient");
        
        let new_balance = self.balances.get(to) + amount;
        self.balances.insert(to, new_balance);
        
        let new_supply = self.total_supply.get() + amount;
        self.total_supply.set(new_supply);
        
        evm::log(Transfer {
            from: Address::ZERO,
            to,
            value: amount,
        });
    }
}

// Internal implementation
impl Erc20Token {
    fn transfer_impl(&mut self, from: Address, to: Address, amount: U256) {
        // Validate addresses
        assert!(to != Address::ZERO, "Invalid recipient");
        assert!(from != Address::ZERO, "Invalid sender");
        
        // Check balance
        let balance_from = self.balances.get(from);
        assert!(balance_from >= amount, "Insufficient balance");
        
        // Update balances
        self.balances.insert(from, balance_from - amount);
        let balance_to = self.balances.get(to);
        self.balances.insert(to, balance_to + amount);
        
        // Emit event
        evm::log(Transfer {
            from,
            to,
            value: amount,
        });
    }
}
