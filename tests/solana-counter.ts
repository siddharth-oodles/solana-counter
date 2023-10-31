import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCounter } from "../target/types/solana_counter";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("solana-counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaCounter as Program<SolanaCounter>;

  const COUNTER_SEED = "oodles_technologies_counter";

  const initializer = Keypair.generate();

  const counterAccount = PublicKey.findProgramAddressSync(
    [
      Buffer.from(COUNTER_SEED), initializer.publicKey.toBuffer()
    ],
    program.programId
  )[0];

  async function requestAirdrop(key: PublicKey) {
    const keyBalance = await program.provider.connection.getBalance(key);

    if (keyBalance < 2e9) {
      const airdropSignature = await program.provider.connection.requestAirdrop(
        key,
        2e9
      );

      const latestBlockHash =
        await program.provider.connection.getLatestBlockhash();

      await program.provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,
      });
    }
  }

  it("Is initialized!", async () => {
    await requestAirdrop(initializer.publicKey)

    const tx = await program.methods.initialize().accounts({
      counterAccount,
      initializer: initializer.publicKey,
      systemProgram: SystemProgram.programId
    })
      .signers([initializer])
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Counter value", (await program.account.counter.fetch(counterAccount)).counter.toString());
  });

  it("Is incremented!", async () => {
    const tx = await program.methods.incrementCounter().accounts({
      counterAccount,
      updater: initializer.publicKey,
    })
      .signers([initializer])
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Counter value", (await program.account.counter.fetch(counterAccount)).counter.toString());
  });

  it("Is decremented!", async () => {
    const tx = await program.methods.decrementCounter().accounts({
      counterAccount,
      updater: initializer.publicKey,
    })
      .signers([initializer])
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Counter value", (await program.account.counter.fetch(counterAccount)).counter.toString());
  });

  it("Is removed!", async () => {
    await requestAirdrop(initializer.publicKey)

    const tx = await program.methods.removeCounter().accounts({
      counterAccount,
      remover: initializer.publicKey,
    })
      .signers([initializer])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
