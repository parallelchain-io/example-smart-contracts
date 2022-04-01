The size of a smart contract is a critical characteristic of a blockchain network. The homogeneously large size of the compiled smart contract will result in high gas costs during deployment by the user. The node performance will degrade because the node needs to load large amounts of bytes before executing the operations in a contract.

You can use our optimize.sh script to mitigate the above issues.

## Requirements to run the script
* install `wasm-opt` with npm
```
$ npm i wasm-opt -g
```

* install `wasm-snip` with Cargo:
```
$ cargo install wasm-snip
```

## How to optimize the smart contract size?
* Build your smart contract with `wasm32-unknown-unknown` flag. The compiled contract ends with `.wasm` file extension.
* Download the bash script `optimize.sh`. It is located in the same directory as this README.  
* Change the permissions of the script.
```
$ chmod u+x optimize.sh
```
* Run the script by providing your compiled contract file path.
```
$ ./optimize.sh -f <file path>
```
* The output file `optimized-<original_filename>.wasm` is created under the same output directory as your original `.wasm` code.