fn main() {
    let coffee_price = 5.99;

    {
        let coffee_price = 1.99;
        println!("The coffee price is {coffee_price}");
    }

    println!("The coffee price is {coffee_price}");
}
