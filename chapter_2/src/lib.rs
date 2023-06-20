use pchain_sdk::{
    contract, contract_methods, call, contract_field
};

// MyLittlePony is a contract to demonstrate how contract can:
// - define entrypoint methods
// - define fields as data in contract storage

/// ### Section 1:
/// The macro `contract` on struct allow loading/storing fields from/into world state.
/// The key to be store is u8 integer ordered by the order of the fields. E.g. `name` has key [0] while `age` has key [1]
#[contract]
pub struct MyLittlePony {
    name: String,
    age: u32,
    gender: Gender,
}

/// ### Section 2:
/// The contract field can be used in contract struct so that the key-value pair can be accessed in canonical format.
/// For example, `name` in Gender has a key [2][0] for contract `MyLittlePony`.
#[contract_field]
struct Gender {
    name: String,
    description: String
}

#[contract_methods]
impl MyLittlePony {
    
    /// ### Section 3: 
    /// Use receiver `&self` to load all data before executing this method. 
    /// All data will be loaded to receiver self from world state.
    #[call]
    fn self_introduction(&self) -> String {
        format!("Hi, I am {}. Age of {}. I am {} that means {}.",
            self.name, self.age, self.gender.name, self.gender.description
        )
    }

    /// ### Section 4:
    /// This method use contract getter and setter to store the updated data to field `data` to world state. 
    /// Write cost is small because there is only one key-value pair in world state to be mutated.
    #[call]
    fn grow_up() {
        let age = Self::get_age();
        Self::set_age(age+1)
    }

    /// ### Section 5:
    /// Use mutable receiver `&mut self` to load data before executing this method, and then store all data after execution.
    /// Be cautious to use mutable receiver as it is expansive to load and storte all key-value pairs in world state
    #[call]
    fn change_person(&mut self, name: String, age: u32, gender_name: String, description: String) {
        pchain_sdk::log(
            "update_gender".to_string().as_bytes(), 
            format!("update name:{} description: {}", name, description).as_bytes());
        self.name = name;
        self.age = age;
        self.gender.name = gender_name;
        self.gender.description = description;
    }
}