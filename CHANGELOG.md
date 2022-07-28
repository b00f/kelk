# CHANGELOG

## Version 0.3

- Introducing `Context` and `OwnedContext` structs for dependency injection at runtime
- Adding `derive` and `allocate` crates
- Adding `StorageBST`, `StorageLinkedList`, `StorageVec` and `StorageStr` data types
- Adding `allocate` for storage module and basic storage read/write functions
- Adding `Storage` and `Blockchain` modules


## Version 0.2

- Defining Memory Pointer for managing memory allocation in WASM's linear memory.
- Implementing `deallocate` API to free the allocated memory for WASM module
- Implementing Instantiate, Process and Query API
- Updating Storage API for primitive types
- Storage BST (Binary Search Tree) for lib crate

## Version 0.1

- Instantiating contract actor
- Processing messages
- Read and write storage APIs