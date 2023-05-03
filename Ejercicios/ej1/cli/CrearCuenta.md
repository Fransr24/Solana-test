En el ejemplo de output, se crea el programa con solana program deploy, guarda el Id del programa en constants, llama a su clave publica y despues la utiliza como program id en core.ts
En output todo está hecho en core.ts mientras que en el otro ejemplo se separa en index y functions

Maneras de crear cuenta en solana:
 - Linea de comandos: keygen new te genera una clave publica y privada
 - Desde el cliente en el archivo index.ts en el main
 - Desde el programa mio (en process)

 Mejor manera: desde el cliente
 Pasos:
Tengo que llamar a la funcion CreateAccount, para eso necesito obtener todos los parametros que me pide que son: 
https://solana-labs.github.io/solana-web3.js/types/CreateAccountParams.html
 - from_pubkey: Signer, será el feepayer. En el ejemplo de change owner el signer es una cuenta externa que se creó por linea de comandos (le paso la pubkey)
 - Lamports: cantidad de lamports que tendrá al ser inicializada (para la rent)
 - new_account_pubkey: clave publica de la nueva cuenta que se creará (antes genero con Keypair.generate() una keypair nueva, le paso solo la publica)
 - Program_id: programa owner de la cuenta (en el ejemplo es una public key nueva que generé, se llama first owner)
 - Space: Tamaño de la cuenta, solo tiene un dato 
Me devuelve una Transaction instruction que es la que voy a usar como parametro al ejecutar la transaccion
Despues ejecuto la transaccion con la cuenta ix creada
Y finalmente confirmo la transaccion mandando mis 2 keypairs(el signer y la que creo)
El retorno de toda esta funcion de la cuenta va a ser el keypair de la cuenta creada
Ahora llamo a increase que se va a encargar de hacer la transaccion de la instruccion de contador, esa funcion recibe la public key de la nueva cuenta creada y conection
retorna transaction signature, que representa la firma de la transaccion y en caso de error, va a retornar ese error
Y finalmente llamo a la funcion GetCounter, en donde me imprime datos relevantes de la cuenta e imprimo lo que retorna GetCounter (que es su data)