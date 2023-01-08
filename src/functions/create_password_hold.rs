use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Program,
};
use anyhow::Result;
use unsafe_password::Unsafehold;

pub fn create_password_hold(
    program: &Program,
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
    let warm_hold: Keypair = Keypair::new();
    let tx: Signature = program
        .request()
        .accounts(unsafe_password::accounts::CreatePasswordHold {
            warm_hold: warm_hold.pubkey(),
            user: program.payer(),
            system_program: system_program::ID,
        })
        .signer(&warm_hold)
        .args(unsafe_password::instruction::CreatePasswordHold {
            authority: program.payer(),
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
    let account: Unsafehold = program.account(warm_hold.pubkey())?;
    println!("Tx: {}", tx);
    println!("Account: {}", warm_hold.pubkey());
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
