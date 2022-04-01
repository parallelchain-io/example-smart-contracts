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



use anyhow::Result;
use base64;
use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Sha256, Digest};

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
    pub amount: Option<u64>,
    pub operation: Operation,
}

impl BankAccount {

    pub fn open_account(
        tx: &Transaction<BankAccount>,
        first_name: &String, 
        last_name: &String, 
        account_id: &String,
        initial_deposit: Option<u64>,
    ) -> Result<Self> {
        

        // check existence of amount
        // if user did not supply aa amount, then it is assumed to be 0
        let parsed_initial_deposit = match initial_deposit {
            Some(s) => s,
            None => 0
        };

        // check existence of account number
        // if user did not supply a custom account id (can be anything as long as it is String)
        // the user can supply "" in the account_id field and a new account_id 
        // will be generated for the user.
        let parsed_account_id= if account_id != "" {
            account_id.to_owned()
        } else {
            // generate a new account id using the base64 encoded sha256 hash
            // of the first and last name concatenated together
      
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", &first_name, &last_name).as_bytes());

            base64::encode(hasher.finalize())
 
        };


        let opened_bank_account = BankAccount {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            account_id: parsed_account_id,
            amount: Some(parsed_initial_deposit),
            operation: Operation::Open,
        };

        tx.set_bank_account(
            &opened_bank_account.account_id.as_bytes(),
            &opened_bank_account
        );

        tx.emit_event(
            format!("bank_account: {:?}", &opened_bank_account.operation).as_bytes(),
            format!("Successfully opened 
            account for {}, {} 
            with account_id: {}",
            &opened_bank_account.first_name,
            &opened_bank_account.last_name,
            &opened_bank_account.account_id).as_bytes()
        );

        Ok(opened_bank_account)
    }

    pub fn deposit_to_balance(&mut self, amount_to_add: u64) {
        self.amount = Some(self.amount.unwrap() + amount_to_add);
        self.operation = Operation::Deposit;
    }

    pub fn withdraw_from_balance(&mut self, amount_to_withdraw: u64) -> Option<u64> {
        if amount_to_withdraw <= self.amount.unwrap() {
           self.amount = Some(self.amount.unwrap() - amount_to_withdraw);
           self.operation = Operation::Withdraw;
           self.amount
        } else {
           None
        } 
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum Operation {
    Open,
    Query,
    Withdraw,
    Deposit,
}
