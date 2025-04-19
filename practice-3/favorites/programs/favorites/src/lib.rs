use anchor_lang::prelude::*;

declare_id!("6SkmsSXGuD6kvZov5vTLwpXcGAiTNHWGfDxGitj17FaW");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// what will be put inside the Favorites PDA
#[account]
#[derive(InitSpace)]
pub struct Favorites{
    pub number: u64,

    #[max_len(50)]
    pub color: String
}

// When people call the set_favorites instruction, they will need to provide the accounts that will
// be modified. This keeps Solana fast!
#[derive(Accounts)]
pub struct SetFavorites<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer= user,
        space= ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds= [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

// Solana program
#[program]
pub mod favorites {
    use super::*;

    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();

        msg!("Greeting from {}", context.program_id);
        msg!("
            User {}'s favorit number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context.accounts.favorites.set_inner(Favorites {number, color});
        Ok(())
    }
    // We can also add a get_favorites instruction to get the user's favorite number and color

}

