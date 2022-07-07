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

/// The bank smart contract simulates
/// banking operations with data stored
/// in ParallelChain Mainnet.

use smart_contract::{
    contract, action, precompile
};

mod bank_account;

use bank_account::{BankAccount, sdk_typed_BankAccount};
/// ### Lesson 1:
/// The macro `contract` on struct allow loading/storing fields from/into world state.
/// The key to be store is u8 integer ordered by the order of the fields. E.g. `num_of_account` has key [0]
#[contract]
struct MyBank {
    num_of_account: u64
}

/// ### Lesson 2:
/// The macro `contract` generates entrypoint methods that can be called in transaction
#[contract]
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
            precompile::sha256(input)
        };

        let opened_bank_account = BankAccount {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            account_id:  base64::encode(parsed_account_id),
            amount: initial_deposit,
        };

        let tx = Transaction::new();
        tx.set_bank_account(
            &opened_bank_account.account_id.as_bytes(),
            &opened_bank_account
        );

        let initial_num_of_account = MyBank::get_num_of_account();
        MyBank::set_num_of_account(initial_num_of_account + 1);

        Transaction::emit_event(
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
        let tx = Transaction::new();
        match tx.get_bank_account(account_id.as_bytes()) {
            Some(balance) => {
                Transaction::emit_event(
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
                Transaction::emit_event(
                    format!("bank: query_account_balance").as_bytes(),
                    format!("No such account found").as_bytes()
                );
            }
        }
    }

    /// entrypoint method "withdraw_money"
    #[action]
    fn withdraw_money(account_id: String, amount_to_withdraw: u64) {
        let tx = Transaction::new();
        match tx.get_bank_account(account_id.as_bytes()) {
            Some(mut query_result) => {
                match query_result.withdraw_from_balance(amount_to_withdraw) {
                    Some(balance) => {
    
                        // update the world state
                        tx.set_bank_account(account_id.as_bytes(), &query_result);
    
                        Transaction::emit_event(
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
                    None => Transaction::emit_event(
                        format!("bank: withdraw_money").as_bytes(),
                        format!("You do not have enough funds to withdraw from this account.").as_bytes()
                    ),
                }
            },
            None => Transaction::emit_event(
                format!("bank: withdraw_money").as_bytes(),
                format!("No such account found").as_bytes()
            ),
        };
    }

    /// entrypoint method "deposit_money"
    #[action]
    fn deposit_money(account_id: String, amount_to_deposit: u64) {
        let tx = Transaction::new();
        match tx.get_bank_account(account_id.as_bytes()) {
            Some(mut query_result) => {
                query_result.deposit_to_balance(amount_to_deposit);
    
                // update the world state
                tx.set_bank_account(account_id.as_bytes(), &query_result);
    
                Transaction::emit_event(
                    format!("bank: deposit_money").as_bytes(),
                    format!("The updated balance is: \nName: {} {}\nAccount Number: {}\nBalance: {}", 
                    &query_result.first_name,
                    &query_result.last_name,
                    &query_result.account_id,
                    &query_result.amount).as_bytes()
                );
            },
            None => Transaction::emit_event(
                format!("bank: query_account_balance").as_bytes(),
                format!("No such account found").as_bytes()
            ),
        };
    }
}