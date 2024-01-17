use anchor_lang::prelude::*;

//This is automatically there
// This is the id for the program
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_calculator_dapp {
    use super::*;

    // Context - list of accounts we can retrieve data from
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    // ! We don't need different structs for other ops if the struct is the same
    // The course does this but it seems unecessary
    pub fn add(ctx: Context<Add>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Add>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Add>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn division(ctx: Context<Add>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}

// defining what accounts will be apart of the create Context
#[derive(Accounts)]
pub struct Create<'info> {
    // init specifies new account, user will pay cost, specifies the amount of space on blkchn
    #[account(init, payer=user, space=264)]
    // Accounts must be a generic struct
    pub calculator: Account<'info, Calculator>,
    // signer for the creation
    // makes the user arugment mutable
    #[account(mut)]
    pub user: Signer<'info>,
    // this our system specification of the Solana Blk
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtract<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiply<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// ! is there only one instance of calculator or are they sharing the same instance
// it seems like there is only one instance
#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}

