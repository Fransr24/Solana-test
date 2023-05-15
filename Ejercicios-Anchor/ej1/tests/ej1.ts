import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ej1 } from "../target/types/ej1";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { expect } from "chai";

describe("ej1", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env(); 
  anchor.setProvider(provider); //la libreria de anchor nos provee a provider que nos sirve para tener unaa ketpair y que firme transacciones, la keypair es la mia

  const program = anchor.workspace.Ej1 as Program<Ej1>;
  it("Increase Test", async () => {

    const [counterPubkey] = await findProgramAddressSync([
      Buffer.from('counter')
    ], program.programId)

    await program.methods.increase().accounts({
      counter: counterPubkey,
    }).rpc();

    const counterAfterAddOne = await program.account.counter.fetch(counterPubkey);
    console.log("{}", counterAfterAddOne.number);
    expect(counterAfterAddOne.number).to.equal(1);

    await program.methods.increase().accounts({
      counter: counterPubkey,
    }).rpc();
    const counterAfterAddtwo = await program.account.counter.fetch(counterPubkey);
    expect(counterAfterAddtwo.number).to.equal(2);
  });
});
