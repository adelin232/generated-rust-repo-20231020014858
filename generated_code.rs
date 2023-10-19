#![no_std]

use multiversx_sc::*;

mod token_storage;

use token_storage::*;

#[derive(Debug, Clone, MultiversX)]
pub struct MultiversXToken {
    /* Events */
    #[event("0x1234567890123456789012345678901234567890123456789012345678900001")]
    pub ContractCreated: {},
    #[event("0x1234567845123456789012345678901234567890123456789012345678900001")]
    pub Transfer: {
        from: Address,
        to: Address,
        amount: BigInt
    },
    /* Storage Contract */
    pub token: TokenStorage,
}

impl MultiversXToken {
    #[constructor]
    pub fn new(&mut self) -> Self {
        self.emit_ContractCreated();
        self
    }
    
    // Mint new tokens
    pub fn mint(&mut self, account: Address, amount: BigInt) {
        self.token.mint(account, amount);
    }

    // Transfer tokens from one account to another
    pub fn transfer(&mut self, from: Address, to: Address, amount: BigInt) {
        self.token.transfer(from, to, amount);
        self.emit_Transfer(from, to, amount);
    }
}