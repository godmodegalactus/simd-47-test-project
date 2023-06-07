use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    sysvar::{last_restart_slot, Sysvar}, sysvar::last_restart_slot::LastRestartSlot,
    program_error::ProgramError,
};


entrypoint!(process_instruction);
pub fn process_instruction(
    _: &Pubkey,
    accounts: &[AccountInfo],
    _: &[u8],
) -> ProgramResult {
    let last_restart_slot_sysvar = &accounts[0];
    assert_eq!(last_restart_slot_sysvar.key, &last_restart_slot::id());
    
    solana_program::log::sol_log("testing last restart slot");
    let account_data = last_restart_slot_sysvar.data.try_borrow().map_err(|_| ProgramError::AccountBorrowFailed)?;
    let lrs_data_from_sysvar = if let Ok(data) = bincode::deserialize::<LastRestartSlot>(&account_data) {
        data
    } else {
        return Err(ProgramError::InvalidInstructionData)
    };

    let lrs_data_from_syscall = LastRestartSlot::get()?;

    let msg = format!("sysvar {} syscall {}", lrs_data_from_sysvar.last_restart_slot, lrs_data_from_syscall.last_restart_slot);
    solana_program::log::sol_log(msg.as_str());
    assert_eq!(lrs_data_from_sysvar, lrs_data_from_syscall);
    Ok(())
}