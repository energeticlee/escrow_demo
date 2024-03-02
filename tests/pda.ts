import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Escrow } from "../target/types/escrow";

export const getEscrowPda = (program: Program<Escrow>, pubkey: PublicKey) => {
  const [escrowPda, _escrowPdaBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), pubkey.toBytes()],
    program.programId
  );
  return escrowPda;
};
export const getEscrowData = async (
  program: Program<Escrow>,
  pubkey: PublicKey
) => {
  const data = await program.account.escrow.fetch(pubkey);
  return data;
};
