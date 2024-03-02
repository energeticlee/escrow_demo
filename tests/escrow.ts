import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { PublicKey } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { getEscrowData, getEscrowPda } from "./pda";
import { assert } from "chai";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.Escrow as Program<Escrow>;

  let initializer = anchor.web3.Keypair.generate();
  let initializerOfferingAta: PublicKey;
  let initializerAskingAta: PublicKey;
  let taker = anchor.web3.Keypair.generate();
  let takerOfferingAta: PublicKey;
  let takerAskingAta: PublicKey;
  let offerMint: PublicKey;
  let askingMint: PublicKey;
  let escrow: PublicKey;
  let vault: PublicKey;
  let randomKey: PublicKey;

  it("Setup", async () => {
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: await connection.requestAirdrop(initializer.publicKey, 1e9),
    });
    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: await connection.requestAirdrop(taker.publicKey, 1e9),
    });

    offerMint = await createMint(
      connection,
      initializer,
      initializer.publicKey,
      initializer.publicKey,
      6
    );
    askingMint = await createMint(
      connection,
      initializer,
      initializer.publicKey,
      initializer.publicKey,
      6
    );
    initializerOfferingAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        initializer,
        offerMint,
        initializer.publicKey
      )
    ).address;
    takerAskingAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        initializer,
        askingMint,
        taker.publicKey
      )
    ).address;
    await mintTo(
      connection,
      initializer,
      offerMint,
      initializerOfferingAta,
      initializer,
      10_000_000
    );
    await mintTo(
      connection,
      initializer,
      askingMint,
      takerAskingAta,
      initializer,
      10_000_000
    );
  });
  it("Initialize", async () => {
    randomKey = anchor.web3.Keypair.generate().publicKey;
    escrow = getEscrowPda(program, randomKey);
    vault = getAssociatedTokenAddressSync(offerMint, escrow, true);

    try {
      await program.methods
        .initialize(
          new anchor.BN(10_000), // offering_amount
          new anchor.BN(12_000) // asking_amount
        )
        .accounts({
          initializer: initializer.publicKey,
          offeringMint: offerMint,
          askingMint: askingMint,
          initializerOfferAta: initializerOfferingAta,
          escrow: escrow,
          randomSeed: randomKey,
          vault: vault,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([initializer])
        .rpc();
    } catch (error) {
      console.log("error", error);
    }
    const escrowData = await getEscrowData(program, escrow);

    assert(escrowData.randomSeed.toString(), randomKey.toString());
    assert(escrowData.initializer.toString(), initializer.publicKey.toString());
    assert(escrowData.offeringMint.toString(), offerMint.toString());
    assert(escrowData.askingMint, askingMint.toString());
    assert(escrowData.offeringAmount, "10_000");
    assert(escrowData.askingAmount, "12_000");
  });
  it("Cancel", async () => {
    await program.methods
      .cancel()
      .accounts({
        initializer: initializer.publicKey,
        offeringMint: offerMint,
        initializerOfferAta: initializerOfferingAta,
        escrow: escrow,
        randomSeed: randomKey,
        vault: vault,
      })
      .signers([initializer])
      .rpc();
    try {
      await getEscrowData(program, escrow);
      assert.fail();
    } catch (error) {
      assert.ok(error.toString().includes("Account does not exist"));
    }
  });
  it("Re-initialize and Exchange", async () => {
    randomKey = anchor.web3.Keypair.generate().publicKey;
    escrow = getEscrowPda(program, randomKey);
    vault = getAssociatedTokenAddressSync(offerMint, escrow, true);
    takerOfferingAta = getAssociatedTokenAddressSync(
      offerMint,
      taker.publicKey
    );
    initializerAskingAta = getAssociatedTokenAddressSync(
      askingMint,
      initializer.publicKey
    );

    await program.methods
      .initialize(
        new anchor.BN(10_000), // offering_amount
        new anchor.BN(12_000) // asking_amount
      )
      .accounts({
        initializer: initializer.publicKey,
        offeringMint: offerMint,
        askingMint: askingMint,
        initializerOfferAta: initializerOfferingAta,
        escrow: escrow,
        randomSeed: randomKey,
        vault: vault,
      })
      .signers([initializer])
      .rpc();
    try {
      await program.methods
        .exchange()
        .accounts({
          taker: taker.publicKey,
          initializer: initializer.publicKey,
          offeringMint: offerMint,
          askingMint: askingMint,
          takerOfferAta: takerOfferingAta,
          takerAskingAta: takerAskingAta,
          initializerAskingAta: initializerAskingAta,
          escrow: escrow,
          randomSeed: randomKey,
          vault: vault,
        })
        .signers([taker])
        .rpc();
    } catch (error) {
      console.log("error", error);
    }
    try {
      // ACCOUNT SHOULD BE CLOSE IN EXCHANGE IX
      await getEscrowData(program, escrow);
      assert.fail();
    } catch (error) {
      assert.ok(error.toString().includes("Account does not exist"));
    }

    const takerOfferedBalance = await connection.getTokenAccountBalance(
      takerOfferingAta
    );
    const initAskingBalance = await connection.getTokenAccountBalance(
      initializerAskingAta
    );
    assert(takerOfferedBalance.value.uiAmount, "10_000");
    assert(initAskingBalance.value.uiAmount, "12_000");
  });
});
