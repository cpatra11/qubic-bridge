// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/solana/client/src/transaction_builder.rs
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_instruction;

pub struct TransactionBuilder {
    payer: Keypair,
}

impl TransactionBuilder {
    pub fn new(payer: Keypair) -> Self {
        TransactionBuilder { payer }
    }

    pub fn build_transfer_transaction(
        &self,
        to: Pubkey,
        lamports: u64,
        recent_blockhash: solana_sdk::hash::Hash,
    ) -> Transaction {
        let instruction = system_instruction::transfer(&self.payer.pubkey(), &to, lamports);
        let message = Message::new(&[instruction], Some(&self.payer.pubkey()));
        Transaction::new(&[&self.payer], message, recent_blockhash)
    }

    // Additional methods for building other types of transactions can be added here
}