// Test module for the Solana-Qubic Bridge
// This file ensures tests are properly organized and can be run

// Integration test modules
mod integration {
    mod bridge_tests;
    mod cross_chain_tests;
}

// Unit test modules
mod unit {
    mod solana_tests;
    mod shared_tests;
}

// Test utilities
pub mod test_utils {
    use solana_program::{pubkey::Pubkey, system_instruction};
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    
    /// Creates a test environment with a funded account
    pub async fn setup_test_environment() -> (BanksClient, Keypair, solana_sdk::hash::Hash) {
        let program_test = ProgramTest::default();
        let (banks_client, payer, recent_blockhash) = program_test.start().await;
        (banks_client, payer, recent_blockhash)
    }
    
    /// Creates a funded account for testing
    pub fn create_funded_account(lamports: u64) -> (Keypair, Account) {
        let keypair = Keypair::new();
        let account = Account::new(lamports, 0, &solana_program::system_program::id());
        (keypair, account)
    }
    
    /// Creates a test transaction
    pub fn create_test_transaction(
        instructions: &[solana_program::instruction::Instruction],
        payer: &Keypair,
        recent_blockhash: solana_sdk::hash::Hash,
    ) -> Transaction {
        Transaction::new_signed_with_payer(
            instructions,
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        )
    }
}
