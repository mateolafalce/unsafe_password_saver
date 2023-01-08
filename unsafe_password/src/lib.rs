use anchor_lang::prelude::*;

declare_id!("J4SWVUscY5P4jMNaqVbWmq3ZiAWuaDJ4pSKh6XWbVWEs");

#[program]
pub mod unsafe_password {
    use super::*;

    pub fn create_password_hold(
        ctx: Context<CreatePasswordHold>,
        authority: Pubkey,
        instagram: String,
        github: String,
        facebook: String,
        gmail: String,
        discord: String,
        phamtom: String,
        metamask: String,
        solsea: String,
        twitter: String,
    ) -> Result<()> {
        let password: &mut Account<Unsafehold> = &mut ctx.accounts.warm_hold;
        let clock: Clock = Clock::get().unwrap();
        password.authority = authority;
        password.instagram = instagram;
        password.github = github;
        password.facebook = facebook;
        password.gmail = gmail;
        password.discord = discord;
        password.phamtom = phamtom;
        password.metamask = metamask;
        password.solsea = solsea;
        password.twitter = twitter;
        password.times_checked = 0;
        password.clock = clock.unix_timestamp;
        Ok(())
    }

    pub fn modify(
        ctx: Context<Modify>,
        instagram: String,
        github: String,
        facebook: String,
        gmail: String,
        discord: String,
        phamtom: String,
        metamask: String,
        solsea: String,
        twitter: String,
    ) -> Result<()> {
        let password: &mut Account<Unsafehold> = &mut ctx.accounts.warm_hold;
        let clock: Clock = Clock::get().unwrap();
        password.instagram = instagram;
        password.github = github;
        password.facebook = facebook;
        password.gmail = gmail;
        password.discord = discord;
        password.phamtom = phamtom;
        password.metamask = metamask;
        password.solsea = solsea;
        password.twitter = twitter;
        password.clock = clock.unix_timestamp;
        password.times_checked += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(mut)]
    pub warm_hold: Account<'info, Unsafehold>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreatePasswordHold<'info> {
    #[account(init, payer = user, space = Unsafehold::LEN)]
    pub warm_hold: Account<'info, Unsafehold>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Unsafehold {
    pub authority: Pubkey,
    pub instagram: String,
    pub github: String,
    pub facebook: String,
    pub gmail: String,
    pub discord: String,
    pub phamtom: String,
    pub metamask: String,
    pub solsea: String,
    pub twitter: String,
    pub clock: i64,
    pub times_checked: u16,
}

impl Unsafehold {
    pub const LEN: usize = DISCRIMINATOR
        + PUBKEY
        + INSTAGRAM
        + GITHUB
        + FACEBOOK
        + GMAIL
        + DISCORD
        + PHAMTOM
        + METAMASK
        + SOLSEA
        + TWITTER
        + CLOCK
        + TIMES;
}
// modify the lenght in function of your needs
const DISCRIMINATOR: usize = 8;
const PUBKEY: usize = 32;
const INSTAGRAM: usize = 4 + 50;
const GITHUB: usize = 4 + 50;
const FACEBOOK: usize = 4 + 50;
const GMAIL: usize = 4 + 50;
const DISCORD: usize = 4 + 50;
const PHAMTOM: usize = 4 + 50;
const METAMASK: usize = 4 + 50;
const SOLSEA: usize = 4 + 50;
const TWITTER: usize = 4 + 50;
const CLOCK: usize = 8;
const TIMES: usize = 2;
