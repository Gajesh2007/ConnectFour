use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub fn did_player_win(
    game: &mut Box<Account<Game>>,
    side: u8
) -> bool {
    let board = game.board[side as usize];
    let directions: [u8; 4] = [1, 7, 6, 8];

    let bb: u64;

    for i in 0..4 {
        bb = board & (board >> directions[i]);
        if (bb & (bb >> (2 * directions[i]))) != 0 {
            return true;
        }
    }

    return false;
}

/// @title Connect Four
/// @author Gajesh Naik 
/// @notice Connect Four Game on Solana

#[program]
pub mod connect_four {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let connect_four = &mut ctx.accounts.connect_four;
        connect_four.top_column = 283691315109952;
        connect_four.game_id = 1;
        
        for i in 0..7 {
            connect_four.initial_height[i] = (7 * i) as u64;
        }

        Ok(())
    }

    pub fn challenge(ctx: Context<Challenge>, nonce: u8, opponent: Pubkey) -> ProgramResult {
        let connect_four = &mut ctx.accounts.connect_four;
        let game = &mut ctx.accounts.game;

        game.player1 = opponent;
        game.player2 = ctx.accounts.challenger.key();
        game.height = connect_four.initial_height;
        game.board = [0 as u64, 0 as u64];
        game.moves = 0;
        game.finished = false;
        game.nonce = nonce;

        connect_four.game_id += 1;

        Ok(())
    }

    pub fn make_move(ctx: Context<MakeMove>, row: u8) -> ProgramResult {
        let connect_four = &mut ctx.accounts.connect_four;
        let game = &mut ctx.accounts.game;

        if (player.key() != (game.moves & 1 == 0 ? game.player1 : game.player2)) {
            return Err(ErrorCode::Unauthorized.into());
        }

        if (game.finished) {
            return Err(ErrorCode::GameFinished.into());
        }

        game.board[game.moves & 1] ^= (1 as u64) << game.height[row]++;

        if ((game.board[games.moves & 1] & connect_four.top_column) != 0) {
            return Err(ErrorCode::InvalidMove.into());
        }

        if (did_player_win(game, game.moves++ & 1)) {
            game.finished = true;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub connect_four: Account<'info, ConnectFour>,

    // Misc.
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct Challenge<'info> {
    #[account(mut)]
    pub connect_four: Account<'info, ConnectFour>,

    #[account(
        seeds = [
            connect_four.to_account_info().key.as_ref(),
            connect_four.game_id.to_string().as_ref(),
        ],
        bump = nonce
    )]
    pub game: Account<'info, Game>,

    pub challenger: Signer<'info>,

    // Misc.
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub connect_four: Account<'info, ConnectFour>,

    #[account(
        seeds = [
            connect_four.to_account_info().key.as_ref(),
            game.game_id.to_string().as_ref(),
        ],
        bump = nonce
    )]
    pub game: Account<'info, Game>,

    pub player: Signer<'info>,

    // Misc.
    system_program: Program<'info, System>,
}

#[account]
pub struct ConnectFour {
    pub initial_height: [u64; 7],
    pub top_column: u64,
    pub game_id: u128,
    
}

#[account]
pub struct Game {
    pub game_id: u128,
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub height: [u64; 7],
    pub board: [u64; 2],
    pub moves: u8,
    pub finished: bool,
    pub nonce: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid Move")]
    InvalidMove,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Game Finished")]
    GameFinished,
}