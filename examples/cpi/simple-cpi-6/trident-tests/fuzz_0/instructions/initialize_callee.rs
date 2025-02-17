use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL")]
# [discriminator ([164u8 , 75u8 , 79u8 , 32u8 , 57u8 , 23u8 , 116u8 , 175u8 ,])]
pub struct InitializeCalleeInstruction {
    pub accounts: InitializeCalleeInstructionAccounts,
    pub data: InitializeCalleeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeCalleeInstructionAccounts {
    pub signer: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeCalleeInstructionData {
    pub input: u16,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionSetters for InitializeCalleeInstruction {
    type IxAccounts = FuzzAccounts;
}
