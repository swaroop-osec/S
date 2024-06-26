use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum LidoCalculatorError {
    #[error("Lido exchange rate not yet updated")]
    ExchangeRateNotUpdatedInThisEpoch = 0,
}
impl From<LidoCalculatorError> for ProgramError {
    fn from(e: LidoCalculatorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for LidoCalculatorError {
    fn type_of() -> &'static str {
        "LidoCalculatorError"
    }
}
impl PrintProgramError for LidoCalculatorError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
