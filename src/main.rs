use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
    Client, Cluster,
};
use anyhow::Result;
use std::rc::Rc;
use std::str::FromStr;
pub mod functions;

fn main() -> Result<()> {
    let program = Client::new(
        Cluster::Devnet,
        Rc::new(
            read_keypair_file(&*shellexpand::tilde(
                "C:/Users/Mateo/.config/solana/id.json",
            ))
            .expect("Example requires a keypair file"),
        ),
    )
    .program(Pubkey::from_str("J4SWVUscY5P4jMNaqVbWmq3ZiAWuaDJ4pSKh6XWbVWEs").unwrap());
    let warm_hold: Pubkey = Pubkey::from_str("F1M7YJdjxLDRL6utWJtUPbaSXHXdG29VyppaB82nq4tM").unwrap();
    functions::create_password_hold::create_password_hold(
        &program,
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
        "foo".to_string(),
    )?;
    functions::modify::modify(
        &program,
        warm_hold,
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
        "foo2".to_string(),
    )?;
    Ok(())
}
