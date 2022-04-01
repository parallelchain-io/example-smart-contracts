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
    sdk_method_bindgen,
    Transaction
};

use borsh::{BorshDeserialize, BorshSerialize};

// REPLACE ARGUMENT STRUCT WITH YOUR OWN IF ANY
#[sdk_method_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
struct MyStruct {
    // SETUP CONTRACT ARGUMENT
}

// The `contract_init` macro is required to convert the smart contract code
// from idiomatic rust to a contract that is readable and executable in
// ParallelChain Mainnet Fullnode.
#[contract_init]
// REPLACE `A` WITH RUST PRIMITIVE TYPES OR A CUSTOM DATA TYPE SUCH AS `MyStruct` 
pub fn contract(tx: Transaction<A>) {

    // CALL SDK METHODS HERE

    // WRITE CONTRACT STATE
    
    // ADD RESULT RETURN IF REQUIRED
}