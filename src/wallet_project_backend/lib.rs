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



//Tests
#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::api::call;
    use ic_cdk::export::Principal;
    use std::cell::RefCell;
    use std::collections::HashMap;

    // Helper function to simulate getting the balance of a caller
    fn get_test_balance(caller: Principal) -> u64 {
        BALANCES.with(|balances| {
            balances
                .borrow()
                .get(&caller)
                .map(|token| token.balance)
                .unwrap_or(0)
        })
    }

    // Test for initializing the balance
    #[test]
    fn test_init_balance() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        // Simulate initializing balance for the caller
        BALANCES.with(|balances| {
            let token = Token { balance: 1000 };
            balances.borrow_mut().insert(caller.clone(), token);
        });

        let balance = get_test_balance(caller.clone());
        assert_eq!(balance, 1000, "Balance should be initialized to 1000.");
    }

    // Test for getting the balance
    #[test]
    fn test_get_balance() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        // Initialize balance to 1000
        BALANCES.with(|balances| {
            let token = Token { balance: 1000 };
            balances.borrow_mut().insert(caller.clone(), token);
        });

        // Get the balance
        let balance = get_test_balance(caller.clone());
        assert_eq!(balance, 1000, "Balance should be 1000.");
    }

    // Test for sending tokens (success case)
    #[test]
    fn test_send_tokens_success() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        let receiver = Principal::from_text("bbbbb-bb").unwrap();
        
        // Initialize balances
        BALANCES.with(|balances| {
            let sender_token = Token { balance: 1000 };
            let receiver_token = Token { balance: 500 };
            balances.borrow_mut().insert(caller.clone(), sender_token);
            balances.borrow_mut().insert(receiver.clone(), receiver_token);
        });

        // Send tokens
        let result = send_tokens(receiver, 500);
        assert!(result.is_ok(), "Sending tokens should succeed.");

        // Verify balances after sending
        let sender_balance = get_test_balance(caller);
        let receiver_balance = get_test_balance(receiver);
        assert_eq!(sender_balance, 500, "Sender's balance should be 500.");
        assert_eq!(receiver_balance, 1000, "Receiver's balance should be 1000.");
    }

    // Test for sending tokens (insufficient balance)
    #[test]
    fn test_send_tokens_insufficient_balance() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        let receiver = Principal::from_text("bbbbb-bb").unwrap();

        // Initialize balances
        BALANCES.with(|balances| {
            let sender_token = Token { balance: 300 };
            let receiver_token = Token { balance: 500 };
            balances.borrow_mut().insert(caller.clone(), sender_token);
            balances.borrow_mut().insert(receiver.clone(), receiver_token);
        });

        // Attempt to send more tokens than the sender has
        let result = send_tokens(receiver, 500);
        assert_eq!(result, Err("Insufficient balance".to_string()), "Sending tokens should fail due to insufficient balance.");
    }

    // Test for sending tokens (sender not found)
    #[test]
    fn test_send_tokens_sender_not_found() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        let receiver = Principal::from_text("bbbbb-bb").unwrap();

        // Attempt to send tokens when the sender is not initialized
        let result = send_tokens(receiver, 500);
        assert_eq!(result, Err("Sender not found".to_string()), "Sending tokens should fail due to sender not found.");
    }

    // Test for sending tokens (receiver not found, initialized with 0 balance)
    #[test]
    fn test_send_tokens_receiver_not_found() {
        let caller = Principal::from_text("aaaaa-aa").unwrap();
        let receiver = Principal::from_text("bbbbb-bb").unwrap();

        // Initialize sender balance
        BALANCES.with(|balances| {
            let sender_token = Token { balance: 1000 };
            balances.borrow_mut().insert(caller.clone(), sender_token);
        });

        // Send tokens to a receiver who is not found in the system
        let result = send_tokens(receiver, 500);
        assert!(result.is_ok(), "Sending tokens should succeed even if receiver is not found, as balance should be initialized to 0.");
        
        // Verify the receiver's balance is now initialized
        let receiver_balance = get_test_balance(receiver);
        assert_eq!(receiver_balance, 500, "Receiver's balance should be initialized to 500.");
    }
}
