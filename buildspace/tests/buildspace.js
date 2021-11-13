const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log('Testing...')

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Buildspace;

  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  console.log('GIF Count:', account.totalGifs.toString());

  await program.rpc.addGif("https://media.giphy.com/media/wtD9sB1AB7zxbt0h0y/giphy.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF Count:', account.totalGifs.toString())
  console.log(account.gifList)

  await program.rpc.addVote(0, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(account.gifList)

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