use s_controller_interface::{
    AddLiquidityAccounts, EndRebalanceAccounts, RemoveLiquidityAccounts, StartRebalanceAccounts,
    SyncSolValueAccounts,
};
use solana_program::account_info::AccountInfo;

use super::{DstLstMintOf, DstLstPoolReservesOf, SrcLstMintOf, SrcLstPoolReservesOf};

pub trait GetLstStateListAccountInfo<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info>;
}

impl<'me, 'info, T: GetLstStateListAccountInfo<'me, 'info>> GetLstStateListAccountInfo<'me, 'info>
    for &T
{
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        (*self).get_lst_state_list_account_info()
    }
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for SyncSolValueAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for StartRebalanceAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for EndRebalanceAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for AddLiquidityAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}

impl<'me, 'info> GetLstStateListAccountInfo<'me, 'info> for RemoveLiquidityAccounts<'me, 'info> {
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.lst_state_list
    }
}

// impls for src_dst wrapper newtypes

impl<'me, 'info, A: GetLstStateListAccountInfo<'me, 'info>> GetLstStateListAccountInfo<'me, 'info>
    for SrcLstMintOf<'me, A>
{
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_lst_state_list_account_info()
    }
}

impl<'me, 'info, A: GetLstStateListAccountInfo<'me, 'info>> GetLstStateListAccountInfo<'me, 'info>
    for DstLstMintOf<'me, A>
{
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_lst_state_list_account_info()
    }
}

impl<'me, 'info, A: GetLstStateListAccountInfo<'me, 'info>> GetLstStateListAccountInfo<'me, 'info>
    for SrcLstPoolReservesOf<'me, A>
{
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_lst_state_list_account_info()
    }
}

impl<'me, 'info, A: GetLstStateListAccountInfo<'me, 'info>> GetLstStateListAccountInfo<'me, 'info>
    for DstLstPoolReservesOf<'me, A>
{
    fn get_lst_state_list_account_info(&self) -> &'me AccountInfo<'info> {
        self.0.get_lst_state_list_account_info()
    }
}