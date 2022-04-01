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

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};

// REPLACE ARGUMENT STRUCT WITH YOUR OWN IF ANY
#[sdk_method_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MyLittlePony {
    pub name: String,
    pub age: u32,
    pub gender: Gender,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum Gender {
    Female,
    Male,
}

impl MyLittlePony {
    
    fn new(tx: &Transaction<MyLittlePony>, name: &String, age: u32, gender: &Gender) -> Self {
        let little_pony = MyLittlePony {
            name: name.to_owned(),
            age,
            gender: gender.to_owned(),
        };

        tx.set_my_little_pony(little_pony.name.as_bytes(), &little_pony);

        little_pony

    }

    fn neigh(&self, tx: &Transaction<MyLittlePony>) {

        match tx.get_my_little_pony(self.name.as_bytes()) {
            Some(pony) => {
                let topic: String = "Neigh Message".to_string();
                let value: String = format!("Pony with name {} neighs at age: {}", self.name, pony.age);
                tx.emit_event(topic.as_bytes(), value.as_bytes());
            },
            None => {
                let topic: String = "No Pony".to_string();
                let value: String = format!("Pony not found");
                tx.emit_event(topic.as_bytes(), value.as_bytes());
            }       
        };

    }
}


// The `contract_init` macro is required to convert the smart contract code
// from idiomatic rust to a contract that is readable and executable in
// ParallelChain Mainnet Fullnode.
#[contract_init]
pub fn contract(tx: Transaction<MyLittlePony>) -> Result<String> {
    let pony_specification = &tx.arguments;
    let new_pony = MyLittlePony::new(
        &tx, 
        &pony_specification.name, 
        pony_specification.age, 
        &pony_specification.gender
    );

    Ok(format!("Welcome pony, {}", &new_pony.name))

}