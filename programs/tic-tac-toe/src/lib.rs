#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;
use state::*;

declare_id!("hF42rUUnXiwZHKPJg5v3fsAF2cybonVNen8VGsAetEg");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn create(ctx: Context<Create>, game_id: String) -> Result<()> {
        instructions::create::handler(ctx, game_id)
    }

    pub fn join(ctx: Context<Join>) -> Result<()> {
        instructions::join::handler(ctx)
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play::handler(ctx, tile)
    }
}