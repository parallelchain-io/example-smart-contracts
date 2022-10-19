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

/// The bank smart contract simulates
/// banking operations with data stored
/// in ParallelChain Mainnet.

use pchain_sdk::{
    contract, contract_methods, action, precompiles
};

mod bank_account;

use bank_account::BankAccount;
/// ### Lesson 1:
/// The macro `contract` on struct allow loading/storing fields from/into world state.
/// The key to be store is u8 integer ordered by the order of the fields. E.g. `num_of_account` has key [0]
#[contract]
struct MyBank {
    
    num_of_account: u64
}

/// ### Lesson 2:
/// The macro `contract` generates entrypoint methods that can be called in transaction
#[contract_methods]
impl MyBank {

    /// entrypoint method "open_account"
    #[action]
    fn open_account(
        first_name: String,
        last_name: String,
        account_id: String,
        initial_deposit: u64,
    ) {
        let parsed_account_id= 
        if account_id != "" {
            account_id.to_owned().as_bytes().to_vec()
        } else {
            // generate a new account id using the base64 encoded sha256 hash
            // of the first and last name concatenated together
            let input = format!("{}{}", &first_name, &last_name).to_string().as_bytes().to_vec();
            precompiles::sha256(input)
        };

        let opened_bank_account = BankAccount {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            account_id:  base64::encode(parsed_account_id),
            amount: initial_deposit,
        };

        bank_account::set_bank_account(
            &opened_bank_account.account_id.as_bytes(),
            &opened_bank_account
        );

        let initial_num_of_account = MyBank::get_num_of_account();
        MyBank::set_num_of_account(initial_num_of_account + 1);

        pchain_sdk::emit_event(
            "bank_account: Open".as_bytes(),
            format!("Successfully opened 
            account for {}, {} 
            with account_id: {}",
            &opened_bank_account.first_name,
            &opened_bank_account.last_name,
            &opened_bank_account.account_id).as_bytes()
        );
    }

    /// entrypoint method "query_account_balance"
    #[action]
    fn query_account_balance(account_id: String) {
        match bank_account::get_bank_account(account_id.as_bytes()) {
            Some(balance) => {
                pchain_sdk::emit_event(
                    format!("bank: query_account_balance").as_bytes(),
                    format!("The current balance is : \nName: {} {}\nAccount Number: {}\nBalance: {}", 
                    &balance.first_name,
                    &balance.last_name,
                    &balance.account_id,
                    // `balance` is an abstract field stored in the world state with the field BankAccount.amount.
                    // Any interaction using the `amount` field to the world state will affect the balance of 
                    // the bank account. 
                    &balance.amount).as_bytes()
                );
            },
            None => {
                pchain_sdk::emit_event(
                    format!("bank: query_account_balance").as_bytes(),
                    format!("No such account found").as_bytes()
                );
            }
        }
    }

    /// entrypoint method "withdraw_money"
    #[action]
    fn withdraw_money(account_id: String, amount_to_withdraw: u64) {
        match bank_account::get_bank_account(account_id.as_bytes()) {
            Some(mut query_result) => {
                match query_result.withdraw_from_balance(amount_to_withdraw) {
                    Some(balance) => {
    
                        // update the world state
                        bank_account::set_bank_account(account_id.as_bytes(), &query_result);
    
                        pchain_sdk::emit_event(
                            format!("bank: withdraw_money").as_bytes(),
                            format!("The updated balance is: \n
                            Name: {} {}\n
                            Account Number: {}\n
                            Balance: {}", 
                            &query_result.first_name,
                            &query_result.last_name,
                            &query_result.account_id,
                            &balance).as_bytes()
                        );
                    }
                    None => pchain_sdk::emit_event(
                        format!("bank: withdraw_money").as_bytes(),
                        format!("You do not have enough funds to withdraw from this account.").as_bytes()
                    ),
                }
            },
            None => pchain_sdk::emit_event(
                format!("bank: withdraw_money").as_bytes(),
                format!("No such account found").as_bytes()
            ),
        };
    }

    /// entrypoint method "deposit_money"
    #[action]
    fn deposit_money(account_id: String, amount_to_deposit: u64) {
        match bank_account::get_bank_account(account_id.as_bytes()) {
            Some(mut query_result) => {
                query_result.deposit_to_balance(amount_to_deposit);
    
                // update the world state
                bank_account::set_bank_account(account_id.as_bytes(), &query_result);
    
                pchain_sdk::emit_event(
                    format!("bank: deposit_money").as_bytes(),
                    format!("The updated balance is: \nName: {} {}\nAccount Number: {}\nBalance: {}", 
                    &query_result.first_name,
                    &query_result.last_name,
                    &query_result.account_id,
                    &query_result.amount).as_bytes()
                );
            },
            None => pchain_sdk::emit_event(
                format!("bank: query_account_balance").as_bytes(),
                format!("No such account found").as_bytes()
            ),
        };
    }
}