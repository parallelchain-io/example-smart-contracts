/*
 Copyright 2022 ParallelChain Lab

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 */

use pchain_sdk::{
    use_contract, contract, contract_methods, action
};

// Contract Proxy serves as middle-man to another contract `MyLittlePony`.
// This example shows how contract can interact with other contract by:
// - call action and view entrypoint method
// - send tokens from balance of this contract to other contract


/// ### Lesson 1:
/// use macro `use_contract` to specify the contract action entrypoint methods in a trait.
/// The address is hard-coded when using macro `use_contract`.
/// It is recommended to remove/comment out the methods that are not intended to be used.
#[use_contract("-jUt6jrEfMRD1JM9n6_yAASl2cwsc4tg1Bqp07gvQpU")]
pub trait MyLittlePony {
    //fn self_introduction() -> String;
    fn grow_up();
    //fn change_person(name :String, age :u32, gender_name :String, description :String);
}

#[contract]
pub struct ContractProxy {}

#[contract_methods]
impl ContractProxy {

    /// ### Lesson 2:
    /// The trait will be transformed to mod by using macro `use_contract`. 
    /// Calling the contract `MyLittlePony` can be simply calling associate methods according to defined method in the trait.
    /// Value and Gas will be needed in cross contract call
    #[action]
    fn grow_up() {
        my_little_pony::grow_up(0);
    }

    /// ### Lesson 3:
    /// It is also possible to use call_action_untyped() instead of macro `use_contract` to make a cross contract call.
    /// Address can also be passed as argument so that contract address is not necessary hard-coded.
    #[action]
    fn grow_up_2() {
        let contract_address = pchain_types::Base64URL::decode("-jUt6jrEfMRD1JM9n6_yAASl2cwsc4tg1Bqp07gvQpU").unwrap().try_into().unwrap();
        pchain_sdk::call_action_untyped(
            contract_address,
            "grow_up", 
            Vec::new(),
            0);
    }

    /// ### Lesson 4:
    /// use method pay() to send tokens from this contract balance to specific address.
    #[action]
    fn send_tokens(value :u64){
        let contract_address = pchain_types::Base64URL::decode("-jUt6jrEfMRD1JM9n6_yAASl2cwsc4tg1Bqp07gvQpU").unwrap().try_into().unwrap();
        pchain_sdk::pay(
            contract_address,
            value
        );
    }

    /// ### Lesson 5:
    /// use method view_contract() to access view entrypoint methods from specific contract address
    #[action]
    fn is_adult() -> bool {
        let contract_address = pchain_types::Base64URL::decode("-jUt6jrEfMRD1JM9n6_yAASl2cwsc4tg1Bqp07gvQpU").unwrap().try_into().unwrap();
        if let Some(age) = pchain_sdk::call_view::<u32>(
            contract_address,
            "age",
            Vec::new()) {
            return age > 18;
        }
        return false;
    }
}