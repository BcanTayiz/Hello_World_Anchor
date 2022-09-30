use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;


// must change after build
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_world {
    use super::*;

    pub fn set_data(ctx: Context<SetData>,data:MyAccount) -> Result<()> {
        if data.data >= 100 {
            return err!(MyError::DataTooLarge);
        }
        // or require!(data.data < 100, MyError::DataTooLarge);
        if ctx.accounts.token_account.amount > 0{
            ctx.accounts.my_account.set_inner(data)
        }
        Ok(())
    }
}

//giving error
/* #[account(<constraints>)]
pub account: AccountType
*/

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub potentially_dangerous: UncheckedAccount<'info>
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    #[account(
        constraint = my_account.mint == token_account.mint,
        has_one = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct  MyAccount{
    data: u64,
    age: u8,
    mint: Pubkey,
}

#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge
}




#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Data {
    pub data: u64,
    pub age: u8
}






