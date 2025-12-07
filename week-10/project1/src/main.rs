struct Laptop {
    name: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {

    let hp = Laptop {
        name: String::from("HP"),
        price: 650000,
    };

    let dell = Laptop {
        name: String::from("Dell"),
        price: 850000,
    };

    let ibm = Laptop {
        name: String::from("IBM"),
        price:755000,
    };

    let toshiba = Laptop {
        name: String::from("Toshiba"),
        price: 550000,
    };

    let qty:u32 = 3;

    let cost_hp = hp.total_cost(qty);
    let cost_ibm = ibm.total_cost(qty);
    let cost_toshiba = toshiba.total_cost(qty);
    let cost_dell = dell.total_cost(qty);

    let total = cost_hp + cost_ibm + cost_toshiba + cost_dell;

    println!("Total cost for buying {} units of each brand is: {}", qty, total);

}