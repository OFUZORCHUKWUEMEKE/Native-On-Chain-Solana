use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize,Debug)]
pub struct InstructionData{
    pub vault_bump_seed:u8,
    pub lamports:u64,
}

pub static VAULT_ACCOUNT_SIZE:u64 = 1024;


fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult{
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    // The Vault PDA , derieved from the payers address
    let vault = next_account_info(account_info_iter)?;

    let mut instruction_data = instruction_data;
    let instr = InstructionData::deserialize(&mut instruction_data)?;

    let vault_bump_seed = instr.vault_bump_seed;
    let lamports = instr.lamports;
    let vault_size = VAULT_ACCOUNT_SIZE;


    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &vault.key,
            lamports,
            vault_size,
            &program_id
        ),
        &[
            payer.clone(),
            vault.clone()
        ],

        &[
            &[
                b"vault",
                payer.key.as_ref(),
                &[vault_bump_seed]
            ]
        ]
    )?;
    Ok(())

}