use anchor_lang::prelude::*;

// 이 ID는 나중에 Build 후 업데이트됩니다.
declare_id!("FW52ywcwuEzQMXfgs12syQ4gC5zTgNSxRSCR9BptGwCT");

#[program]
pub mod last_signal_rust {
    use super::*;

    // [1] 캡슐 생성
    pub fn create_capsule(
        ctx: Context<CreateCapsule>,
        beneficiary: Pubkey,
        secret: String,
    ) -> Result<()> {
        let capsule = &mut ctx.accounts.capsule;
        capsule.owner = ctx.accounts.owner.key();
        capsule.beneficiary = beneficiary;
        capsule.encrypted_msg = secret;

        // 현재 시간 + 100초 뒤 잠금 해제
        let clock = Clock::get()?;
        capsule.unlock_time = clock.unix_timestamp + 100;

        msg!("Capsule created! Unlocks at {}", capsule.unlock_time);
        Ok(())
    }

    // [2] 생존 신고 (Heartbeat)
    pub fn heartbeat(ctx: Context<Heartbeat>) -> Result<()> {
        let capsule = &mut ctx.accounts.capsule;

        // 시간 연장
        let clock = Clock::get()?;
        capsule.unlock_time = clock.unix_timestamp + 100;

        msg!("Heartbeat received! Timer reset to {}", capsule.unlock_time);
        Ok(())
    }

    // [3] 유산 상속 (Claim)
    pub fn claim_capsule(ctx: Context<ClaimCapsule>) -> Result<()> {
        let capsule = &mut ctx.accounts.capsule;
        let clock = Clock::get()?;

        // 시간 체크
        if clock.unix_timestamp <= capsule.unlock_time {
            return err!(ErrorCode::TimeNotPassed);
        }

        // 소유권 이전
        capsule.owner = ctx.accounts.receiver.key();
        msg!("Capsule claimed by {}", ctx.accounts.receiver.key());
        Ok(())
    }
}

// [데이터 구조]
#[account]
pub struct TimeCapsule {
    pub owner: Pubkey,
    pub beneficiary: Pubkey,
    pub unlock_time: i64,
    pub encrypted_msg: String,
}

// [권한 설정]
#[derive(Accounts)]
pub struct CreateCapsule<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 200, seeds = [b"capsule", owner.key().as_ref()], bump)]
    pub capsule: Account<'info, TimeCapsule>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Heartbeat<'info> {
    #[account(mut, has_one = owner)]
    pub capsule: Account<'info, TimeCapsule>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimCapsule<'info> {
    #[account(mut, has_one = beneficiary)]
    pub capsule: Account<'info, TimeCapsule>,
    pub beneficiary: Signer<'info>, // 상속자가 서명해야 함
    pub receiver: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Time has not passed yet.")]
    TimeNotPassed,
}
