use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod gestor_tareas {
    use super::*;

    pub fn crear_tarea(
        ctx: Context<CrearTarea>,
        contenido: String
    ) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;

        tarea.contenido_del = contenido;
        tarea.completada = false;
        tarea.usuario = ctx.accounts.usuario.key();

        Ok(())
    }

    pub fn completar_tarea(
        ctx: Context<CompletarTarea>
    ) -> Result<()> {
        let tarea = &mut ctx.accounts.tarea;

        tarea.completada = true;

        Ok(())
    }

    pub fn eliminar_tarea(
        _ctx: Context<EliminarTarea>
    ) -> Result<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------------
// CONTEXTOS
// ---------------------------------------------------------------------

#[derive(Accounts)]
pub struct CrearTarea<'info> {
    #[account(
        init,
        payer = usuario,
        space = 8 + 32 + 4 + 200 + 1, // <-- CORREGIDO: Espacio fijo
        seeds = [b"tarea", usuario.key().as_ref()],
        bump
    )]
    pub tarea: Account<'info, Tarea>,

    #[account(mut)]
    pub usuario: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompletarTarea<'info> {
    #[account(
        mut,
        seeds = [b"tarea", usuario.key().as_ref()],
        bump
    )]
    pub tarea: Account<'info, Tarea>,

    pub usuario: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarTarea<'info> {
    #[account(
        mut,
        close = usuario,
        seeds = [b"tarea", usuario.key().as_ref()],
        bump
    )]
    pub tarea: Account<'info, Tarea>,

    #[account(mut)]
    pub usuario: Signer<'info>,
}

// ---------------------------------------------------------------------
// CUENTA / ESTRUCTURA
// ---------------------------------------------------------------------

#[account]
pub struct Tarea {
    pub usuario: Pubkey,
    pub contenido_del: String,
    pub completada: bool,
}
