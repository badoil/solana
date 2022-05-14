import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DogMoney } from "../target/types/dog_money";
import {   
  TOKEN_PROGRAM_ID,
  sleep,
  getTokenAccount,
  createTokenAccount,
  createMint, } from './utils';

describe("dog-money", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  let dataAccount = null;

  const program = anchor.workspace.DogMoney as Program<DogMoney>;

  let programSigner;
  it("Is initialized!", async () => {
    // Add your test here.

    //  dataAccount = await anchor.web3.PublicKey.createWithSeed();
    dataAccount = await anchor.web3.Keypair.generate();
    const usdcMint = await createMint(program.provider, provider.wallet.publicKey);

    // program signer PDA - sign transactions for the program
    const [_programSigner, nonce ] = await anchor.web3.PublicKey.findProgramAddress(
      [dataAccount.PublicKey.toBuffer()],
      program.programId
    )
    programSigner = _programSigner;

    // associated account PDA - store user data
    const userData = await program.account.userData.associatedAddress(
      provider.wallet.publicKey,
      usdcMint
    );

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
