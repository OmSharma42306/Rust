// 💵 Bank Account Manager (struct Account, deposit, withdraw, ownership concepts)

#[derive(Debug)]
struct Account{
    account_no : String,
    account_name : String,
    account_balance : i32,
}

// deposit Feature.

fn deposit(from_account_no:String,to_account_no:String,_amount:i32,_bank : &mut Vec<Account>){
    let mut sender_account:Option<&mut Account> = None ;
    let mut receiver_account:Option<&mut Account> = None;
    
    for account in _bank{
        if account.account_no == from_account_no{
            sender_account = Some(account);
        }else if account.account_no == to_account_no{
            receiver_account = Some(account);
        }
    };

    
    println!("Sender Accountx  : {:?}",sender_account);
    
    println!("Receiver Accountx  : {:?}",receiver_account);
    
    match(sender_account,receiver_account){
        (Some(sender),Some(receiver))=>{
            if sender.account_balance >= _amount{
                
                sender.account_balance -= _amount;
                receiver.account_balance += _amount;
                println!("From {} A/c to {} A/c Amount Deposited Successfully! ",sender.account_name,receiver.account_name);
            }
            else {
                println!("Insufficient Balance at Your Account.{}",sender.account_name);
            }
        }
        _ => {
                println!("Accounts not Found!");
        }
        
    }

}

// withdraw feature.

fn withdraw(account_no : String,amount:i32, bank : &mut Vec<Account>){
    let mut flag : bool = false;
    for account in bank{
        if account.account_no == account_no && account.account_balance >= amount{
            account.account_balance -= amount;
            println!("Rs:{} Withdrawn from Your A/c.{} Successfully!.",amount,account_no);
            flag = true;
            break;
        }
    }
    if flag == false{
        println!("Account Not Found!");
    }
    
    

}

fn add_account_to_bank(_bank : &mut Vec<Account>,account : Account){
    // _bank.push(account);
    println!("{:?}",account);
    _bank.push(account);
}
fn main(){
    
    // the Store of All Account of People
    let mut state_bank_of_india: Vec<Account> = vec![];

    // creating Accounts in Bank of People.
    
    // user 1
    let om_sharma = Account{
        account_no : String::from("124920293"),account_name : String::from("Om Sharma"),account_balance:910842
    };

    // user 2

    let sagar_sharma:Account = Account { account_no: String::from("132412443"), account_name:String::from("Sagar Sharma"), account_balance: 3124 };

    add_account_to_bank(&mut state_bank_of_india,   om_sharma);
    add_account_to_bank(&mut state_bank_of_india,   sagar_sharma);
    
    println!("{:?}",state_bank_of_india);

    // deposit(String::from("124920293"),String::from("132412443"), 500,&mut state_bank_of_india);
    withdraw(String::from("124920293"),90000,&mut state_bank_of_india);
    println!("{:?}",state_bank_of_india);



}