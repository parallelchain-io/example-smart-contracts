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

use smart_contract::{
    contract_init,
    Transaction
};

// The `contract_init` macro is required to convert the smart contract code
// from idiomatic rust to a contract that is readable and executable in
// ParallelChain Mainnet Fullnode.
#[contract_init]
pub fn contract(tx: Transaction<String>) {

    tx.emit_event(
        format!("my_first_contract: START").as_bytes(),
        format!("The smart contract received an argument of: {:?}", tx.arguments).as_bytes()
    );

    tx.set(
    &tx.arguments.as_bytes(),
    &tx.arguments.as_bytes(),
    );

    let value = tx.get(&tx.arguments.as_bytes());
    let v = match value {
        Some(v) => {
            String::from_utf8(v).unwrap()
        },
        None => {
            "".to_string()
        }
    };

    tx.emit_event(
        format!("my_first_contract: SDK get").as_bytes(),
        format!("The obtained value from the world state is: {}", v).as_bytes()
    );
    
    tx.emit_event(
        format!("my_first_contract: END").as_bytes(),
        format!("Completed Call").as_bytes()
    );


    tx.return_value(
        format!("The length of the argument is: {:?}", &tx.arguments.len())
                .as_bytes()
                .to_vec()
    );
    
}