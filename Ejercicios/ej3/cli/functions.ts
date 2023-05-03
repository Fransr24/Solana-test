import * as web3 from '@solana/web3.js';
import { Buffer } from 'buffer';
import { programId, SIGNER } from './constants';
import * as utils from './utils';
import { Contador } from './types';

export const createCounter = async (
  connection: web3.Connection,
  ownerId: web3.PublicKey,
  signer: web3.Keypair = SIGNER
): Promise<web3.Keypair> => {
  const keypair = web3.Keypair.generate();

  console.log("Counter Address: ", keypair.publicKey.toBase58());

  let ix = web3.SystemProgram.createAccount({
    fromPubkey: signer.publicKey,
    lamports: 10000000,
    newAccountPubkey: keypair.publicKey,
    programId: ownerId,
    space: 8,
  });

  let tx = new web3.Transaction().add(ix);

  await web3.sendAndConfirmTransaction(connection, tx, [signer, keypair]);

  return keypair;
};

export const increase = async (
  counterAddress: web3.PublicKey,
  connection: web3.Connection,
  amount: number,
  signer: web3.Keypair = SIGNER,
) => {
  let instructionNumber = 0;

  let dataBuffer = Buffer.from('');
  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);
  dataBuffer = utils.packUInt8(dataBuffer, amount);
  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [{ pubkey: counterAddress, isSigner: false, isWritable: true }],
    data: dataBuffer,
  });
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

  let counterStruct = Contador.decode(data);
  return counterStruct;
};
