use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    pub discriminant: String,
    pub reviewer: Pubkey,
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MovieCommentCounter {
    pub discriminant: String,
    pub is_initialized: bool,
    pub counter: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieComment {
    pub discriminant: String,
    pub is_initialized: bool,
    pub review: Pubkey,
    pub commenter: Pubkey,
    pub comment: String,
    pub count: u64,
}

impl Sealed for MovieAccountState {}

impl IsInitialized for MovieAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl MovieAccountState {
    pub const DISCRIMINATOR: &'static str = "review";

    pub fn get_account_size(title: String, description: String) -> usize {
        return (4 + MovieAccountState::DISCRIMINATOR.len())
            + 1
            + 1
            + (4 + title.len())
            + (4 + description.len());
    }
}

impl IsInitialized for MovieCommentCounter {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl MovieCommentCounter {
    pub const DISCRIMINATOR: &'static str = "counter";

    pub fn get_account_size() -> usize {
        return (4 + MovieAccountState::DISCRIMINATOR.len()) + 1 + 8;
    }
}

impl IsInitialized for MovieComment {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Sealed for MovieCommentCounter {}

impl MovieComment {
    pub const DISCRIMINATOR: &'static str = "comment";

    pub fn get_account_size(comment: String) -> usize {
        return (4 + MovieAccountState::DISCRIMINATOR.len())
            + 1
            + 32
            + 32
            + (4 + comment.len())
            + 8;
    }
}
