use anchor_lang::prelude::*;

declare_id!("5VTgsSmeqpsMsEcvP8611oLoxi3FHMNcJ884Jhto5wFU");

#[program]
pub mod encode_programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
