use s_controller_interface::{set_rebalance_authority_ix, SControllerError};
use s_controller_lib::{
    KnownAuthoritySetRebalanceAuthorityFreeArgs, SetRebalanceAuthorityFreeArgs,
};
use s_controller_test_utils::{
    assert_rebalance_authority, MockPoolState, PoolStateProgramTest, DEFAULT_POOL_STATE,
};
use sanctum_solana_test_utils::{assert_custom_err, test_fixtures_dir, IntoAccount};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};

use crate::common::*;

#[tokio::test]
async fn admin_set() {
    let mock_auth_kp =
        read_keypair_file(test_fixtures_dir().join("s-controller-test-initial-authority-key.json"))
            .unwrap();
    let new_rebalance_authority = Pubkey::new_unique();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_rebalance_authority_ix(
        KnownAuthoritySetRebalanceAuthorityFreeArgs {
            new_rebalance_authority,
            pool_state: MockPoolState(DEFAULT_POOL_STATE).into_account(),
        }
        .resolve_pool_admin()
        .unwrap(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &mock_auth_kp], last_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    assert_rebalance_authority(&mut banks_client, new_rebalance_authority).await;
}

#[tokio::test]
async fn rebalance_authority_set() {
    let current_rebalance_authority = Keypair::new();
    let new_rebalance_authority = Pubkey::new_unique();

    let mut pool_state = DEFAULT_POOL_STATE;
    pool_state.rebalance_authority = current_rebalance_authority.pubkey();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(pool_state);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_rebalance_authority_ix(
        KnownAuthoritySetRebalanceAuthorityFreeArgs {
            new_rebalance_authority,
            pool_state: MockPoolState(pool_state).into_account(),
        }
        .resolve_current_rebalance_authority()
        .unwrap(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer, &current_rebalance_authority], last_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    assert_rebalance_authority(&mut banks_client, new_rebalance_authority).await;
}

#[tokio::test]
async fn unauthorized_signer() {
    let new_rebalance_authority = Pubkey::new_unique();

    let program_test = ProgramTest::default()
        .add_s_program()
        .add_pool_state(DEFAULT_POOL_STATE);
    let (mut banks_client, payer, last_blockhash) = program_test.start().await;

    let ix = set_rebalance_authority_ix(
        SetRebalanceAuthorityFreeArgs {
            new_rebalance_authority,
            signer: payer.pubkey(), // payer is unauthorized
        }
        .resolve(),
    )
    .unwrap();

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], last_blockhash);
    let err = banks_client.process_transaction(tx).await.unwrap_err();

    assert_custom_err(
        err,
        SControllerError::UnauthorizedSetRebalanceAuthoritySigner,
    );
}
