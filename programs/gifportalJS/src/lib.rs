use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Z3yPgytyD5vmiLwQSo5QQrfx6kKvrZeCt3yv92hsDDH");

#[program]
pub mod gifportal_js {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gif = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key
        };
        
        base_account.gif_list.push(item);
        base_account.total_gif += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer =user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct AddGif<'info>
{
    #[account(mut)]
    pub base_account : Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey
}

#[account]
pub struct BaseAccount{
    total_gif: u64,
    gif_list: Vec<ItemStruct>
}
