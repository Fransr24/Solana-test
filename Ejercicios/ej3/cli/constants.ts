import * as web3 from '@solana/web3.js';

export const SECRET = [185,219,159,223,144,225,105,130,72,33,63,5,213,221,87,72,0,234,120,53,245,188,29,155,149,16,98,52,189,180,137,232,139,143,52,203,13,244,108,231,211,79,171,244,168,56,249,162,209,110,126,64,165,136,82,31,44,74,168,206,56,9,248,231];
// your local private test-key. Try: $cat ~/.config/solana/id.json
export const PROGRAM_ID = ''; // Whatever returned from solana deploy
export const KEY: Uint8Array = Uint8Array.from(SECRET);
export const programId = new web3.PublicKey(PROGRAM_ID);
export const SIGNER: web3.Keypair = web3.Keypair.fromSecretKey(KEY);
