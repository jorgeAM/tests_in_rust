pub struct SavingsAccount {
    balance: u32,
}

impl SavingsAccount {
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }

    pub fn get_balance(&self) -> u32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount
    }
}

#[cfg(test)]
mod tests {
    use super::SavingsAccount;

    #[test]
    fn should_have_a_starting_balance_of_zero() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0)
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100)
    }
}
