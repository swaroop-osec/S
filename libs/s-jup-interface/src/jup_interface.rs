use anyhow::anyhow;
use jupiter_amm_interface::{
    AccountMap, Amm, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapParams,
};
use s_controller_lib::find_lst_state_list_address;
use sanctum_lst_list::SanctumLstList;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::SPoolJup;

pub const LABEL: &str = "Sanctum Infinity";

impl Amm for SPoolJup {
    /// Initialized by lst_state_list account, NOT pool_state.
    ///
    /// params can optionally be a b58-encoded pubkey string that is the S controller program's program_id.
    ///
    /// Must be updated 2 more times before it can be used, see docs for [`Self::from_lst_state_list_account`]
    fn from_keyed_account(
        KeyedAccount {
            key,
            account,
            params,
        }: &KeyedAccount,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let (program_id, lst_state_list_addr) = match params {
            // default to INF if program_id params not provided
            None => (
                s_controller_lib::program::ID,
                s_controller_lib::program::LST_STATE_LIST_ID,
            ),
            Some(value) => {
                // TODO: maybe unnecessary clone() here?
                let program_id =
                    Pubkey::from_str(&serde_json::from_value::<String>(value.clone())?)?;
                (program_id, find_lst_state_list_address(program_id).0)
            }
        };
        if *key != lst_state_list_addr {
            return Err(anyhow!(
                "Incorrect LST state list addr. Expected {lst_state_list_addr}. Got {key}"
            ));
        }
        let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
        Self::from_lst_state_list_account(program_id, account.clone(), &sanctum_lst_list)
    }

    fn label(&self) -> String {
        LABEL.into()
    }

    fn program_id(&self) -> Pubkey {
        self.program_id
    }

    /// S Pools are 1 per program, so just use program ID as key
    fn key(&self) -> Pubkey {
        self.program_id()
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        self.get_reserve_mints_full()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.get_accounts_to_update_full()
    }

    fn update(&mut self, account_map: &AccountMap) -> anyhow::Result<()> {
        self.update_full(account_map)
    }

    fn quote(&self, quote_params: &QuoteParams) -> anyhow::Result<Quote> {
        self.quote_full(quote_params)
    }

    fn get_swap_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> anyhow::Result<SwapAndAccountMetas> {
        self.get_swap_and_account_metas_full(swap_params)
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn has_dynamic_accounts(&self) -> bool {
        true
    }

    /// TODO: this is not true for AddLiquidity and RemoveLiquidity
    fn supports_exact_out(&self) -> bool {
        true
    }
}
