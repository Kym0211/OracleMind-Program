import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { OraclemindProgram } from "../target/types/oraclemind_program";
import { Keypair, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { Account, createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

describe("oraclemind-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.oraclemindProgram as Program<OraclemindProgram>;

  let marketCreator: Keypair;
  let market: PublicKey;
  let vault: PublicKey;
  let mint: PublicKey;
  let tokenCreator: Keypair;
  let bettor: Keypair;
  let bettorATA: Account;
  let bettorAccount: PublicKey;

  console.log("state")
  before(async() => {
    marketCreator = Keypair.generate();
    tokenCreator = Keypair.generate();
    bettor = Keypair.generate();
    await airdrop(connection, marketCreator.publicKey, 5);
    await airdrop(connection, tokenCreator.publicKey, 5);
    await airdrop(connection, bettor.publicKey, 5);

    mint = await createMint(
      connection,
      tokenCreator,
      tokenCreator.publicKey,
      null,
      6
    );  

    bettorATA = await getOrCreateAssociatedTokenAccount(
      connection,
      bettor,
      mint,
      bettor.publicKey,
    );

    mintTo(
      connection,
      tokenCreator,
      mint,
      bettorATA.address,
      tokenCreator,
      10_000_000
    );

    [market] = PublicKey.findProgramAddressSync(
      [Buffer.from("market-account"), Buffer.from("title")],
      program.programId
    );

    [bettorAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("bettor"), market.toBuffer(), bettor.publicKey.toBuffer()],
      program.programId
    );

    vault = await getAssociatedTokenAddress(
      mint,
      market,
      true
    );

  })


  it("Creating Market...!", async () => {
    // Add your test here.
    const tx = await program.methods
      .createMarket("title", new BN(1783838753))
      .accounts({
        signer: marketCreator.publicKey,
        mint: mint,
      })
      .signers([marketCreator])
      .rpc({ commitment: "confirmed" })
      
      console.log("Market Created Transaction", tx);
  });

  it("Placing Bet...!", async () => {
    // Add your test here.
    
    const tx = await program.methods
    .placeBet(true, new BN(10))
    .accounts({
      signer: bettor.publicKey,
      marketAccount: market,
      bettorATA: bettorATA,
      vault: vault,
      mint: mint,
    })
    .signers([bettor])
    .rpc({ commitment: "confirmed" })
    
    console.log("Bet Created Transaction", tx);

    // const marketAccount = await program.account.market.fetch(market);
    // const bettorA = await program.account.market.fetch(bettorAccount);

    // console.log("marketAccount: ", marketAccount)
    // console.log("bettorAccount: ", bettorA)
  });

  it("Resolving Market...!", async () => {
    // Add your test here.
    
    const tx = await program.methods
    .resolveMarket(true)
    .accounts({
      signer: marketCreator.publicKey,
      marketAccount: market,
    })
    .signers([marketCreator])
    .rpc({ commitment: "confirmed" })
    
    console.log("Market Resolve Transaction", tx);

    const marketAccount = await program.account.market.fetch(market);

    // console.log("marketAccount: ", marketAccount)
  });

    it("Claiming Winnings...!", async () => {
      // Add your test here.


      
      const tx = await program.methods
      .claimWinnings()
      .accounts({
        signer: bettor.publicKey,
        marketAccount: market,
        bettor: bettorAccount,
        bettorAta: bettorATA.address,
        vault: vault
      })
      .signers([bettor])
      .rpc({ commitment: "confirmed" })
      
      console.log("Claim Winnings Transaction", tx);
  
      const marketAccount = await program.account.market.fetch(market);

      const bettorAcc = await program.account.bettor.fetch(bettorAccount);
      // console.log("bettor account :", bettorAcc)

      // console.log("marketAccount: ", marketAccount)
    });
});


async function airdrop(connection: anchor.web3.Connection, address: PublicKey, amount: number) {
    let airdrop_signature = await connection.requestAirdrop(
      address,
      amount * LAMPORTS_PER_SOL
    );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪙 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✍🏾 Tx Signature: ", confirmedAirdrop, "");
}