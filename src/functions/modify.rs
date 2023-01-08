use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey,
        signature::Signature
    },
    Program,
};
use anyhow::Result;
use unsafe_password::Unsafehold;

pub fn modify(
    program: &Program,
    warm_hold: Pubkey,
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
    let tx: Signature = program
        .request()
        .accounts(unsafe_password::accounts::Modify {
            warm_hold: warm_hold,
            user: program.payer(),
        })
        .args(unsafe_password::instruction::Modify {
            instagram,
            github,
            facebook,
            gmail,
            discord,
            phamtom,
            metamask,
            solsea,
            twitter,
        })
        .send()?;
    let account: Unsafehold = program.account(warm_hold)?;
    println!("Tx: {}", tx);
    println!("Account: {}", warm_hold);
    println!("instagram: {}", account.instagram);
    println!("github: {}", account.github);
    println!("facebook: {}", account.facebook);
    println!("gmail: {}", account.gmail);
    println!("discord: {}", account.discord);
    println!("phamtom: {}", account.phamtom);
    println!("metamask: {}", account.metamask);
    println!("solsea: {}", account.solsea);
    println!("twitter: {}", account.twitter);
    Ok(())
}
