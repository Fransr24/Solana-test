import * as web3 from '@solana/web3.js';

import {
  getCounter,
  increase,
  createCounter,
} from './functions';
import { programId } from './constants';

//const connection = new web3.Connection('http://127.0.0.1:8899');
const connection = new web3.Connection('http://localhost:8899');

async function main() {
  //let counterKeypair = await createCounter(connection, programId);
  let counterAddress = new web3.PublicKey('Cf77Zyz4xpFrYY12jJDdHBo5ZZEdRbbdYtwvWFXQ9LoB');
  let amount = 3;
  await increase(counterAddress, connection, amount);
  console.log(await getCounter(counterAddress, connection));
}
//Esto no lo entiendo
main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
