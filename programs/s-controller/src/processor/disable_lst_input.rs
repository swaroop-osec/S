use s_controller_interface::{
    disable_lst_input_verify_account_keys, disable_lst_input_verify_account_privileges,
    DisableLstInputAccounts, DisableLstInputIxArgs,
};
use s_controller_lib::{
    index_to_usize, try_lst_state_list_mut, try_pool_state, DisableEnableLstInputFreeArgs,
    U8BoolMut,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::verify::verify_not_rebalancing_and_not_disabled;

pub fn process_disable_lst_input(
    accounts: &[AccountInfo],
    args: DisableLstInputIxArgs,
) -> ProgramResult {
    let (accounts, lst_index) = verify_disable_lst_input(accounts, args)?;

    let mut lst_state_list_data = accounts.lst_state_list.try_borrow_mut_data()?;
    let lst_state_list = try_lst_state_list_mut(&mut lst_state_list_data)?;

    // lst_index checked in verify
    U8BoolMut(&mut lst_state_list[lst_index].is_input_disabled).set_true();

    Ok(())
}

fn verify_disable_lst_input<'me, 'info>(
    accounts: &'me [AccountInfo<'info>],
    DisableLstInputIxArgs { index }: DisableLstInputIxArgs,
) -> Result<(DisableLstInputAccounts<'me, 'info>, usize), ProgramError> {
    let lst_index = index_to_usize(index)?;

    let actual: DisableLstInputAccounts = load_accounts(accounts)?;

    let free_args = DisableEnableLstInputFreeArgs {
        lst_index,
        pool_state: actual.pool_state,
        lst_state_list: actual.lst_state_list,
    };
    let expected = free_args.resolve_disable()?;

    disable_lst_input_verify_account_keys(actual, expected)
        .map_err(log_and_return_wrong_acc_err)?;
    disable_lst_input_verify_account_privileges(actual)
        .map_err(log_and_return_acc_privilege_err)?;

    let pool_state_bytes = actual.pool_state.try_borrow_data()?;
    let pool_state = try_pool_state(&pool_state_bytes)?;
    verify_not_rebalancing_and_not_disabled(pool_state)?;

    Ok((actual, lst_index))
}
