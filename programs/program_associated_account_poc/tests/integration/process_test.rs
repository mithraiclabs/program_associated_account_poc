
use crate::{
  solana_helpers,
  spl_token_helpers,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
  commitment_config::CommitmentConfig,
  signature::{Keypair, Signer},
};
use solana_program::{
  instruction::{AccountMeta, Instruction},
};
use spl_associated_token_account::get_associated_token_address;

#[test]
fn test_sanity() {
  assert_eq!(true, true);
}

#[test]
fn test_process() {
  let client = RpcClient::new_with_commitment(
    "http://localhost:8899".to_string(),
    CommitmentConfig::processed(),
  );
  let program_id = solana_helpers::load_bpf_program(&client, "program_associated_account_poc");

  let payer_keys = solana_helpers::create_account_with_lamports(&client, 100_000_000_000);

  let mint_keys = Keypair::new();
  spl_token_helpers::create_spl_mint_account(&client, &mint_keys, &payer_keys).unwrap();

  let ix = program_associated_account_poc::instruction::main_instruction(
    &program_id,
    &payer_keys.pubkey(),
    &mint_keys.pubkey(),
  );
  let signers = vec![&payer_keys];
  solana_helpers::send_and_confirm_transaction(
    &client,
    ix,
    &payer_keys.pubkey(),
    signers,
)
.unwrap();


  assert_eq!(true, false);
}
