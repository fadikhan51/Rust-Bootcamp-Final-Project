use std::collections::HashMap;
use std::io::{self, Write};

// Product structure
#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

// Inventory structure
struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            products: HashMap::new(),
        }
    }
    fn add_product(&mut self, name_param: String, description: String, price: f64, quantity: u32) {
        let product = Product {
            name: name_param.clone(),
            description,
            price,
            quantity,
        };
        self.products.insert(name_param.clone(), product);
        println!("Product '{}' added to the inventory.", name_param);
    }

    fn edit_product(&mut self, name: &str, price: f64, quantity: u32) {
        if let Some(product) = self.products.get_mut(name) {
            product.price = price;
            product.quantity = quantity;
            println!("Product '{}' edited successfully.", name);
        } else {
            println!("Product '{}' not found in the inventory.", name);
        }
    }

    fn delete_product(&mut self, name: &str) {
        if let Some(_) = self.products.remove(name) {
            println!("Product '{}' deleted from the inventory.", name);
        } else {
            println!("Product '{}' not found in the inventory.", name);
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        println!(
            "{:<15} {:<20} {:<10} {:<10}",
            "Name", "Description", "Price", "Quantity"
        );
        println!("{}", "-".repeat(55));

        for product in self.products.values() {
            println!(
                "{:<15} {:<20} {:<10} {:<10}",
                product.name, product.description, product.price, product.quantity
            );
        }
        println!("\n{}\n", "-".repeat(55));
    }

    fn find_product(&self, name: &str) -> bool {
        self.products.contains_key(name)
    }
}

fn authenticate() -> bool {
    // Basic authentication using a hard-coded username and password, dynamic approach could also be implemented
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let username = read_line();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_line();

    username == "admin" && password == "admin"
}

fn main() {
    // Authenticate the user
    if !authenticate() {
        println!("Authentication failed. Exiting Inventory Management System.");
        return;
    }

    println!("\nLogin Success! Welcome to IMS\n");

    let mut inventory = Inventory::new();

    loop {
        println!("1. Add Product\n2. Edit Product\n3. Delete Product\n4. Generate Report\n5. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                // Add Product
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                let name = read_line();

                print!("Enter product description: ");
                io::stdout().flush().unwrap();
                let description = read_line();

                print!("Enter product price: ");
                io::stdout().flush().unwrap();
                let price: f64 = loop {
                    match read_line().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Invalid price format. Please enter a valid number."),
                    }
                };

                print!("Enter product quantity: ");
                io::stdout().flush().unwrap();
                let quantity: u32 = loop {
                    match read_line().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Invalid quantity format. Please enter a valid number."),
                    }
                };

                inventory.add_product(name, description, price, quantity);
            }
            2 => {
                // Edit Product
                print!("Enter product name to edit: ");
                io::stdout().flush().unwrap();
                let name = read_line();

                if inventory.find_product(&name) {
                    print!("Enter new price: ");
                    io::stdout().flush().unwrap();
                    let price: f64 = loop {
                        match read_line().parse() {
                            Ok(num) => break num,
                            Err(_) => {
                                println!("Invalid price format. Please enter a valid number.")
                            }
                        }
                    };

                    print!("Enter new quantity: ");
                    io::stdout().flush().unwrap();
                    let quantity: u32 = loop {
                        match read_line().parse() {
                            Ok(num) => break num,
                            Err(_) => {
                                println!("Invalid quantity format. Please enter a valid number.")
                            }
                        }
                    };

                    inventory.edit_product(&name, price, quantity);
                } else {
                    println!("The product you entered does not exist in inventory. Try adding the product first.")
                }
            }
            3 => {
                // Delete Product
                print!("Enter product name to delete: ");
                io::stdout().flush().unwrap();
                let name = read_line();

                if inventory.find_product(&name) {
                    inventory.delete_product(&name);
                } else {
                    println!("The product you entered does not exist in inventory.")
                }
            }
            4 => {
                // Generate Report
                inventory.generate_report();
            }
            5 => {
                // Exit
                println!("Exiting Inventory Management System.");
                break;
            }
            _ => {
                //Default Option if none matches
                println!("Invalid choice. Please enter a number between 1 and 5.");
            }
        }
        
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
