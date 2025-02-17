use fuzz_transactions::FuzzTransactions;
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use unauthorized_access_2::entry as entry_unauthorized_access_2;
struct TransactionsSequence;
/// Define transaction sequences for execution.
/// `starting_sequence` runs at the start, `middle` in the middle, and `ending`
/// at the end.
/// For example, to call `InitializeFn`, `UpdateFn` and then `WithdrawFn` during
/// each fuzzing iteration:
/// ```
/// impl FuzzDataBuilder<FuzzTransactions> for InstructionsSequence {
///     fn starting_sequence(fuzzer_data: &mut FuzzerData) ->
/// SequenceResult<FuzzTransactions> {
///         let seq1 = sequence!([InitializeFn, UpdateFn], fuzzer_data);
///         Ok(seq1)
///     }
///     fn middle_sequence(fuzzer_data: &mut FuzzerData) ->
/// SequenceResult<FuzzTransactions> {
///         let seq1 = sequence!([WithdrawFn], fuzzer_data);
///         Ok(seq1)
///     }
///}
/// ```
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/instructions-sequences/#instructions-sequences
impl FuzzSequenceBuilder<FuzzTransactions> for TransactionsSequence {
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<FuzzTransactions> {
        let seq1 = sequence!([InitializeTransaction], fuzzer_data);
        Ok(seq1)
    }
}
fn main() {
    let program_unauthorized_access_2 = ProgramEntrypoint::new(
        pubkey!("5XvBmfPNcHLCgbRK4nRYvfodAnhjArHSed2B3rhkF1Ug"),
        None,
        processor!(entry_unauthorized_access_2),
    );
    let config = TridentConfig::new();
    let mut client = TridentSVM::new_client(&[program_unauthorized_access_2], &config);
    fuzz_trident!(| fuzz_data : TransactionsSequence , client : TridentSVM , config : TridentConfig |);
}
