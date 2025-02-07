// SPDX-License-Identifier: MIT
use solana_program::{
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};

#[derive(Debug, Default)]
struct Config {
    fee_rate: u64,
    pool_status: u8,
}

struct RaydiumAMM {
    config: Config,
}

impl RaydiumAMM {
    pub fn update_amm_config(&mut self, new_config: Config) -> Result<(), ProgramError> {
        // No input validation here (vulnerability)
        self.config.fee_rate = new_config.fee_rate;
        self.config.pool_status = new_config.pool_status;
        Ok(())
    }

    pub fn swap(&self, amount: u64) -> u64 {
        let fee = (amount * self.config.fee_rate) / 100_000; // Assume fee_rate is in basis points
        msg!("Swapped {} tokens, Fee: {}", amount, fee);

        // Use checked_sub to prevent underflow
        match amount.checked_sub(fee) {
            Some(output_amount) => output_amount,
            None => {
                msg!("Swap failed due to insufficient input amount");
                0 // Return 0 if the subtraction would underflow
            }
        }
    }

    pub fn withdraw(&self) -> Result<(), ProgramError> {
        if self.config.pool_status == 0 {
            return Err(ProgramError::InvalidAccountData); // Pool is frozen
        }
        msg!("Withdrawal successful");
        Ok(())
    }
}

fn main() -> ProgramResult {
    let mut amm = RaydiumAMM {
        config: Config {
            fee_rate: 3000, // Initial fee rate (30%)
            pool_status: 1, // Initial pool status (active)
        },
    };

    msg!("Attacker exploiting update_amm_config...");
    let malicious_config = Config {
        fee_rate: 990000, // Extreme fee rate (99%)
        pool_status: 0,   // Disable pool withdrawals
    };

    amm.update_amm_config(malicious_config)?;

    msg!("Updated fee rate: {}", amm.config.fee_rate);
    msg!("Updated pool status: {}", amm.config.pool_status);

    let trade_amount = 1000;
    let output_amount = amm.swap(trade_amount);
    msg!("Output after swap: {}", output_amount);

    match amm.withdraw() {
        Ok(_) => msg!("Withdrawal succeeded (unexpected behavior)"),
        Err(e) => msg!("Withdrawal failed: {:?}", e),
    }

    Ok(())
}