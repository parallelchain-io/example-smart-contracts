/// This is an example of smart contract as programming model.
/// 
/// 

use pchain_sdk::{
    contract, contract_methods, call, storage, log, 
};

#[contract]
struct HelloContract {}

#[contract_methods]
impl HelloContract {

    #[call]
    fn hello() {
        pchain_sdk::log(
            "topic: Hello".as_bytes(), 
            "Hello, Contract".as_bytes()
        );
    }

    #[call]
    fn hello_from(name :String) -> u32 {
        pchain_sdk::log(
            "topic: Hello From".as_bytes(), 
            format!("Hello, Contract. From: {}", name).as_bytes()
        );
        name.len() as u32
    }

    #[call]
    fn hello_set_many() {
        for i in 1..10 {
            let key = format!("hello-key-{}", i);
            let value = vec![0_u8; 1024*10]; //10KB
            storage::set(key.as_bytes(), &value);
        }
    }

    #[call]
    fn hello_read_many() {
        for i in 1..10 {
            let key = format!("hello-key-{}", i);
            if let Some(value) = storage::get(key.as_bytes()) {
                log(
                    "topic: Hello read".as_bytes(), 
                    format!("key: {}, len: {}", key, value.len()).as_bytes()
                );
            }
        }
    }

    #[call]
    fn i_say_hello() -> String {
        "you say world!".to_string()
    }
}
