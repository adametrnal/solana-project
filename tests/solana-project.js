const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3; 

const main = async() => {
 
  const provider = anchor.Provider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // Add your test here.
  const program = anchor.workspace.SolanaProject;

  //create an account keypair for our program to use
  const baseAccount = anchor.web3.Keypair.generate();

  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  
  console.log("Your transaction signature", tx);

  //fetch data from the account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Gif Count', account.totalGifs.toString());

  await program.rpc.addGif({
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });

  //fetch data from the account..again
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Gif Count (second)', account.totalGifs.toString());

}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();