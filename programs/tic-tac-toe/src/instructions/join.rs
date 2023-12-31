use anchor_lang::prelude::*;

use crate::*;

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(
        mut,
        seeds = [b"game", game.game_id.as_bytes()],
        bump = game.bump,
    )]
    pub game: Account<'info, Game>,

    pub player: Signer<'info>,
}

// Get the game from PDA using the game id
pub fn handler(ctx: Context<Join>) -> Result<()> {
    require_keys_neq!(
        ctx.accounts.player.key(),
        ctx.accounts.game.players[0].unwrap(),
        TicTacToeError::PlayerOneCannotJoin
    );

    ctx.accounts.game.join(ctx.accounts.player.key())?;
    msg!("Game joined by {}", ctx.accounts.player.key());
    Ok(())
}