use app::token_wallet::Wallet;

#[test]
fn test_send_tokens() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    alice.receive_tokens(100);  // Add initial balance to Alice
    alice.send_tokens(50, &bob).unwrap();  // Send tokens

    assert_eq!(alice.get_balance(), 50);
    assert_eq!(bob.get_balance(), 50);
}

#[test]
fn test_receive_tokens() {
    let wallet = Wallet::new();
    wallet.receive_tokens(100);
    assert_eq!(wallet.get_balance(), 100);
}

#[test]
fn test_insufficient_funds() {
    let alice = Wallet::new();
    let bob = Wallet::new();

    alice.receive_tokens(30);
    assert!(alice.send_tokens(50, &bob).is_err());
}
