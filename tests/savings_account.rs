use bank::SavingsAccount;

#[test]
fn should_have_a_starting_balance_of_zero() {
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0)
}
