use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    msg,
};
use spl_associated_token_account::create_associated_token_account;

pub mod instruction;

pub mod fee_owner_key {
    use solana_program::declare_id;
    declare_id!("9FSS317C4EU1dhvHdn56NkwrLA6wcDMfDPzFFTe3PqxJ");
}

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_info_iter)?;
    let funding_account = next_account_info(account_info_iter)?;
    let associated_account = next_account_info(account_info_iter)?;
    let fee_owner_acct = next_account_info(account_info_iter)?;
    let sys_rent_acct = next_account_info(account_info_iter)?;
    let spl_token_program_acct = next_account_info(account_info_iter)?;
    let sys_program_acct = next_account_info(account_info_iter)?;
    let _spl_associated_token_acct = next_account_info(account_info_iter)?;
    

    let create_account_ix = create_associated_token_account(
        &funding_account.key,
        &fee_owner_key::ID,
        &mint_account.key,
    );
    invoke(
        &create_account_ix,
        &[
            associated_account.clone(),
            funding_account.clone(),
            fee_owner_acct.clone(),
            mint_account.clone(),
            spl_token_program_acct.clone(),
            sys_program_acct.clone(),
            sys_rent_acct.clone(),
        ],
    )?;
    msg!("after create_associated_token_account");

    Ok(())
}
