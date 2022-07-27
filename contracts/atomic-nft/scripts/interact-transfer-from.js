const { getWarp, getContractTxId, loadWallet } = require('warp-contract-utils');

(async () => {
  const warp = getWarp();
  const [wallet, walletAddress] = await loadWallet(warp);

  let targetWallet;
  if (warp.environment === 'testnet' || warp.environment === 'local') {
    targetWallet = await warp.testing.generateWallet();
  } else {
    targetWallet = await warp.arweave.wallets.generate();
  }
  const targetAddress = await warp.arweave.wallets.getAddress(targetWallet);

  const atomicNFT = warp.contract(getContractTxId(warp.environment, __dirname)).connect(wallet);
  const atomicNFTFromTarget = warp.contract(getContractTxId(warp.environment, __dirname)).connect(targetWallet);

  const approveId = await atomicNFT.writeInteraction(
    { function: 'approve', spender: targetAddress, amount: 10000 },
    { strict: true }
  );

  console.log('Approve tx id', approveId);

  const transferResponse = await atomicNFTFromTarget.writeInteraction(
    {
      function: 'transferFrom',
      from: walletAddress,
      to: targetAddress,
      amount: 100000,
    },
    { strict: true }
  );
  console.log('Transfer From tx id: ', transferResponse.originalTxId);
})();
