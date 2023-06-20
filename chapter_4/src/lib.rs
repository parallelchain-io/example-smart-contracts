use pchain_sdk::{
    use_contract, call, contract, contract_methods
};

// Contract Proxy serves as middle-man to another contract `MyLittlePony`.
// This example shows how contract can interact with other contract by:
// - calling entrypoint methods
// - sending tokens from balance of this contract to other contract


/// ### Section 1:
/// use macro `use_contract` to specify the contract action entrypoint methods in a trait.
/// The address is hard-coded when using macro `use_contract`, ie. once you deployed this
/// contract, the address cannot be changed.
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

    /// ### Section 2:
    /// The trait will be transformed to mod by using macro `use_contract`. 
    /// Calling the contract `MyLittlePony` can be simply calling associate methods according to defined method in the trait.
    /// Value and Gas will be needed in cross contract call
    #[call]
    fn grow_up() {
        my_little_pony::grow_up(0);
    }

    /// ### Section 3:
    /// It is also possible to use call_untyped() instead of macro `use_contract` to make a cross contract call.
    /// Address can also be passed as argument so that contract address is not necessary hard-coded.
    #[call]
    fn grow_up_2() {
        let contract_address = base64url::decode("-jUt6jrEfMRD1JM9n6_yAASl2cwsc4tg1Bqp07gvQpU").unwrap().try_into().unwrap();
        pchain_sdk::call_untyped(
            contract_address,
            "grow_up", 
            Vec::new(),
            0);
    }

    /// ### Section 4:
    /// use method transfer() to send tokens from this contract balance to specific address.
    #[call]
    fn send_tokens(value :u64){
        let contract_address = base64url::decode("-jUt6jrEfMRD1JM9n6_yAASl2cwsc4tg1Bqp07gvQpU").unwrap().try_into().unwrap();
        pchain_sdk::transfer(
            contract_address,
            value
        );
    }
}