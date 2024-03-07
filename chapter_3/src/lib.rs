/// The bank smart contract simulates
/// banking operations with data stored
/// in ParallelChain Mainnet.

use pchain_sdk::{
    contract, contract_methods, call, crypto
};

mod bank_account;

use bank_account::BankAccount;
/// ### Section 1:
/// The macro `contract` on struct allows loading/storing fields from/into world state.
/// The key to be stored is u8 integer ordered by the index of the fields. E.g. `num_of_account` has key [0]
#[contract]
struct MyBank {
    num_of_account: u64
}

/// ### Section 2:
/// The macro `contract` generates entrypoint methods that can be called in transaction
#[contract_methods]
impl MyBank {

    /// entrypoint method "open_account"
    #[call]
    fn open_account(
        first_name: String,
        last_name: String,
        account_id: String,
        initial_deposit: u64,
    ) {
        let parsed_account_id= 
        if !account_id.is_empty() {
            account_id.as_bytes().to_vec()
        } else {
            // generate a new account id using the base64 encoded sha256 hash
            // of the first and last name concatenated together
            crypto::sha256(format!("{}{}", &first_name, &last_name).as_bytes().to_vec())
        };

        let opened_bank_account = BankAccount {
            first_name,
            last_name,
            account_id:  base64::encode(parsed_account_id),
            amount: initial_deposit,
        };

        bank_account::set_bank_account(
            &opened_bank_account.account_id.as_bytes(),
            &opened_bank_account
        );

        MyBank::set_num_of_account(MyBank::get_num_of_account() + 1);

        pchain_sdk::log(
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
    #[call]
    fn query_account_balance(account_id: String) {
        match bank_account::get_bank_account(account_id.as_bytes()) {
            Some(balance) => {
                pchain_sdk::log(
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
                pchain_sdk::log(
                    format!("bank: query_account_balance").as_bytes(),
                    format!("No such account found").as_bytes()
                );
            }
        }
    }

    /// entrypoint method "withdraw_money"
    #[call]
    fn withdraw_money(account_id: String, amount_to_withdraw: u64) {
        match bank_account::get_bank_account(account_id.as_bytes()) {
            Some(mut query_result) => {
                match query_result.withdraw_from_balance(amount_to_withdraw) {
                    Some(balance) => {
                        // update the world state
                        bank_account::set_bank_account(account_id.as_bytes(), &query_result);
    
                        pchain_sdk::log(
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
                    None => pchain_sdk::log(
                        format!("bank: withdraw_money").as_bytes(),
                        format!("You do not have enough funds to withdraw from this account.").as_bytes()
                    ),
                }
            },
            None => pchain_sdk::log(
                format!("bank: withdraw_money").as_bytes(),
                format!("No such account found").as_bytes()
            ),
        };
    }

    /// entrypoint method "deposit_money"
    #[call]
    fn deposit_money(account_id: String, amount_to_deposit: u64) {
        match bank_account::get_bank_account(account_id.as_bytes()) {
            Some(mut query_result) => {
                query_result.deposit_to_balance(amount_to_deposit);
    
                // update the world state
                bank_account::set_bank_account(account_id.as_bytes(), &query_result);
    
                pchain_sdk::log(
                    format!("bank: deposit_money").as_bytes(),
                    format!("The updated balance is: \nName: {} {}\nAccount Number: {}\nBalance: {}", 
                    &query_result.first_name,
                    &query_result.last_name,
                    &query_result.account_id,
                    &query_result.amount).as_bytes()
                );
            },
            None => pchain_sdk::log(
                format!("bank: query_account_balance").as_bytes(),
                format!("No such account found").as_bytes()
            ),
        };
    }
}
