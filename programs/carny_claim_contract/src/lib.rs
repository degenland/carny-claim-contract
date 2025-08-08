use anchor_lang::prelude::*;

declare_id!("6jntGt2WUWhr3uMdqRozqXKmMcjU361s2Hpft7TUVnch");

#[program]
pub mod carny_claim_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
