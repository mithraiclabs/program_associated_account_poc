use crate::fee_owner_key;
use solana_program::{
  instruction::{AccountMeta, Instruction},
  pubkey::Pubkey,
  sysvar,
};
use spl_associated_token_account::get_associated_token_address;

pub fn main_instruction(program_id: &Pubkey, payer_key: &Pubkey, mint_key: &Pubkey) -> Instruction {
  let associated_account_key = get_associated_token_address(&fee_owner_key::ID, &mint_key);

  let accounts = vec![
    AccountMeta::new_readonly(*mint_key, false),
    AccountMeta::new(*payer_key, true),
    AccountMeta::new(associated_account_key, false),
    AccountMeta::new(fee_owner_key::ID, false),
    AccountMeta::new_readonly(sysvar::rent::id(), false),
    AccountMeta::new_readonly(spl_token::id(), false),
    AccountMeta::new_readonly(solana_program::system_program::id(), false),
    AccountMeta::new_readonly(spl_associated_token_account::id(), false),
  ];
  // Create the instruction
  Instruction {
    program_id: *program_id,
    accounts,
    data: vec![],
  }
}
