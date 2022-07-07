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
    use_contract, contract, action,
};

// Contract Proxy serves as middle-man to another contract `MyLittlePony`.
// This example shows how contract can interact with other contract by:
// - call action and view entrypoint method
// - send tokens from balance of this contract to other contract


/// ### Lesson 1:
/// use macro `use_contract` to specify the contract action entrypoint methods in a trait.
/// The address is hard-coded when using macro `use_contract`.
/// It is recommended to remove/comment out the methods that are not intended to be used.
#[use_contract("1gHmqonrY2MruxfqKPE80FerpcuCFdHBV4lfx_CMuEc")]
pub trait MyLittlePony {
    //fn self_introduction() -> String;
    fn grow_up();
    //fn change_person(name :String, age :u32, gender_name :String, description :String);
}

#[contract]
pub struct ContractProxy {}

#[contract]
impl ContractProxy {

    /// ### Lesson 2:
    /// The trait will be transformed to mod by using macro `use_contract`. 
    /// Calling the contract `MyLittlePony` can be simply calling associate methods according to defined method in the trait.
    /// Value and Gas will be needed in cross contract call
    #[action]
    fn grow_up() {
        my_little_pony::grow_up(0, 120000);
    }

    /// ### Lesson 3:
    /// It is also possible to use call_contract() instead of macro `use_contract` to make a cross contract call.
    /// Address can also be passed as argument so that contract address is not necessary hard-coded.
    #[action]
    fn grow_up_2() {
        Transaction::call_contract(
            smart_contract::decode_contract_address("1gHmqonrY2MruxfqKPE80FerpcuCFdHBV4lfx_CMuEc".to_string()),
            "grow_up", 
            Vec::new(),
            0, 120000);
    }

    /// ### Lesson 4:
    /// use method pay() to send tokens from this contract balance to specific address.
    #[action]
    fn send_tokens(value :u64){
        Transaction::pay(
            smart_contract::decode_contract_address("1gHmqonrY2MruxfqKPE80FerpcuCFdHBV4lfx_CMuEc".to_string()),
            value
        );
    }

    /// ### Lesson 5:
    /// use method view_contract() to access view entrypoint methods from specific contract address
    #[action]
    fn is_adult() -> bool {
        if let Some(age) = Transaction::view_contract(
            smart_contract::decode_contract_address("1gHmqonrY2MruxfqKPE80FerpcuCFdHBV4lfx_CMuEc".to_string()),
            "age",
            Vec::new()) {
            if let Some(age) = smart_contract::convert_from::<u32>(&age) {
                return age > 18;
            }
        }
        return false;
    }
}