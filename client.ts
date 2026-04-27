const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.GestorTareas;

async function main() {
  console.log("Iniciando...");

  const user = provider.wallet.publicKey;

  const [tareaPubkey] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("tarea"), user.toBuffer()],
    program.programId
  );

  console.log("Creando tarea...");

  const tx = await program.methods
    .crearTarea("Hola Solana! ")
    .accounts({
      tarea: tareaPubkey,
      usuario: user,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  console.log(" LISTO!", tx);
}

main();
