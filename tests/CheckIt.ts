import * as anchor from "@project-serum/anchor";import { Program } from "@project-serum/anchor";
import { UnsafePassword } from "../target/types/unsafe_password";
const provider = anchor.AnchorProvider.env();anchor.setProvider(provider);
const wallet = provider.wallet as anchor.Wallet;
const CreatedAccountPubkey = new anchor.web3.PublicKey("5rLPJrgrNyWZ3TwvbcJvGmV5qaJdBZ9c72dG3cCbywzW");
function timeConverter(UNIX_timestamp){var a = new Date(UNIX_timestamp * 1000);var months = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];var year = a.getFullYear();var month = months[a.getMonth()];var date = a.getDate();var hour = a.getHours();var min = a.getMinutes();var sec = a.getSeconds();var time = date + ' ' + month + ' ' + year + ' ' + hour + ':' + min + ':' + sec ;return time;}
describe("test", () => {
    const program = anchor.workspace.UnsafePassword as Program<UnsafePassword>;
  it("Ticket", async () => { 
    const tx = await program.methods.checkIt().accounts({warmHold: CreatedAccountPubkey,user: provider.wallet.publicKey,}).signers([wallet.payer]).rpc();
    const Account = await program.account.unsafehold.fetch(CreatedAccountPubkey);
    console.log("---------------------------------------------")
    console.log("Authority ",Account.authority.toString());
    console.log("Times Checked ", Account.timesChecked.valueOf());
    console.log("At Time", timeConverter(Account.clock));
    console.log("---------------------------------------------")
    console.log("Discord: ", Account.discord.toString());
    console.log("Facebook: ", Account.facebook.toString());
    console.log("GitHub: ", Account.github.toString());
    console.log("Gmail: ", Account.gmail.toString());
    console.log("Instagram: ", Account.instagram.toString());
    console.log("Metamask: ", Account.metamask.toString());
    console.log("Phamtom: ", Account.phamtom.toString());
    console.log("Solsea: ", Account.solsea.toString());
    console.log("Twitter: ", Account.twitter.toString());
    console.log("---------------------------------------------")
    console.log("Your transaction signature", tx);
  });
});