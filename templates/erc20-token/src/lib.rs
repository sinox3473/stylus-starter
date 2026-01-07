//! # Stylus ERC721 NFT
//! 
//! A production-ready ERC721 (NFT) implementation in Rust.

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

sol! {
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
}

sol_storage! {
    #[entrypoint]
    pub struct Erc721Nft {
        mapping(uint256 => address) owners;
        mapping(address => uint256) balances;
        mapping(uint256 => address) token_approvals;
        mapping(address => mapping(address => bool)) operator_approvals;
        address owner;
        uint256 next_token_id;
    }
}

#[public]
impl Erc721Nft {
    pub fn initialize(&mut self) {
        let caller = self.vm().msg_sender();
        if self.owner.get() == Address::ZERO {
            self.owner.set(caller);
            self.next_token_id.set(U256::from(1));
        }
    }
    
    pub fn name() -> String {
        String::from("Stylus NFT")
    }
    
    pub fn symbol() -> String {
        String::from("SNFT")
    }
    
    pub fn owner_of(&self, token_id: U256) -> Address {
        let owner = self.owners.get(token_id);
        assert!(owner != Address::ZERO, "Token does not exist");
        owner
    }
    
    pub fn balance_of(&self, owner: Address) -> U256 {
        assert!(owner != Address::ZERO, "Invalid owner");
        self.balances.get(owner)
    }
    
    pub fn get_approved(&self, token_id: U256) -> Address {
        assert!(self.exists(token_id), "Token does not exist");
        self.token_approvals.get(token_id)
    }
    
    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        self.operator_approvals.get(owner).get(operator)
    }
    
    pub fn owner(&self) -> Address {
        self.owner.get()
    }
    
    pub fn total_supply(&self) -> U256 {
        let next = self.next_token_id.get();
        if next > U256::ZERO {
            next - U256::from(1)
        } else {
            U256::ZERO
        }
    }
    
    pub fn transfer_from(&mut self, from: Address, to: Address, token_id: U256) -> bool {
        assert!(self.is_approved_or_owner(token_id), "Not authorized");
        self.transfer_impl(from, to, token_id);
        true
    }
    
    pub fn safe_transfer_from(&mut self, from: Address, to: Address, token_id: U256) -> bool {
        self.transfer_from(from, to, token_id)
    }
    
    pub fn approve(&mut self, to: Address, token_id: U256) {
        let owner = self.owner_of(token_id);
        let caller = self.vm().msg_sender();
        
        assert!(
            caller == owner || self.is_approved_for_all(owner, caller),
            "Not authorized"
        );
        
        self.token_approvals.insert(token_id, to);
        
        evm::log(Approval {
            owner,
            approved: to,
            tokenId: token_id,
        });
    }
    
    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
        let caller = self.vm().msg_sender();
        assert!(operator != caller, "Cannot approve self");
        
        self.operator_approvals.setter(caller).insert(operator, approved);
        
        evm::log(ApprovalForAll {
            owner: caller,
            operator,
            approved,
        });
    }
    
    pub fn mint(&mut self, to: Address) -> U256 {
        let caller = self.vm().msg_sender();
        let owner = self.owner.get();
        assert!(caller == owner, "Only owner can mint");
        assert!(to != Address::ZERO, "Invalid recipient");
        
        let token_id = self.next_token_id.get();
        self.next_token_id.set(token_id + U256::from(1));
        
        self.mint_impl(to, token_id);
        
        token_id
    }
    
    pub fn burn(&mut self, token_id: U256) {
        assert!(self.is_approved_or_owner(token_id), "Not authorized");
        
        let owner = self.owners.get(token_id);
        self.token_approvals.insert(token_id, Address::ZERO);
        
        let balance = self.balances.get(owner);
        self.balances.insert(owner, balance - U256::from(1));
        self.owners.insert(token_id, Address::ZERO);
        
        evm::log(Transfer {
            from: owner,
            to: Address::ZERO,
            tokenId: token_id,
        });
    }
}

impl Erc721Nft {
    fn exists(&self, token_id: U256) -> bool {
        self.owners.get(token_id) != Address::ZERO
    }
    
    fn is_approved_or_owner(&self, token_id: U256) -> bool {
        let owner = self.owners.get(token_id);
        if owner == Address::ZERO {
            return false;
        }
        
        let caller = self.vm().msg_sender();
        caller == owner
            || self.token_approvals.get(token_id) == caller
            || self.is_approved_for_all(owner, caller)
    }
    
    fn mint_impl(&mut self, to: Address, token_id: U256) {
        let balance = self.balances.get(to);
        self.balances.insert(to, balance + U256::from(1));
        self.owners.insert(token_id, to);
        
        evm::log(Transfer {
            from: Address::ZERO,
            to,
            tokenId: token_id,
        });
    }
    
    fn transfer_impl(&mut self, from: Address, to: Address, token_id: U256) {
        assert!(to != Address::ZERO, "Invalid recipient");
        assert!(self.owners.get(token_id) == from, "Not token owner");
        
        self.token_approvals.insert(token_id, Address::ZERO);
        
        let from_balance = self.balances.get(from);
        self.balances.insert(from, from_balance - U256::from(1));
        
        let to_balance = self.balances.get(to);
        self.balances.insert(to, to_balance + U256::from(1));
        
        self.owners.insert(token_id, to);
        
        evm::log(Transfer {
            from,
            to,
            tokenId: token_id,
        });
    }
}
