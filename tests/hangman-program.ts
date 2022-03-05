import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { HangmanProgram } from '../target/types/hangman_program';

describe('hangman-program', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.HangmanProgram as Program<HangmanProgram>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
