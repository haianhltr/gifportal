const anchor = require("@project-serum/anchor")
const main = async() => {
  console.log("Starting...");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.GifportalJs;
  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram : anchor.web3.SystemProgram.programId
    },
    signers: [baseAccount]
  });
  console.log("Your transaction signature",tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log("GIF count:", account.totalGif.toString());


  await program.rpc.addGif({
    accounts: {
      baseAccount: baseAccount.publicKey,
    }
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("GIF count: ", account.totalGif.toString());
}

const runMain = async() => {
  try{
    await main();
    process.exit(0);
  }
  catch(error)
  {
    console.error(error);
    process.exit(1);
  }
}

runMain();