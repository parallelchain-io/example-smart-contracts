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
    contract, init, action, view,
    ContractField,
};

// MyLittlePoiny is a contract to demonstrate how contract can:
// - define init, action and view entrypoint methods
// - define fields as data in contract storage
// - create contract metadata

/// ### Lesson 1:
/// The macro `contract` on struct allow loading/storing fields from/into world state.
/// The key to be store is u8 integer ordered by the order of the fields. E.g. `name` has key [0] while `age` has key [1]
#[contract]
pub struct MyLittlePony {
    name: String,
    age: u32,
    gender: Gender,
}

/// ### Lesson 2:
/// The contract field can be used in contract struct so that the key-value pair can be accessed in canonical format.
/// For example, `name` in Gender has a key [2][0] for contract `MyLittlePony`.
#[derive(ContractField)]
struct Gender {
    name: String,
    description: String
}

/// ### Lesson 3:
/// When attribute `meta` is included in the macro contract, contract metadata is created to allow proving information for cross contract call.
/// The metadata is a str slice that is also rust source code representing a trait. Developer can directly include this trait in cross contract call. 
/// Please note the trait is only applicable to action entrypoint methods
#[contract(meta)]
impl MyLittlePony {
    
    /// ### Lesson 4:
    /// This method is `init` method that will be execution during contract deployment process
    #[init]
    fn new(name: String, age: u32 ) {
        Transaction::emit_event(
            "Init Contract".to_string().as_bytes(),
            format!("{} at age{} was born.", name, age).as_bytes()
        );
        MyLittlePony {
            name,
            age,
            gender: Gender { 
                name: String::default(), 
                description: String::default()
            }
        }.set(); // this setter applies to all fields in whole struct
    }

    /// ### Lesson 5: 
    /// Use receiver `&self` to load all data before executing this method. 
    /// All data will be loaded to receiver self from world state.
    #[action] 
    fn self_introduction(&self) -> String {
        format!("Hi, I am {}. Age of {}. I am {} that means {}.",
            self.name, self.age, self.gender.name, self.gender.description
        ).to_string()
    }

    /// ### Lesson 6:
    /// This method use contract getter and setter to store the updated data to field `data` to world state. 
    /// Write cost is small because there is only one key-value pair in world state to be mutated.
    #[action]
    fn grow_up() {
        let age = Self::get_age();
        Self::set_age(age+1)
    }

    /// ### Lesson 7:
    /// Use mutable receiver `&mut self` to load data before executing this method, and then store all data after execution.
    /// Be cautious to use mutable receiver as it is expansive to load and storte all key-value pairs in world state
    #[action]
    fn change_person(&mut self, name: String, age: u32, gender_name: String, description: String) {
        Transaction::emit_event(
            "update_gender".to_string().as_bytes(), 
            format!("update name:{} description: {}", name, description).as_bytes());
        self.name = name;
        self.age = age;
        self.gender.name = gender_name;
        self.gender.description = description;
    }

    /// ### Lesson 8:
    /// View entrypoint method provides cost-free execution of a contract.
    /// View methods are limited by allowing only execution of getting world-state data.
    #[view]
    fn age() -> u32 {
        Self::get_age()
        // This will cause panic:
        // Self::set_name("my name".to_string())
    }
}