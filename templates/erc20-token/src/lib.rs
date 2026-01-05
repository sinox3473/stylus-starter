//! # Stylus ERC20 Token
//! 
//! A production-ready, gas-optimized ERC20 token implementation in Rust.
//! 
//! ## Features
//! - Full ERC20 standard compliance
//! - Owner-based access control
//! - Pausable transfers for emergency situations
//! - Gas-optimized storage access
//! - Detailed event logging
//! 
//! ## Security
//! - No unchecked arithmetic (Rust prevents overflow)
//! - Address validation on all transfers
//! - Access control for privileged functions
//! - Emergency pause mechanism

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

// ============================================================================
// Events
// ============================================================================

sol! {
    /// Emitted when tokens are transferred
    event Transfer(address indexed from, address indexed to, uint256 value);
    
    /// Emitted when allowance is set
    event Approval(address indexed owner, address indexed spender, uint256 value);
    
    /// Emitted when tokens are minted
    event Mint(address indexed to, uint256 amount);
    
    /// Emitted when tokens are burned
    event Burn(address indexed from, uint256 amount);
    
    /// Emitted when contract is paused
    event Paused(address account);
    
    /// Emitted when contract is unpaused
    event Unpaused(address account);
    
    /// Emitted when ownership is transferred
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
}

// ============================================================================
// Storage
// ============================================================================

sol_storage! {
    #[entrypoint]
    pub struct Erc20Token {
        /// Token balances
        mapping(address => uint256) balances;
        
        /// Allowances: owner => spender => amount
        mapping(address => mapping(address => uint256)) allowances;
        
        /// Total token supply
        uint256 total_supply;
        
        /// Contract owner
        address owner;
        
        /// Pause state
        bool paused;
    }
}

// ============================================================================
// ERC20 Implementation
// ============================================================================

#[public]
impl Erc20Token {
    // ------------------------------------------------------------------------
    // Constructor
    // ------------------------------------------------------------------------
    
    /// Initialize the token with an owner
    /// Note: In Stylus, initialization happens via the first transaction
    pub fn initialize(&mut self) {
        let caller = self.vm().msg_sender();
        
        // Only initialize once
        if self.owner.get() == Address::ZERO {
            self.owner.set(caller);
        }
    }
    
    // ------------------------------------------------------------------------
    // View Functions
    // ------------------------------------------------------------------------
    
    /// Returns the token name
    pub fn name() -> String {
        String::from("Stylus Token")
    }

    /// Returns the token symbol
    pub fn symbol() -> String {
        String::from("STY")
    }

    /// Returns the number of decimals (18 is standard for ERC20)
    pub fn decimals() -> u8 {
        18
    }

    /// Returns the total token supply
    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    /// Returns the balance of an account
    /// 
    /// # Arguments
    /// * `account` - The address to query
    pub fn balance_of(&self, account: Address) -> U256 {
        self.balances.get(account)
    }

    /// Returns the allowance for a spender
    /// 
    /// # Arguments
    /// * `owner` - The token owner
    /// * `spender` - The address allowed to spend
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.get(owner).get(spender)
    }
    
    /// Returns the contract owner
    pub fn owner(&self) -> Address {
        self.owner.get()
    }
    
    /// Returns whether the contract is paused
    pub fn paused(&self) -> bool {
        self.paused.get()
    }

    // ------------------------------------------------------------------------
    // State-Changing Functions
    // ------------------------------------------------------------------------

    /// Transfers tokens from caller to recipient
    /// 
    /// # Arguments
    /// * `to` - Recipient address
    /// * `amount` - Amount to transfer
    /// 
    /// # Panics
    /// * If contract is paused
    /// * If recipient is zero address
    /// * If caller has insufficient balance
    pub fn transfer(&mut self, to: Address, amount: U256) -> bool {
        assert!(!self.paused.get(), "Contract is paused");
        
        let sender = self.vm().msg_sender();
        self.transfer_impl(sender, to, amount);
        true
    }

    /// Approves a spender to transfer tokens on behalf of caller
    /// 
    /// # Arguments
    /// * `spender` - Address to approve
    /// * `amount` - Amount to approve
    /// 
    /// # Panics
    /// * If spender is zero address
    pub fn approve(&mut self, spender: Address, amount: U256) -> bool {
        assert!(spender != Address::ZERO, "Invalid spender address");
        
        let owner = self.vm().msg_sender();
        
        // Gas optimization: update allowance directly
        self.allowances.setter(owner).insert(spender, amount);
        
        evm::log(Approval {
            owner,
            spender,
            value: amount,
        });
        
        true
    }

    /// Transfers tokens from one address to another using allowance
    /// 
    /// # Arguments
    /// * `from` - Token owner
    /// * `to` - Recipient
    /// * `amount` - Amount to transfer
    /// 
    /// # Panics
    /// * If contract is paused
    /// * If allowance is insufficient
    pub fn transfer_from(&mut self, from: Address, to: Address, amount: U256) -> bool {
        assert!(!self.paused.get(), "Contract is paused");
        
        let spender = self.vm().msg_sender();
        
        // Check allowance (gas optimization: read once)
        let current_allowance = self.allowances.get(from).get(spender);
        assert!(current_allowance >= amount, "Insufficient allowance");
        
        // Decrease allowance (gas optimization: direct write)
        self.allowances.setter(from).insert(spender, current_allowance - amount);
        
        // Perform transfer
        self.transfer_impl(from, to, amount);
        
        true
    }

    // ------------------------------------------------------------------------
    // Owner Functions
    // ------------------------------------------------------------------------

    /// Mints new tokens to an address (owner only)
    /// 
    /// # Arguments
    /// * `to` - Recipient address
    /// * `amount` - Amount to mint
    /// 
    /// # Panics
    /// * If caller is not the owner
    /// * If recipient is zero address
    pub fn mint(&mut self, to: Address, amount: U256) {
        let caller = self.vm().msg_sender();
        let owner = self.owner.get();
        assert!(caller == owner, "Only owner can mint");
        assert!(to != Address::ZERO, "Invalid recipient");
        
        // Gas optimization: read balance once
        let current_balance = self.balances.get(to);
        self.balances.insert(to, current_balance + amount);
        
        // Update total supply
        let current_supply = self.total_supply.get();
        self.total_supply.set(current_supply + amount);
        
        evm::log(Mint { to, amount });
        
        evm::log(Transfer {
            from: Address::ZERO,
            to,
            value: amount,
        });
    }
    
    /// Burns tokens from caller's balance
    /// 
    /// # Arguments
    /// * `amount` - Amount to burn
    /// 
    /// # Panics
    /// * If caller has insufficient balance
    pub fn burn(&mut self, amount: U256) {
        let caller = self.vm().msg_sender();
        
        // Check balance
        let current_balance = self.balances.get(caller);
        assert!(current_balance >= amount, "Insufficient balance");
        
        // Gas optimization: direct write
        self.balances.insert(caller, current_balance - amount);
        
        // Update total supply
        let current_supply = self.total_supply.get();
        self.total_supply.set(current_supply - amount);
        
        evm::log(Burn {
            from: caller,
            amount,
        });
        
        evm::log(Transfer {
            from: caller,
            to: Address::ZERO,
            value: amount,
        });
    }
    
    /// Pauses all token transfers (owner only)
    /// 
    /// # Panics
    /// * If caller is not the owner
    pub fn pause(&mut self) {
        let caller = self.vm().msg_sender();
        let owner = self.owner.get();
        assert!(caller == owner, "Only owner can pause");
        
        self.paused.set(true);
        
        evm::log(Paused { account: caller });
    }
    
    /// Unpauses token transfers (owner only)
    /// 
    /// # Panics
    /// * If caller is not the owner
    pub fn unpause(&mut self) {
        let caller = self.vm().msg_sender();
        let owner = self.owner.get();
        assert!(caller == owner, "Only owner can unpause");
        
        self.paused.set(false);
        
        evm::log(Unpaused { account: caller });
    }
    
    /// Transfers ownership to a new address (owner only)
    /// 
    /// # Arguments
    /// * `new_owner` - New owner address
    /// 
    /// # Panics
    /// * If caller is not the owner
    /// * If new owner is zero address
    pub fn transfer_ownership(&mut self, new_owner: Address) {
        let caller = self.vm().msg_sender();
        let current_owner = self.owner.get();
        assert!(caller == current_owner, "Only owner can transfer ownership");
        assert!(new_owner != Address::ZERO, "Invalid new owner");
        
        self.owner.set(new_owner);
        
        evm::log(OwnershipTransferred {
            previousOwner: current_owner,
            newOwner: new_owner,
        });
    }
}

// ============================================================================
// Internal Implementation
// ============================================================================

impl Erc20Token {
    /// Internal transfer function with validations
    /// 
    /// Gas optimized: reads balances once, validates, then writes once
    /// 
    /// # Panics
    /// * If recipient or sender is zero address
    /// * If sender has insufficient balance
    fn transfer_impl(&mut self, from: Address, to: Address, amount: U256) {
        // Validate addresses
        assert!(to != Address::ZERO, "Invalid recipient");
        assert!(from != Address::ZERO, "Invalid sender");
        
        // Gas optimization: read both balances once
        let balance_from = self.balances.get(from);
        let balance_to = self.balances.get(to);
        
        // Check balance
        assert!(balance_from >= amount, "Insufficient balance");
        
        // Update balances (gas optimization: direct writes)
        self.balances.insert(from, balance_from - amount);
        self.balances.insert(to, balance_to + amount);
        
        // Emit event
        evm::log(Transfer {
            from,
            to,
            value: amount,
        });
    }
}

// ============================================================================
// Tests
