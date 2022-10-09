pub struct Payment {
    pub id: i32,
    pub amount: i32,
    pub description: String,
}

impl Payment {
    pub fn send(&self) {
        if !self.validate() {
            return println!("Payment is not valid");
        }
        println!(
            "Sending payment: {:?} of ${}",
            self.description, self.amount
        );
    }

    fn validate(&self) -> bool {
        self.amount > 0
    }
}
