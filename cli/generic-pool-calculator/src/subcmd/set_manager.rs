use clap::Args;
use generic_pool_calculator_interface::{set_manager_ix_with_program_id, SetManagerKeys};
use generic_pool_calculator_lib::{pda::CalculatorStateFindPdaArgs, utils::try_calculator_state};
use sanctum_solana_cli_utils::{parse_pubkey_src, parse_signer, TxSendingNonblockingRpcClient};
use solana_sdk::{
    message::{v0::Message, VersionedMessage},
    transaction::VersionedTransaction,
};

use super::Subcmd;

#[derive(Args, Debug)]
#[command(long_about = "Sets the SOL value calculator program's manager")]
pub struct SetManagerArgs {
    #[arg(
        long,
        short,
        help = "The program's current manager signer. Defaults to config wallet if not set."
    )]
    pub curr_manager: Option<String>,

    #[arg(help = "The new program's manager to set. Can be a pubkey or signer.")]
    pub new_manager: String,
}

impl SetManagerArgs {
    pub async fn run(args: crate::Args) {
        let Self {
            curr_manager,
            new_manager,
        } = match args.subcmd {
            Subcmd::SetManager(a) => a,
            _ => unreachable!(),
        };
        let payer = args.config.signer();
        let rpc = args.config.nonblocking_rpc_client();
        let program_id = args.program;

        let curr_manager_signer = curr_manager.map(|s| parse_signer(&s).unwrap());
        let curr_manager = curr_manager_signer.as_ref().unwrap_or(&payer);

        let new_manager = parse_pubkey_src(&new_manager).unwrap();
        let state_pda = CalculatorStateFindPdaArgs { program_id }
            .get_calculator_state_address_and_bump_seed()
            .0;
        let state_data = rpc.get_account_data(&state_pda).await.unwrap();
        let state = try_calculator_state(&state_data).unwrap();
        if state.manager != curr_manager.pubkey() {
            eprintln!(
                "Wrong manager. Expected: {}. Got: {}",
                state.manager,
                curr_manager.pubkey()
            );
            std::process::exit(-1);
        }

        let ix = set_manager_ix_with_program_id(
            program_id,
            SetManagerKeys {
                manager: state.manager,
                new_manager: new_manager.pubkey(),
                state: state_pda,
            },
        )
        .unwrap();

        let mut signers = vec![payer.as_ref(), curr_manager.as_ref()];
        signers.dedup();

        let rbh = rpc.get_latest_blockhash().await.unwrap();
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(Message::try_compile(&payer.pubkey(), &[ix], &[], rbh).unwrap()),
            &signers,
        )
        .unwrap();

        rpc.handle_tx(&tx, args.send_mode).await;
    }
}
