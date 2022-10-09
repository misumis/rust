#[derive(Debug)]

struct Animal {
    name: String,
    age: u32,
    weight: f32,
    sound: String,
}

struct Drug {
    name: String,
    price: f32,
    strength: u32,
    quantity: u32,
    dosage: f32,
    boxes: u32,
}

impl Drug {
    fn get_price_per_box(&self) -> f32 {
        self.price * self.quantity as f32
    }

    fn get_total_number_of_doses(&self) -> f32 {
        self.quantity as f32 * self.dosage * self.boxes as f32
    }
}

fn main() {
    let mut cat = Animal {
        name: "Fluffy".to_string(),
        age: 3,
        weight: 5.0,
        sound: "meow".to_string(),
    };

    cat.name = "Mittens".to_string();

    println!(
        "{} is {} years old and weighs {} kg and says {}",
        cat.name, cat.age, cat.weight, cat.sound
    );

    let dog = Animal {
        name: "Fido".to_string(),
        sound: "woof".to_string(),
        ..cat
    };

    println!("dog is {:?}", dog);

    println!(
        "{} is {} years old and weighs {} kg and says {}",
        dog.name, dog.age, dog.weight, dog.sound
    );

    let fentanyl = Drug {
        name: "Fentanyl".to_string(),
        strength: 100,
        dosage: 0.1,
        quantity: 15,
        boxes: 10,
        price: 426.90,
    };

    println!(
        "{} is {} mg and is sold in {} boxes of {}. Total number of doses is {}",
        fentanyl.name,
        fentanyl.strength,
        fentanyl.boxes,
        fentanyl.quantity,
        fentanyl.get_total_number_of_doses()
    );

    print!(
        "{} costs ${} per box",
        fentanyl.name,
        fentanyl.get_price_per_box()
    );
}
