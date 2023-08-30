use anchor_lang::prelude::*;

declare_id!("hF42rUUnXiwZHKPJg5v3fsAF2cybonVNen8VGsAetEg");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
