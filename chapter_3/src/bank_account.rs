use borsh::{BorshDeserialize, BorshSerialize};

use pchain_sdk::{
    storage,
};
// An example of a data struct using the `sdk_method_bindgen` macro provided 
// by ParallelChain Mainnet Smart Contract SDK.
//
// Note that both the serializer and deserializer macros such as Borsh need to 
// be applied to this struct for it to work. See the ParallelChain Mainnet 
// documentation smart_contract_macros for more information.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BankAccount {
    pub first_name: String,
    pub last_name: String,
    pub account_id: String,
    pub amount: u64,
}
pub fn get_bank_account(key: &[u8]) -> Option<BankAccount> {
    match storage::get(key) {
        Some(raw_result) => {
            let p: Option<BankAccount> =
                match BorshDeserialize::deserialize(&mut raw_result.as_ref()) {
                    Ok(d) => Some(d),
                    Err(_) => None,
                };
            p
        }
        None => None,
    }
}
pub fn set_bank_account(key: &[u8], value: &BankAccount) {
    let mut buffer: Vec<u8> = Vec::new();
    value.serialize(&mut buffer).unwrap();
    storage::set(key, buffer.as_ref());
}

impl BankAccount {
    pub fn deposit_to_balance(&mut self, amount_to_add: u64) {
        self.amount += amount_to_add;
    }
    pub fn withdraw_from_balance(&mut self, amount_to_withdraw: u64) -> Option<u64> {
        if amount_to_withdraw <= self.amount {
            self.amount -= amount_to_withdraw;
            Some(self.amount)
        } else {
            None
        }
    }
}
