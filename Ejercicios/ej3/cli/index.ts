import * as web3 from '@solana/web3.js';

import {
  getCounter,
  increase,
  createCounter,
} from './functions';

//const connection = new web3.Connection('http://127.0.0.1:8899');
const connection = new web3.Connection('http://localhost:8899');

async function main() {
  let programId = new web3.PublicKey('');
  let counterKeypair = await createCounter(connection, programId);

  let cantidad = 3;
  let counterAddress = counterKeypair.publicKey
  await increase(counterAddress, connection, cantidad);
  console.log(await getCounter(counterAddress, connection));
  console.log('Counter Increase: ', cantidad);
}
//Esto no lo entiendo
main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
