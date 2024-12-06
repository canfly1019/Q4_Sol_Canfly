import * as anchor from "@coral-xyz/anchor";
import { Program, BN, AnchorError } from "@coral-xyz/anchor";
import { PredictionGame } from "../target/types/prediction_game";
import { PublicKey, Transaction, LAMPORTS_PER_SOL, Keypair, SystemProgram, Authorized } from "@solana/web3.js"
import crypto from 'crypto';
import * as assert from "assert";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

describe("prediction-game", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const connection = provider.connection;
  const program = anchor.workspace.PredictionGame as Program<PredictionGame>;
  const authority = Keypair.generate();
  const player = Keypair.generate();
  
  const wallet = new anchor.Wallet(authority);
  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const priceFeedId = "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43"; // BTC/USD
  const gameDuration = 40;

  const priceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, priceFeedId)
    .toBase58();

  // prepare PDAs
  const gameId = crypto.randomUUID().substring(0, 16);

  const gameStatePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("game_state"), Buffer.from(gameId)],
    program.programId
  )[0];

  const vaultPDA = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), gameStatePDA.toBuffer()],
    program.programId
  )[0];

  const playerStatePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("player_state"), gameStatePDA.toBuffer(), player.publicKey.toBuffer()],
    program.programId
  )[0];

  
  // confirm transaction function
  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  // log transaction function
  const log = async (signature: string): Promise<string> => {
    const cluster = 'devnet';
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=${cluster}`
    );
    return signature;
  };

  it("Transfer SOL to authority and player.", async () => {
    let tx = new Transaction();
    tx.add(
      SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: authority.publicKey,
          lamports: 0.1 * LAMPORTS_PER_SOL,
      }),
      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: player.publicKey,
        lamports: 0.1 * LAMPORTS_PER_SOL,
      })
    );

    await provider
    .sendAndConfirm(tx)
    .then(log);
  });

  it("Start game.", async () => {
    await program.methods
    .startGame({
      identifier: gameId,
      token: priceFeedId,
      duration: new BN(gameDuration),
      initialAmount: new BN(0.05 * LAMPORTS_PER_SOL)
    })
    .accounts({
      authority: authority.publicKey,
      gameState: gameStatePDA,
      vault: vaultPDA,
      priceUpdate: priceFeedAccount,
      systemProgram: SystemProgram.programId,
    })
    .signers([authority])
    .rpc()
    .then(confirm)
    .then(log);
  });

  it("Player guess.", async () => {
    await program.methods
    .playerGuess({
      guess: true,
      betAmount: new BN(0.0003 * LAMPORTS_PER_SOL),
    })
    .accounts({
      player: player.publicKey,
      playerState: playerStatePDA,
      gameState: gameStatePDA,
      vault: vaultPDA,
      systemProgram: SystemProgram.programId,
    })
    .signers([player])
    .rpc()
    .then(confirm)
    .then(log);
  });

  it("Finalize game.", async () => {
    const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
    console.log("Wait for the game to end in 40s.")
    await sleep(gameDuration * 1000);
    await program.methods
    .finalizeGame()
    .accounts({
      authority: authority.publicKey,
      gameState: gameStatePDA,
      priceUpdate: priceFeedAccount
    })
    .signers([authority])
    .rpc()
    .then(confirm)
    .then(log);
  });

  it("Player claim.", async () => {
    try {
      await program.methods
        .playerClaim()
        .accounts({
          player: player.publicKey,
          playerState: playerStatePDA,
          gameState: gameStatePDA,
          vault: vaultPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([player])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (_err) {
      assert.ok(_err instanceof AnchorError, "Error is not an AnchorError");
  
      const err: AnchorError = _err;
  
      const expectedErrMsg = "No reward to claim.";
      const expectedErrCode = 6004;
  
      assert.strictEqual(
        err.error.errorMessage,
        expectedErrMsg,
        `Unexpected error message: ${err.error.errorMessage}`
      );
      assert.strictEqual(
        err.error.errorCode.number,
        expectedErrCode,
        `Unexpected error code: ${err.error.errorCode.number}`
      );
  
      console.error("Caught expected error:", err.toString());
    }
  });
  
  
  it("Close game.", async () => {
    await program.methods
      .closeGame({
        reclaimAmount: new anchor.BN(0.049 * LAMPORTS_PER_SOL)
      })
      .accounts({
        authority: authority.publicKey,
        gameState: gameStatePDA,
        vault: vaultPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc()
      .then(confirm)
      .then(log);
  });
});
