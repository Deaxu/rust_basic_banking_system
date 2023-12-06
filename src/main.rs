use std::io;
fn main() {
    let mut my_account = BankAccount{
        account_number: 100,
        holder_name: String::from("Emre"),
        balance: 14000,
    };

    let mut account1 = BankAccount{
        account_number: 101,
        holder_name: String::from("Mehmet"),
        balance: 17500,
    };

    let mut account2 = BankAccount{
        account_number: 102,
        holder_name: String::from("Burak"),
        balance: 10000,
    };

    let mut account3 = BankAccount{
        account_number: 103,
        holder_name: String::from("Umut"),
        balance: 15000,
    };

    loop {
        println!("Hesap numaraniz: {}, Adiniz: {} ,Bakiye: {}",my_account.account_number, my_account.holder_name,my_account.balance);

    
        println!("
        1)Yatirma: 
        2)Çekim:
        3)Transfer:
        Diğer hesap nolari: 101, 102, 103
        Yapilacak islemi secin:");
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("");
        let choice: u32 = choice.trim().parse().expect("Invalid input");
    
        match choice {
            1 => {
                println!("Yatirilacak miktar:");
                let amount = String::new();
                let amount = take_input(amount);
                let amount = parse_u32(amount);
    
                my_account.deposit(amount);
                println!("Yeni bakiye: {}", my_account.balance());
            }
            2 => {
                println!("Cekilecek miktar: ");
                let amount = String::new();
                let amount = take_input(amount);
                let amount = parse_u32(amount);
    
                my_account.withdraw(amount);
                println!("Yeni bakiye: {}", my_account.balance());
            }
            3 => {
                println!("Transfer yapilacak hesap numarasi:");
                let target_account_number = String::new();
                let target_account_number = take_input(target_account_number);
                let target_account_number = parse_u32(target_account_number);
    
                println!("Transfer miktari giriniz: ");
                let amount = String::new();
                let amount =  take_input(amount);
                let amount = parse_u32(amount);
    
                let target_account = match target_account_number {
                    101 => &mut account1,
                    102 => &mut account2,
                    103 => &mut account3,
                    _ => {
                        println!("Hedef hesap bulunamadi!");
                        return;
                    }
                };
    
                if amount < my_account.balance{
                    my_account.withdraw(amount);
                    target_account.deposit(amount);
                    println!("Transfer basarili. Bakiyeniz: {}",my_account.balance);
                    println!("Transfer yapilan adresin bakiyesi: {}", target_account.balance)
                }
                else{
                    println!("Transfer basarisiz. Yetersiz bakiye!");
                }
            }
            _ => println!("Geçersiz seçenek!"),
        };
        println!("");
    }
}

struct BankAccount{
    account_number: u32,
    holder_name: String,
    balance:u32,
}

trait Account{
    fn withdraw(&mut self, amount:u32) -> u32;
    fn deposit(&mut self, amount:u32) -> u32;
    fn balance(&self) -> u32;
}

impl Account for BankAccount{
    fn withdraw(&mut self, amount: u32) -> u32 {
        if amount <= self.balance{
            self.balance -= amount
        }
        else {
            println!("Bakiyenizden yuksek tutar giremezsiniz!!")
        }
        self.balance
    }
    fn deposit(&mut self, amount: u32) -> u32 {
        self.balance += amount;
        self.balance
    }
    fn balance(&self) -> u32 {
        self.balance
    }
}

fn take_input(mut input: String) -> String{
    io::stdin().read_line(&mut input);
    input
}

fn parse_u32(input: String) -> u32{
    let input: u32 = input.trim().parse().expect("Invalid input");
    input
}