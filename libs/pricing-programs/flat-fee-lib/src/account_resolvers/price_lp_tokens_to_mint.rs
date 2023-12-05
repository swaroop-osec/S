use flat_fee_interface::PriceLpTokensToMintKeys;
use solana_program::pubkey::Pubkey;

pub struct PriceLpTokensToMintFreeArgs {
    pub input_lst_mint: Pubkey,
}

impl PriceLpTokensToMintFreeArgs {
    pub fn resolve(self) -> PriceLpTokensToMintKeys {
        PriceLpTokensToMintKeys {
            input_lst_mint: self.input_lst_mint,
        }
    }
}