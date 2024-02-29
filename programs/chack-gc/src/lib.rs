use anchor_lang::prelude::*;

declare_id!("6HRuRBbKZJM8wfeHaNJUr896DHCZJTcCvbvt4rniW62j");

#[program]
pub mod chack_gc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
