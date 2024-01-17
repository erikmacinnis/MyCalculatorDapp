import * as anchor from "@project-serum/anchor";
import assert from "assert"
const { SystemProgram } = anchor.web3;
import { MyCalculatorDapp } from '../target/types/my_calculator_dapp';


describe('mycalculatordapp', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.MyCalculatorDapp as anchor.Program< MyCalculatorDapp >;

  it('Creates a calculator', async() => {
    await program.methods.create("Welcome to Solana")
    .accounts({
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      // I should understand what signers means
      .signers([calculator]).rpc()
    // fetches the account of the calculator
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "Welcome to Solana");
  })

  it('Add two values', async() => {
    await program.methods.add(new anchor.BN(10), new anchor.BN(5))
    .accounts({
      calculator: calculator.publicKey,
      // This doesn't work with the .signers() method
      // ! I should figure out why .signers() doesn't work
    }).rpc()
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(15)))
  })

  it('Subtract two values', async() => {
    await program.methods.subtract(new anchor.BN(10), new anchor.BN(5))
    .accounts({
      calculator: calculator.publicKey,
      // This doesn't work with the .signers() method
      // ! I should figure out why .signers() doesn't work
    }).rpc()
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(5)))
  })

  it('Multiply two values', async() => {
    await program.methods.multiply(new anchor.BN(10), new anchor.BN(5))
    .accounts({
      calculator: calculator.publicKey,
      // This doesn't work with the .signers() method
      // ! I should figure out why .signers() doesn't work
    }).rpc()
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(50)))
  })

  it('Divide two values', async() => {
    await program.methods.division(new anchor.BN(10), new anchor.BN(3))
    .accounts({
      calculator: calculator.publicKey,
      // This doesn't work with the .signers() method
      // ! I should figure out why .signers() doesn't work
    }).rpc()
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(3)))
    assert.ok(account.remainder.eq(new anchor.BN(1)))
  })
})
