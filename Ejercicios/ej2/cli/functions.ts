import * as web3 from '@solana/web3.js';
import { Buffer } from 'buffer';
import { programId, SIGNER } from './constants';
import * as utils from './utils';
import { Counter } from './types';

export const increase = async (
  counterAddress: web3.PublicKey,
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let instructionNumber = 0;

  let dataBuffer = Buffer.from('');

  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);

//Creo la instruccion con una unica cuenta a mandar que va a ser la creada, no es signer
  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [{ pubkey: counterAddress, isSigner: false, isWritable: true },
          { pubkey: signer.publicKey, isSigner: true, isWritable: true },
          { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false,}
    ],
    data: dataBuffer,
  });
//confirmo transaccion con signer que es la misma de CreateAccount
//Await: espera a que la transaccion sea confirmada por la blockchain de solana para retornar
console.log('sending Transaction');
  let txReceipt = await web3.sendAndConfirmTransaction(
    connection,
    new web3.Transaction().add(instruction),
    [signer]
  );
  return txReceipt;
};

/******* GETTERS ********/

export const getCounter = async (
  counterAddress: web3.PublicKey,
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let counterInfo = await connection.getAccountInfo(
    counterAddress,
    'processed'
  );

  let data = counterInfo ? counterInfo.data : null;
  if (!data) {
    throw new Error('No data retrieved');
  }

  console.log('COUNTER Data: ', data);
  console.log('COUNTER Owner: ', counterInfo?.owner.toBase58());
  console.log('Number: ', data.readUint32LE());

  let counterStruct = Counter.decode(data);
  return counterStruct;
};
