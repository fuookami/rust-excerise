trait BankAccount {
    fn name(&self) -> &str;
    fn balance(&self) -> f64;

    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> bool;
}

struct BankAccountBase {
    _name: String,
    _balance: f64,
}

impl BankAccount for BankAccountBase {
    fn name(&self) -> &str {
        &self._name
    }

    fn balance(&self) -> f64 {
        self._balance
    }

    fn deposit(&mut self, amount: f64) {
        self._balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self._balance < amount {
            false
        } else {
            self._balance -= amount;
            true
        }
    }
}

struct MoneyMarketAccount {
    account: BankAccountBase,
    withdraw_time: usize,
}

impl BankAccount for MoneyMarketAccount {
    fn name(&self) -> &str {
        self.account.name()
    }

    fn balance(&self) -> f64 {
        self.account.balance()
    }

    fn deposit(&mut self, amount: f64) {
        self.account.deposit(amount);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.account.balance() < (amount + 1.5) {
            false
        } else {
            self.account.withdraw(amount);
            self.withdraw_time += 1;
            if self.withdraw_time > 2 {
                self.account.withdraw(1.5);
            }
            true
        }
    }
}

struct CDAccount {
    account: BankAccountBase,
    interest_rate: f64,
}

impl BankAccount for CDAccount {
    fn name(&self) -> &str {
        self.account.name()
    }

    fn balance(&self) -> f64 {
        self.account.balance()
    }

    fn deposit(&mut self, amount: f64) {
        self.account.deposit(amount);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if !self.account.withdraw(amount) {
            false
        } else {
            self.account.deposit(amount * self.interest_rate * 0.75);
            true
        }
    }
}

fn main() {
    println!("Hello, world!");
}
