import { MapCodecConfig } from './../node_modules/@solana/codecs-data-structures/dist/types/map.d';
import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { OraclemindProgram } from "../target/types/oraclemind_program";
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram} from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { createMint } from "@solana/spl-token";

describe("oraclemind-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.oraclemindProgram as Program<OraclemindProgram>;

  let market_creator: Keypair;
  let market: PublicKey;
  let vault: PublicKey;
  let mint: PublicKey;
  let title: string;

  let market_bump: Number;

  before(async() => {
    console.log("state")
    market_creator = Keypair.generate();
    await airdrop(connection, market_creator.publicKey, 5);

    title = "Will sol reach 200$ by the end of month?"

    // [market] = PublicKey.findProgramAddressSync([
    //   Buffer.from("market-account"),
    //   Buffer.from("")
    // ], program.programId);
    mint = await createMint(
      connection,
      market_creator,
      market_creator.publicKey,
      null,
      6
    )
  })


  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .createMarket(title, new BN(100))
      .accounts({
        signer: market_creator.publicKey,
        mint: mint
      })
      .rpc()
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