# Building smart contracts with ParallelChain F pchain_compile

For deploying your smart contract on ParallelChain F, we recommend using **pchain_compile**, which is a CLI build tool for smart contract developers to build their source code to optimized WASM binaries in an environment that is consistent with that of ParallelChain F. For more information on **pchain_compile**, users are recommended to visit the official documentation page [here](https://docs.parallelchain.io/smart_contract_sdk/build_contract/).


## Requirements to use pchain_compile
* `pchain_compile` only needs Docker to be installed on the system for operation. To know more 
  about Docker and to install it you may refer to the instructions provided [here](https://docs.docker.com/get-docker/).


## Steps to build your smart contracts using pchain_compile

* To run `pchain_compile` after installing prerequisites, give relevant permissions to the executable.

   - In Ubuntu, give permission to run the executable. 
      ```
      chmod +x pchain_compile
      ```
   - For Windows there is no need for this step.

* Run the following command.
  `./pchain_compile build --source <SOURCE_CODE_PATH> --destination  <DESTINATION_PATH>`
  More illustrative examples on how to execute the binary in different OS platforms are shown below.


## Examples
The following is a real life example of how `pchain_compile` can be used where some source code is kept with a manifest file with package name `source_code` within a directory on two different OS platforms.

- on Linux 
      
```
./pchain_compile  build \
--source $HOME/test/source_code \
--destination $HOME/results/

Output: Finished compiling. ParallelChain F smart contract (source_code.wasm) is saved at (home/user/results/).
```

- on Linux 
      
```
./pchain_compile  build \
--source $HOME/test/source_code

Output: Finished compiling. ParallelChain F smart contract (source_code.wasm) is saved at (home/user/test/source_code).
```

- on Windows

```
pchain_compile.exe  build \
--source D:\test\source_code \
--destination D:\results 

Output: Finished compiling. ParallelChain F smart contract (source_code.wasm) is saved at (D:\results).
```

- on Windows

```
pchain_compile.exe  build \
--source D:\test\source_code

Output: Finished compiling. ParallelChain F smart contract (source_code.wasm) is saved at (D:\test\source_code).
```

## Download **pchain_compile**

'pchain_compile' supports Linux, macOS and Windows. Depending on the operating system, they can be downloaded from the 
following links below.

=== "Linux / macOS"

For Linux / macOS, users can download `pchain_compile` from [here](https://cms.parallelchain.io/pchain_compile_linux_v1.1.tar.xz)

=== "Windows"

For Windows, users can download `pchain_compile` from [here](https://cms.parallelchain.io/pchain_compile_win_v1.1.zip)
