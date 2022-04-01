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


pub fn withdraw_money(tx: &Transaction<BankAccount>, account_id: &String, amount_to_withdraw: Option<u64>) {
    
    let parsed_amount_to_withdraw = match amount_to_withdraw {
        Some(a) => a,
        None => 0,
    };

    match tx.get_bank_account(account_id.as_bytes()) {
        Some(mut query_result) => {
            match query_result.withdraw_from_balance(parsed_amount_to_withdraw) {
                Some(balance) => {

                    // update the world state
                    tx.set_bank_account(account_id.as_bytes(), &query_result);

                    tx.emit_event(
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
                None => tx.emit_event(
                    format!("bank: withdraw_money").as_bytes(),
                    format!("You do not have enough funds to withdraw from this account.").as_bytes()
                ),
            }
        },
        None => tx.emit_event(
            format!("bank: withdraw_money").as_bytes(),
            format!("No such account found").as_bytes()
        ),
    };
}
