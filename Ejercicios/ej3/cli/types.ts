import { Buffer } from "buffer";
import * as utils from "./utils";

//Clase contador, con su dato number
export class Contador {
  //en ts, number es para definir un numero, como u32
  constructor(public number: number) {}
// Y su metodo que recibe un buffer y te lo separa en los dos elementos de la clase
  static decode(buffer: Buffer): Contador {
    let newBuffer = utils.copyBuffer(buffer);

    let number;

    [number, newBuffer] = utils.unpackUInt32(newBuffer);

    return new Contador(number);
  }
}
