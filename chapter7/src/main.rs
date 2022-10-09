use crate::models::payments::Payment;

pub mod models;

fn main() {
    let payment = Payment {
        id: 1,
        amount: 100,
        description: String::from("test"),
    };

    payment.send();

    let payment2 = Payment {
        id: 2,
        amount: -100,
        description: String::from("Payment 2"),
    };

    payment2.send();
}
