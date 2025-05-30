use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum PlanetAppError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,

  #[error("arithmetic error")]
  ArithmeticError,

  #[error("current_feeds_account is invalid")]
  InvalidCurrentFeed,
  
  #[error("feed_account_1 is invalid")]
  InvalidFeedDataAccount1,

  #[error("feed_account_2 is invalid")]
  InvalidFeedDataAccount2,

  #[error("feed_account_3 is invalid")]
  InvalidFeedDataAccount3,

  #[error("fallback_account is invalid")]
  InvalidFallBack,


  #[error("payer Account is not Signer")]
  NotSignerPayer,

  #[error("temp Account is not Signer")]
  NotSignerTemp,

  #[error("feed_account_1 is Writable")]
  FirstFeedWritable,

  #[error("feed_account_2 is Writable")]
  SecondFeedWritable,

  #[error("feed_account_3 is Writable")]
  ThirdFeedWritable,

  #[error("fallback_account is Writable")]
  FallBackWritable,

  #[error("authority is not signer")]
  NotSignerAuth,

  #[error("invalid authority")]
  InvalidAuth,

  #[error("invalid config account")]
  InvalidConfig,

  #[error("config account is already initialized")]
  AlreadyInit,

  #[error("invalid subscription package account")]
  InvalidSubsPackAccount,

  #[error("invalid instruction data")]
  InvalidInstructionData,

  #[error("invalid subscription")]
  InvalidSubscrption,

  #[error("invalid fee collector")]
  InvalidFeedCollector,

  #[error("invalid user account")]
  InvalidUserAccount
}

impl From<PlanetAppError> for ProgramError {
  fn from(e: PlanetAppError) -> Self {
    ProgramError::Custom(e as u32)
  }
}