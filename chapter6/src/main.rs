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

enum Animal {
    Dog { name: String, sound: String },
    Cat { name: String, sound: String },
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

    let dog = Animal::Dog {
        name: "Fido".to_string(),
        sound: "woof".to_string(),
    };

    let cat = Animal::Cat {
        name: "Fluffy".to_string(),
        sound: "meow".to_string(),
    };

    println!("cat is {:?}", cat);
}
