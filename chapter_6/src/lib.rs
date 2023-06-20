//! my_pool demonstrates the usage of network commands in smart contract.

use pchain_sdk::{
    call, contract, contract_methods
};

// The six different network commands includes:
// - Create deposit
// - Set deposit settings
// - Top up deposit
// - Withdraw deposit
// - Stake deposit
// - Unstake deposit

// In order to demonstrate how the above network commands can be created and sent 
// to the network through the use of smart contract, the contract MyPool will 
// guide you through the steps of creating a stake in a pool.

type Address = [u8;32];

#[contract]
pub struct MyPool {
    pool_operator: Address,
    my_friend: Address
}

#[contract_methods]
impl MyPool {

    #[call]
    fn init(pool_operator: Address, my_friend: Address){
        MyPool { pool_operator, my_friend }.set();
    } 

    /// ### Section 1:
    /// The network commands are "deferred" because the actual execution of such 
    /// command occurs after the execution of a successful call. 
    /// 
    /// The deposit is created on behalf of the contract address, so make sure 
    /// to transfer some balance to the contract for the operation.
    /// 
    /// You can do the following using pchain_client to check if the deposit
    /// is successfully created: 
    /// ./pchain_client query deposit --operator <OPERATOR_ADDRESS> --owner <CONTRACT_ADDRESS>

    #[call]
    fn create_deposit(balance: u64, auto_stake_rewards: bool) {
        pchain_sdk::network::defer_create_deposit(Self::get_pool_operator(), balance, auto_stake_rewards)
    }

    /// ### Section 1:
    /// In the case of failed call, the deferred network command call will not 
    /// take place. In here, we are making the transaction fail deliberately by transferring
    /// more than what we have, therefore the stake should not be deposited.
    /// 
    /// Check the deposit again using pchain_client, the deposit balance in the pool does not change
    #[call]
    fn transfer_too_much() {
        let balance = pchain_sdk::blockchain::balance();
        pchain_sdk::transfer(Self::get_my_friend(), balance + 1);
        pchain_sdk::network::defer_stake_deposit(Self::get_pool_operator(), balance);
    }

    /// ### Section 2:
    /// The return value in the transaction receipt will be overwritten by the
    /// deferred staking command
    /// 
    /// After executing this transaction, we are expecting the return_values to 
    /// be returning the balance of the contract. However, the return_values will
    /// be overwritten by the return value from the staking command.
    #[call]
    fn stake_deposit(max_amount: u64) -> u64{
        pchain_sdk::network::defer_stake_deposit(Self::get_pool_operator(), max_amount);
        pchain_sdk::blockchain::balance()
    }

    /// ### Section 3:
    /// Multiple network commands can be placed in one transactions. We put the
    /// commands for unstaking and withdrawing deposit in the same function.
    /// 
    /// Both the commands will be executed after the success of the transaction, 
    /// and according to the order that they were called in the function. After
    /// executing this transaction, the stake of the deposit should have reduced. 
    #[call]
    fn multiple_defer(max_amount: u64) {
        let operator = Self::get_pool_operator();
        pchain_sdk::network::defer_unstake_deposit(operator, max_amount);
        pchain_sdk::network::defer_withdraw_deposit(operator, max_amount);
    }


}