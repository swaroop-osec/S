use generic_pool_calculator_interface::{
    init_verify_account_keys, init_verify_account_privileges, InitAccounts, InitKeys,
};
use generic_pool_calculator_lib::{
    account_resolvers::InitFreeArgs, utils::try_calculator_state_mut, GenericPoolSolValCalc,
    CALCULATOR_STATE_SEED, CALCULATOR_STATE_SIZE,
};
use sanctum_misc_utils::{
    load_accounts, log_and_return_acc_privilege_err, log_and_return_wrong_acc_err,
};
use sanctum_system_program_lib::{
    init_rent_exempt_account_invoke_signed, InitRentExemptAccountArgs,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use system_program_interface::CreateAccountAccounts;

/// Call on resolved and checked InitAccounts
pub fn process_init_unchecked<P: GenericPoolSolValCalc>(
    InitAccounts {
        payer,
        state,
        system_program: _,
    }: InitAccounts,
    initial_manager: Pubkey,
) -> Result<(), ProgramError> {
    init_rent_exempt_account_invoke_signed(
        CreateAccountAccounts {
            from: payer,
            to: state,
        },
        InitRentExemptAccountArgs {
            space: CALCULATOR_STATE_SIZE,
            owner: P::ID,
        },
        &[&[CALCULATOR_STATE_SEED, &[P::CALCULATOR_STATE_BUMP]]],
    )?;

    let mut bytes = state.try_borrow_mut_data()?;
    let calc_state = try_calculator_state_mut(&mut bytes)?;

    calc_state.manager = initial_manager;
    calc_state.last_upgrade_slot = 0;
    Ok(())
}

pub fn verify_init<'me, 'info, P: GenericPoolSolValCalc>(
    accounts: &'me [AccountInfo<'info>],
) -> Result<InitAccounts<'me, 'info>, ProgramError> {
    let actual: InitAccounts = load_accounts(accounts)?;

    let root_keys = InitFreeArgs {
        payer: *actual.payer.key,
    };
    let expected: InitKeys = root_keys.resolve::<P>();

    init_verify_account_keys(actual, expected).map_err(log_and_return_wrong_acc_err)?;
    init_verify_account_privileges(actual).map_err(log_and_return_acc_privilege_err)?;

    Ok(actual)
}
