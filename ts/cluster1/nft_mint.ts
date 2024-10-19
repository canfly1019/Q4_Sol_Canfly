import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
        mint: mint,
        name: "FLY_RUG",
        symbol: "FLYRUG",
        uri: "https://devnet.irys.xyz/8eBFciJ2FCkAxhezRetKer7EXJUxXtdyG11rm6Ujq13h",
        sellerFeeBasisPoints: percentAmount(1),

    })
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)
    // https://explorer.solana.com/address/9qhGjCfFzEnX5oucJQBBZbB3Y8KMUCpmdaj4ZRt6h5CT/attributes?cluster=devnet

    console.log("Mint Address: ", mint.publicKey);
})();