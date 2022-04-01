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
    Transaction,
    contract_init,
};


use anyhow::Result;
use borsh::BorshSerialize;

pub mod bank_account;
use bank_account::{
    BankAccount,
    Operation,
};

pub mod bank_operations;
use self::bank_operations::{
    deposit_money,
    query_account_balance,
    withdraw_money,
};

// The `contract_init` macro is required to convert the smart contract code
// from idiomatic rust to a contract that is readable and executable in
// ParallelChain Mainnet Fullnode.
#[contract_init]
pub fn contract(tx: Transaction<BankAccount>) -> Result<()> {
    // need to change this reference later
    let customer = &tx.arguments;

    let first_name = &customer.first_name;
    let last_name = &customer.last_name;
    let account_id = &customer.account_id;

    match customer.operation {
        Operation::Open => {

            BankAccount::open_account(
                &tx,
                &first_name,
                &last_name,
                &account_id,
                customer.amount,
            ).unwrap();
            Ok(())
        },
        Operation::Query => {
            query_account_balance(&tx, &account_id);
            Ok(())
        },
        Operation::Deposit => {
            deposit_money(&tx, &account_id, customer.amount);
            Ok(())
        },
        Operation::Withdraw => {
            withdraw_money(&tx, &account_id, customer.amount);
            Ok(())
        },
    };
    
}