use candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::call;
use ic_cdk::api::caller;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Token {
    balance: u64,
}

#[derive(CandidType, Deserialize)]
pub enum MyResult {
    Ok(()),
    Err(String),
}

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, Token>> = RefCell::new(HashMap::new());
}

#[ic_cdk_macros::init]
fn init() {
    // Initialization code if needed
}

#[ic_cdk_macros::update]
#[candid::candid_method(update)]
pub fn init_balance() {
    let caller = ic_cdk::caller();
    let token = Token { balance: 1000 };
    BALANCES.with(|balances| {
        balances.borrow_mut().insert(caller, token);
    });
}

#[ic_cdk_macros::query]
#[candid::candid_method(query)]
pub fn get_balance() -> u64 {
    let caller = ic_cdk::caller();
    BALANCES.with(|balances| {
        balances
            .borrow()
            .get(&caller)
            .map(|token| token.balance)
            .unwrap_or(0)
    })
}

#[ic_cdk_macros::update]
#[candid::candid_method(update)]
pub fn send_tokens(to: Principal, amount: u64) -> MyResult {
    let caller = ic_cdk::caller();
    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        if let Some(caller_token) = balances.get_mut(&caller) {
            if caller_token.balance >= amount {
                caller_token.balance -= amount;
                let receiver_token = balances.entry(to).or_insert(Token { balance: 0 });
                receiver_token.balance += amount;
                MyResult::Ok(())
            } else {
                MyResult::Err("Insufficient balance".to_string())
            }
        } else {
            MyResult::Err("Sender not found".to_string())
        }
    })
}

// Required Candid export
#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    // Code to handle upgrades if needed
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    // Code to handle pre-upgrade state saving if needed
}

// Export the Candid interface
ic_cdk::export_candid!();
