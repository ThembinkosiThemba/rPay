use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
struct Account {
    id: u64,
    balance: i64,
}

#[derive(Debug)]
struct Transaction {
    from: u64,
    to: u64,
    amount: i64,
    timestamp: u64,
}

struct PaymentSystem {
    accounts: Arc<Mutex<HashMap<u64, Account>>>,
    transaction_log: Arc<Mutex<Vec<Transaction>>>,
}

impl PaymentSystem {
    fn new() -> Self {
        PaymentSystem {
            accounts: Arc::new(Mutex::new(HashMap::new())),
            transaction_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn create_account(&self, id: u64, initial_balance: i64) -> Result<(), String> {
        let mut account = self.accounts.lock().unwrap();
        if account.contains_key(&id) {
            return Err("Account already exists".to_string());
        }
        account.insert(
            id,
            Account {
                id,
                balance: initial_balance,
            },
        );
        Ok(())
    }

    fn transfer(&self, from: u64, to: u64, amount: i64) -> Result<(), String> {
        let mut accounts = self.accounts.lock().unwrap();

        // Check if both accounts exist and get their current balances
        let from_balance = accounts.get(&from).ok_or("From account not found")?.balance;
        let _to_balance = accounts.get(&to).ok_or("To account not found")?.balance;

        if from_balance < amount {
            return Err("Insufficient funds".to_string());
        }

        // Update balances
        accounts.get_mut(&from).unwrap().balance -= amount;
        accounts.get_mut(&to).unwrap().balance += amount;

        let transaction = Transaction {
            from,
            to,
            amount,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.transaction_log.lock().unwrap().push(transaction);

        Ok(())
    }

    fn get_balance(&self, id: u64) -> Result<i64, String> {
        let accounts = self.accounts.lock().unwrap();
        accounts
            .get(&id)
            .map(|account| account.balance)
            .ok_or_else(|| "Account not found".to_string())
    }
}

fn main() {
    let system = PaymentSystem::new();
    system.create_account(1, 20000).unwrap();
    system.create_account(2, 10000).unwrap();

    system.transfer(1, 2, 4000).unwrap();

    println!("transfer complete");
    println!("Account 1 balance: {}", system.get_balance(1).unwrap());
    println!("Account 2 balance: {}", system.get_balance(2).unwrap());
}
