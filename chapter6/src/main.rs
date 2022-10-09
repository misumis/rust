#[derive(Debug)]

enum BillingType {
    Prepaid,
    Postpaid,
}

struct Customer {
    name: String,
    billing_type: BillingType,
    phone_number: String,
}

// Generic enum

enum Option<T> {
    None,
    Some(T),
}

// Implementation for generic enum
impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::None => panic!("Called Option::unwrap() on a None value"),
            Option::Some(val) => val,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let user1 = Customer {
        name: "John Doe".to_string(),
        billing_type: BillingType::Prepaid,
        phone_number: "1234567890".to_string(),
    };

    let postpaid_cust = Customer {
        name: "Jane Doe".to_string(),
        billing_type: BillingType::Postpaid,
        phone_number: "0987654321".to_string(),
    };

    let some_number = Some(4);
    let some_string = Some("a string");

    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("{}", some_number.unwrap());

    println!("{}", value_in_cents(Coin::Dime));
}
