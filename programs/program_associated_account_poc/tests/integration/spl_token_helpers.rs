use crate::solana_helpers::send_and_confirm_transaction;
use solana_client::{client_error::ClientError, rpc_client::RpcClient};
use solana_program::{program_pack::Pack, pubkey::Pubkey, system_instruction};
use solana_sdk::{
  commitment_config::CommitmentConfig,
  message::Message,
  signature::{Keypair, Signer},
  transaction::Transaction,
};
use spl_token::{
  instruction as token_instruction,
  state::{Account, Mint},
};


pub fn create_spl_mint_account(
  client: &RpcClient,
  spl_mint: &Keypair,
  payer_keys: &Keypair,
) -> Result<(), ClientError> {
  let data_len = Mint::LEN;

  let min_balance = client.get_minimum_balance_for_rent_exemption(data_len)?;

  let create_acct_ix = system_instruction::create_account(
    &payer_keys.pubkey(),
    &spl_mint.pubkey(),
    min_balance,
    data_len as u64,
    &spl_token::id(),
  );

  // TODO [rust] whats the best way easily handle multiple error types in a Result
  let init_mint_ix = token_instruction::initialize_mint(
    &spl_token::id(),
    &spl_mint.pubkey(),
    &payer_keys.pubkey(),
    None,
    18,
  )
  .unwrap();

  let message = Message::new(&[create_acct_ix, init_mint_ix], Some(&payer_keys.pubkey()));

  let mut transaction = Transaction::new_unsigned(message.clone());

  let (blockhash, _, _) = client
    .get_recent_blockhash_with_commitment(CommitmentConfig::processed())?
    .value;
  transaction.try_sign(&[payer_keys, spl_mint], blockhash)?;

  client.send_and_confirm_transaction_with_spinner_and_commitment(
    &transaction,
    CommitmentConfig::processed(),
  )?;
  println!(
    "Created and Initialized SPL mint account {}",
    spl_mint.pubkey()
  );

  Ok(())
}