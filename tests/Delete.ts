import * as anchor from "@project-serum/anchor";import { Program } from "@project-serum/anchor";
import { UnsafePassword } from "../target/types/unsafe_password";
const provider = anchor.AnchorProvider.env();anchor.setProvider(provider);
const wallet = provider.wallet as anchor.Wallet;
const account = new anchor.web3.PublicKey("5rLPJrgrNyWZ3TwvbcJvGmV5qaJdBZ9c72dG3cCbywzW");
describe("test", () => {
    const program = anchor.workspace.UnsafePassword as Program<UnsafePassword>;
    it("Is initialized!", async () => {const tx = await program.methods.delete().accounts({warmHold: account,authority: provider.wallet.publicKey,}).signers([wallet.payer]).rpc();
      console.log("Deleted account", tx);
    });
})