fn discount_calculator(day_of_month: u8) {

    let amount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };

    println!("Congrats, you got {}% discount!", amount)
}

fn main() {

    discount_calculator(10);
}
