import { Keypair } from "@solana/web3.js";

const publicKeyStart= "VOV";

console.log(`Generated keypair starting from '${publicKeyStart}'`);
let count = 1;
let startTime = new Date();


while(true){
    const keypair = Keypair.generate();

    let publicKeyStr= keypair.publicKey.toBase58();

    if (publicKeyStr.startsWith(publicKeyStart)){
        console.log("Public key:", publicKeyStr);
        console.log("Secret key:", keypair.secretKey);
        break;
    }

    count++;
}
console.log(`Finished.`);

console.log(`It took: ${((new Date()).getTime() - startTime.getTime()) / 1000} seconds`);
console.log(`Was genarated: ${count} keypairs`);
        