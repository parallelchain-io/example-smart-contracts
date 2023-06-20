# Example ParallelChain F Smart Contracts

## Chapter 1: HelloContract

HelloContract goes through the steps of creating a simple smart contract with the use of `pchain_sdk` macros, including `contract`, `contract_methods`, and `call`.


## Chapter 2: MyLittlePony

MyLittlePony demonstrates how we define entrypoint methods and fields as data in contract storage. It goes through how a contract struct can be defined, as well as the use of `contract_field` macros. It also introduces different ways of getting and setting data in contract storage.

## Chapter 3: MyBank

MyBank simulates banking operations with data stored in ParallelChain Mainnet. We combine the knowledge from the previous chapters and provide the basic functionalities of a bank.

## Chapter 4: ContractProxy

ContractProxy introduces the use of the `use_contract` macro to interact with other contracts. We make use of the MyLittlePony smart contract from Chapter 2 to demonstrate the interaction between two contracts.

## Chapter 5: MyCollections

MyCollections illustrates the functionalities of collections provided by `pchain_sdk`. The collection structures are designed for gas efficiency, they include `Cacher`, `Vector`, `FastMap`, and `IterableMap`.

## Chapter 6: MyPool

MyPool shows how network commands are used in smart contracts. It demonstrates how they can be created in smart contracts and explains their characteristics.