/*
 Copyright 2022 ParallelChain Lab

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 */

use borsh::{BorshDeserialize, BorshSerialize};

use pchain_sdk::{
    storage,
};
// An example of a data struct using the `sdk_method_bindgen` macro provided 
// by ParallelChain Mainnet Smart contract SDK.
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
