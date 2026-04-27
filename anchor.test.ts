import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

describe("gestor-tareas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GestorTareas;
  const wallet = provider.wallet;

  const [tareaPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("tarea"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Crear tarea", async () => {
    await program.methods
      .crearTarea("Aprender Solana")
      .accounts({
        tarea: tareaPda,
        usuario: wallet.publicKey,
      })
      .rpc();

    const cuenta = await program.account.tarea.fetch(tareaPda);
    console.log("CREADA:", cuenta);
  });

  it("Completar tarea", async () => {
    await program.methods
      .completarTarea()
      .accounts({
        tarea: tareaPda,
        usuario: wallet.publicKey,
      })
      .rpc();

    const cuenta = await program.account.tarea.fetch(tareaPda);
    console.log("COMPLETADA:", cuenta);
  });

  it("Eliminar tarea", async () => {
    await program.methods
      .eliminarTarea()
      .accounts({
        tarea: tareaPda,
        usuario: wallet.publicKey,
      })
      .rpc();

    const cuenta = await program.account.tarea.fetchNullable(tareaPda);
    console.log("ELIMINADA:", cuenta);
  });
});
