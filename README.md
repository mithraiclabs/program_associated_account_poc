Re-creates an issue when trying to use `spl_associated_token_account::create_associated_token_account` in a cross program invocation.

To run `cargo test-bpf --manifest-path programs/program_associated_account_poc/Cargo.toml`

### What was expected
Hoping the program could create an associated token account for an account that it does not own.

### What actually happened
The program fails with error `Error processing Instruction 0: instruction modified data of an account it does not own`

Full output
````
thread 'process_test::test_process' panicked at 'called `Result::unwrap()` on an `Err` value: ClientError { request: Some(SendTransaction), kind: RpcError(RpcResponseError { code: -32002, message: "Transaction simulation failed: Error processing Instruction 0: instruction modified data of an account it does not own", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(InstructionError(0, ExternalAccountDataModified)), logs: Some(["Program 4Z96bK2NcQznpSKDXpw1fa3vX7CT3DU4qV4KRssEvq35 invoke [1]", "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [2]", "Program log: Transfer 2039280 lamports to the associated token account", "Program 11111111111111111111111111111111 invoke [3]", "Program 11111111111111111111111111111111 success", "Program log: Allocate space for the associated token account", "Program 11111111111111111111111111111111 invoke [3]", "Program 11111111111111111111111111111111 success", "Program log: Assign the associated token account to the SPL Token program", "Program 11111111111111111111111111111111 invoke [3]", "Program 11111111111111111111111111111111 success", "Program log: Initialize the associated token account", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]", "Program log: Instruction: InitializeAccount", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3920 of 171479 compute units", "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success", "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 25133 of 192011 compute units", "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success", "Program log: after create_associated_token_account", "Program 4Z96bK2NcQznpSKDXpw1fa3vX7CT3DU4qV4KRssEvq35 consumed 33976 of 200000 compute units", "Program 4Z96bK2NcQznpSKDXpw1fa3vX7CT3DU4qV4KRssEvq35 success"]) }) }) }', programs/program_associated_account_poc/tests/integration/process_test.rs:46:2
````
