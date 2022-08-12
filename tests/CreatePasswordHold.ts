import * as anchor from "@project-serum/anchor";import { Program } from "@project-serum/anchor";
import { UnsafePassword } from "../target/types/unsafe_password";
const { SystemProgram } = anchor.web3;
const provider = anchor.AnchorProvider.env();anchor.setProvider(provider);
describe("test", () => {
    const program = anchor.workspace.UnsafePassword as Program<UnsafePassword>;
    const createAccountHolder = anchor.web3.Keypair.generate();
    it("Is initialized!", async () => {
      const tx = await program.methods.createPasswordHold(
        provider.wallet.publicKey,
        "Ada Lovelace",// instagram 
        "Bill Gates",// github
        "James Gosling",//facebook 
        "James Gosling",// gmail 
        "Guido Van Rossum",// discord
        "Alan Turing",// phamtom
        "Dennis Ritchie",// metamask
        "Anders Hejlsberj",// solsea
        "Tim Berners-Lee",// twitter
      ).accounts({warmHold: createAccountHolder.publicKey,user: provider.wallet.publicKey,systemProgram: SystemProgram.programId,}).signers([createAccountHolder]).rpc();
      console.log("Your transaction signature", tx);
      console.log("Pubkey Account: ", createAccountHolder.publicKey.toBase58()); 
    });
})