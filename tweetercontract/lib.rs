#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod tweetercontract {

    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};
    use ink_prelude::{
        vec,
        string::String,
        vec::Vec,
    };

    /// Send tweet
    /// Send private messages
    /// Follow other people
    /// Get list of tweets
    /// Implement an API
    
    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, Clone) )]
    pub struct Tweet {
        tweet_id: i32,
        author: AccountId,
        content: String,
        created_at: u64
    }

    #[derive(Clone, scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo) )]
    pub struct Message {
        content: String,
        from: AccountId,
        to: AccountId,
        created_at: u64
    }

    pub type TweetId = i32;
    pub type MessageId = i32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Tweetercontract {
        tweets: Mapping<TweetId, Tweet>,
        tweets_of: Mapping<AccountId, Vec<Tweet>>,
        conversations: Mapping<MessageId, Vec<Message>>,
        following: Mapping<AccountId, Vec<AccountId>>,
        operators: Mapping<(AccountId, AccountId), bool>,
        tweet_next_id: i32,
        message_next_id: i32,
    }

    #[ink(event)]
    pub struct TweetSent {
        #[ink(topic)]
        id: i32,
        #[ink(topic)]
        author: AccountId,
    }

    #[ink(event)]
    pub struct MessageSent {
        #[ink(topic)]
        id: i32,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
    }

    impl Tweetercontract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_instance: &mut Tweetercontract|{})
        }

        #[ink(message)]
        pub fn tweet(&mut self, content: String){
            let caller = self.env().caller();
            self._tweet(content, caller);
        }

        #[ink(message)]
        pub fn tweet_from(&mut self, content: String, from: AccountId){
            self._tweet(content, from);
        }

        #[ink(message)]
        pub fn _tweet(&mut self, content: String, from: AccountId){
            let caller = self.env().caller();
            let operator = self.operators.get((from, caller)).unwrap_or_default();

            assert!(operator == true, "Operator not authorized");
            let tweet_id = self.next_id_tweet();
            let created_at = self.env().block_timestamp();

            let tweet = Tweet {
                tweet_id,
                author: from,
                content,
                created_at
            };

            self.tweets.insert(tweet_id, &tweet);
            self.tweets_of.insert(from, &(vec![tweet]));
        }

        #[ink(message)]
        pub fn send_message(&mut self, to: AccountId, content: String) {
            let caller = self.env().caller();
            self._send_message(content, caller, to)
        }

        #[ink(message)]
        pub fn send_message_from(&mut self, from: AccountId, to: AccountId, content: String) {
            self._send_message(content, from, to)
        }

        #[ink(message)]
        pub fn _send_message(&mut self, content: String, from: AccountId, to: AccountId) {
            let caller = self.env().caller();
            let operator = self.operators.get((from, caller)).unwrap_or_default();
            let created_at = self.env().block_timestamp();

            assert!(operator == true, "Operator not authorized");

            let message_id = self.next_id_message();
            let message = Message {
                content,
                from,
                to,
                created_at
            };

            self.conversations.insert(message_id, &vec![message]);
            self.env().emit_event(MessageSent {
                id: message_id,
                from,
                to,
            })

        }

        #[ink(message)]
        pub fn follow(&mut self, followed: AccountId) {
            let caller = self.env().caller();
            self.following.insert(caller, &vec![followed]);
        }

        #[ink(message)]
        pub fn get_tweets_of(&mut self, user: AccountId) -> Vec<Tweet> {
            let tweet_of = self.tweets_of.get(user).unwrap_or_default();
            tweet_of
        }

        pub fn next_id_tweet(&mut self) -> TweetId {
            let tweet_id = self.tweet_next_id;
            self.tweet_next_id += 1;
            tweet_id
        }

        pub fn next_id_message(&mut self) -> MessageId {
            let message_id = self.message_next_id;
            self.message_next_id += 1;
            message_id
        }
    }
}
