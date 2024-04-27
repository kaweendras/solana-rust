use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::state::{Account, Mint};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");

    let instruction = instruction_data[0];
    match instruction {
        0 => {
            // Initialize mint
            msg!("Initializing mint");
            initialize_mint(program_id, accounts)?;
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Ok(())
}

fn initialize_mint(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_info = next_account_info(account_info_iter)?;
    let mint_authority_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    // Check if the mint has already been initialized
    if Mint::unpack(&mint_info.data.borrow())?.is_initialized() {
        msg!("Mint is already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Ensure rent is paid
    let rent = &Rent::from_account_info(rent_info)?;
    if !rent.is_exempt(mint_info.lamports(), mint_info.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Initialize the mint
    let mut mint = Mint::unpack_unchecked(&mut mint_info.data.borrow_mut())?;
    mint.mint_authority = *mint_authority_info.key;
    mint.freeze_authority = None;
    Mint::pack(mint, &mut mint_info.data.borrow_mut())?;

    Ok(())
}
