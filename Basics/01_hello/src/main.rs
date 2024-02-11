fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let num: i32 = 24;
    let name: &str = "Suryanshu Tomar";
    let price = 25.00;
    let discounted_price = 0.00; 

    println!("Hello, world!");
    println!("This is a variable {num}");
    println!("My name is {name:?}. And this print statement is printing in debug mode.");

    println!("The price of the course is {}", price);
    println!("The price after discount will be {:?}", discounted_price);

   print_type_of(&num);
   print_type_of(&name);
}
