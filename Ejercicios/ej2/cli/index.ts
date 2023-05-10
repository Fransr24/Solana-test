import * as web3 from '@solana/web3.js';

import {
  getCounter,
  increase,
} from './functions';
import { programId } from './constants';

//const connection = new web3.Connection('http://127.0.0.1:8899');
const connection = new web3.Connection('http://localhost:8899');

async function main() {
  //Instruccion de pda, lo que hago aca es buscar la direccion pero sin inicializarla
  let [counterAddress, _counterBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    programId
  );
  await increase(counterAddress, connection);
  console.log(await getCounter(counterAddress, connection));
}
//Esto no lo entiendo
main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
