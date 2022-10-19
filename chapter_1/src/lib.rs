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

/// This is an example of smart contract as programming model.
/// 
/// 

use pchain_sdk::{
    contract, contract_methods, action, init, view, storage, emit_event, 
};

#[contract]
struct HelloContract {}

#[contract_methods]
impl HelloContract {

    #[init]
    fn new(hello_msg : String) {
        pchain_sdk::emit_event(
            "topic: Init".as_bytes(), 
            format!("{}", hello_msg).as_bytes()
        );
    }

    #[action]
    fn hello() {
        pchain_sdk::emit_event(
            "topic: Hello".as_bytes(), 
            "Hello, Contract".as_bytes()
        );
    }

    #[action]
    fn hello_from(name :String) -> u32 {
        pchain_sdk::emit_event(
            "topic: Hello From".as_bytes(), 
            format!("Hello, Contract. From: {}", name).as_bytes()
        );
        name.len() as u32
    }

    #[action]
    fn hello_set_many() {
        for i in 1..10{
            let key = format!("hello-key-{}", i);
            let value = vec![0_u8; 1024*10]; //10KB
            storage::set(key.as_bytes(), &value);
        }
    }

    #[action]
    fn hello_read_many() {
        for i in 1..10{
            let key = format!("hello-key-{}", i);
            let value = storage::get(key.as_bytes());
            if value.is_some(){
                emit_event(
                    "topic: Hello read".as_bytes(), 
                    format!("key: {}, len: {}", key, value.unwrap().len()).as_bytes()
                );
            }
        }
    }

    #[view]
    fn i_say_hello() -> String {
        "you say world!".to_string()
    }
}