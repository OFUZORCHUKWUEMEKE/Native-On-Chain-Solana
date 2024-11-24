use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{self, ProgramResult},
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

use crate::example::{InstructionData, VAULT_ACCOUNT_SIZE};

pub fn create_vault_account(client: &RpcClient, program_id: Pubkey, payer: &Keypair) -> Result<()> {
    let (vault_pubkey, vault_bump_seed) =
        Pubkey::find_program_address(&[b"vault", payer.pubkey().as_ref], &program_id);

    // Get the amount of lamports needed to pay for the vault's rent
    let vault_account_size = usize::try_from(VAULT_ACCOUNT_SIZE)?;
    let lamports = client.get_minimum_balance_for_rent_exemption(vault_account_size)?;

    let instr_data = InstructionData {
        vault_bump_seed,
        lamports,
    };
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(vault_pubkey, false),
        AccountMeta::new(system_program::ID, false),
    ];

    // Create the instruction by serializing our instruction data via borsh
    let instruction = Instruction::new_with_borsh(program_id, &instr_data, accounts);
    let blockhash = client.get_latest_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    client.send_and_confirm_transaction(&transaction)?;

    Ok(())
}
