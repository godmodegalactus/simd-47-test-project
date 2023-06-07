import { PublicKey, sleep } from "@blockworks-foundation/mango-client";
import { Connection, Keypair, LAMPORTS_PER_SOL, Transaction, TransactionInstruction } from "@solana/web3.js"

async function main() {
    const connection = new Connection('http://0.0.0.0:8899');
    
    let tx = new Transaction().add(new TransactionInstruction({
        programId: new PublicKey('9W9evGasPMLtzDKeisdgG1SzP8V9EZ57ynR1uKfJJ2f7'),
        keys: [
            {
                isSigner: false,
                isWritable: false,
                pubkey: new PublicKey('SysvarLastRestartS1ot1111111111111111111111'),
            }
        ],
    }))

    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    const keypair = Keypair.generate();

    await connection.confirmTransaction(await connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL));
    await sleep(2000);
    const signature = await connection.sendTransaction(tx, [keypair]);
    console.log(signature);
    await connection.confirmTransaction(signature);
}

main().then(x => {
    console.log('finished sucessfully')
}).catch(e => {
    console.log('caught an error : ' + e)
})
