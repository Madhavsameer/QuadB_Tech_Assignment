// src/token_wallet.rs
use ic_cdk::export::candid::{CandidType};
use serde::{Serialize, Deserialize};
use std::cell::RefCell;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct Token {
    pub amount: u64,
}

pub struct Wallet {
    balance: RefCell<u64>,
}

impl Wallet {
    // Initialize a new wallet
    pub fn new() -> Self {
        Wallet {
            balance: RefCell::new(0),
        }
    }

    // Function to send tokens
    pub fn send_tokens(&self, amount: u64, recipient: &Wallet) -> Result<(), String> {
        let current_balance = *self.balance.borrow();
        if current_balance < amount {
            return Err("Insufficient funds".to_string());
        }
        *self.balance.borrow_mut() -= amount;
        recipient.receive_tokens(amount);
        Ok(())
    }

    // Function to receive tokens
    pub fn receive_tokens(&self, amount: u64) {
        *self.balance.borrow_mut() += amount;
    }

    // Function to check the balance
    pub fn get_balance(&self) -> u64 {
        *self.balance.borrow()
    }
}
