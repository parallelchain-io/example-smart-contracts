/*
 Copyright (c) 2022 ParallelChain Lab

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */



use borsh::{BorshDeserialize, BorshSerialize};

use smart_contract::{
    Transaction,
    sdk_method_bindgen,
};
// An example of a data struct using the `sdk_method_bindgen` macro provided 
// by ParallelChain Mainnet Smart contract SDK.
//
// Note that both the serializer and deserializer macros such as Borsh need to 
// be applied to this struct for it to work. See the ParallelChain Mainnet 
// documentation smart_contract_macros for more information.
#[derive(BorshSerialize, BorshDeserialize)]
#[sdk_method_bindgen]
pub struct BankAccount {
    pub first_name: String,
    pub last_name: String,
    pub account_id: String,
    pub amount: u64
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