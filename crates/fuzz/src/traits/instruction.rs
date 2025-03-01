use solana_sdk::instruction::AccountMeta;

use crate::traits::FuzzClient;

/// Trait implementing methods for the fuzzed instructions
pub trait InstructionSetters {
    type IxAccounts;
    /// Set Instruction data
    #[allow(unused_variables)]
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {}

    /// Set Instruction accounts
    #[allow(unused_variables)]
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
    }

    /// Set Instruction remaining accounts
    #[allow(unused_variables)]
    fn set_remaining_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
    }
}

pub trait InstructionMethods: InstructionSetters {
    #[doc(hidden)]
    /// Get Instruction discriminator
    fn get_discriminator(&self) -> Vec<u8>;

    #[doc(hidden)]
    /// Get Instruction program id
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey;

    #[doc(hidden)]
    /// Set accounts before transaction
    fn set_snapshot_before(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Set accounts after transaction
    fn set_snapshot_after(&mut self, client: &mut impl FuzzClient);

    #[doc(hidden)]
    /// Convert accounts to account metas
    fn to_account_metas(&mut self) -> Vec<AccountMeta>;

    #[doc(hidden)]
    /// Resolve accounts
    fn resolve_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    );
}
