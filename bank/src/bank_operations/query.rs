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


use crate::{
    Transaction,
    bank_account::*,
};

pub fn query_account_balance(tx: &Transaction<BankAccount>, account_id: &String) {
    
    match tx.get_bank_account(account_id.as_bytes()) {
        Some(ba) => {

            // check if the `amount` field is set to None
            if let Some(_) = ba.amount {
                tx.emit_event(
                        format!("bank: query_account_balance").as_bytes(),
                        format!("WARNING: Operation::Query does not require the 
                                    \"amount\" field in BankAccount to contain a value. 
                                    Please set the amount to None for this query.")
                                    .as_bytes()
                );
            }
               
            tx.emit_event(
                format!("bank: query_account_balance").as_bytes(),
                format!("The current balance is : \n
                Name: {} {}\n
                Account Number: {}\n
                Balance: {}", 
                &ba.first_name,
                &ba.last_name,
                &ba.account_id,
                // `balance` is an abstract field stored in the world state with the field BankAccount.amount.
                // Any interaction using the `amount` field to the world state will affect the balance of 
                // the bank account. 
                &ba.amount.unwrap()).as_bytes()
            );

        },
        None => tx.emit_event(
            format!("bank: query_account_balance").as_bytes(),
            format!("No such account found").as_bytes()
        ),
    }
}