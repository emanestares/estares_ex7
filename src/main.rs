use std::io;

struct Book {
    book_id : u32 ,
    book_title : String ,
    author : String ,
    price : f64 ,
    stock : u32
}

struct Customer {
    name : String ,
    orders : Vec <String> ,
    total_cost : f64
}

fn create_book(id:u32, title:String, author:String, price:f64, stock:u32) -> Book{
    Book{book_id : id, book_title : title, author : author, price : price, stock : stock}
}

fn create_customer(name: String, orders: Vec <String>, total_cost: f64) -> Customer{
    Customer{name:name, orders:orders, total_cost:total_cost}
}

fn view_all_customers(customer_vec:&mut Vec<Customer>){
    for customer in &mut *customer_vec{
        println!("\nCustomer Name: {:?}\n", customer.name);
        for book in &customer.orders{
            println!{"   - {:?}\n", book};
        }
        println!("Total Cost: {}\n", customer.total_cost);
    }

}

fn view_all_books(book_vec:&mut Vec<Book>){
    for book in &mut *book_vec{
        println!{"\nBook ID: {}\nBook Title: {:?}\nAuthor: {:?}\nPrice: {}\nStock: {}\n",  book.book_id, book.book_title, book.author, book.price, book.stock};
    }
}

fn delete_book_information(book_vec:&mut Vec<Book>){
    let mut id = String::new();
    let mut id_doesnt_exists = true;
    let mut index = 0;

    println!("Enter book ID:");
    io::stdin().read_line(&mut id).expect("Error");
    let id:u32 = id.trim().parse().expect("Error");

    for book in &mut *book_vec{
        if book.book_id == id{
            id_doesnt_exists = false;
            book_vec.remove(index);
            println!("Successfully deleted boook information!");
            break;
        }
        index +=1;
    }

    if id_doesnt_exists {
        println!("Book ID does not exist!");
    }
}

fn edit_book_information(book_vec:&mut Vec<Book>){
    let mut id = String::new();
    let mut price = String::new();
    let mut stock = String::new();
    let mut id_doesnt_exists = true;

    println!("Enter book ID:");
    io::stdin().read_line(&mut id).expect("Error");
    let id:u32 = id.trim().parse().expect("Error");

    for book in &mut *book_vec{
        if book.book_id == id{
            id_doesnt_exists = false;
            println!("Update price:");
            io::stdin().read_line(&mut price).expect("Error");
            let price:f64 = price.trim().parse().expect("Error");

            println!("Update stock:");
            io::stdin().read_line(&mut stock).expect("Error");
            let stock:u32 = stock.trim().parse().expect("Error");

            book.price = price;
            book.stock = stock;

            println!("Book Informatin Successfully Edited!");
            break;
        }
    }

    if id_doesnt_exists {
        println!("Book ID does not exist!");
    }
}

fn buy_a_book(book_vec:&mut Vec<Book>, customer_vec:&mut Vec<Customer>){
    let mut name = String::new();
    let mut id = String::new();
    let mut customer_orders:Vec <String> = Vec::new();
    let mut id_doesnt_exists = true;
    let mut customer_already_exists = false;
    let mut customer_index = 0;

    println!("Enter customer name:");
    io::stdin().read_line(&mut name).expect("Error");
    name.pop();

    for customer in &mut *customer_vec{
        if customer.name == name{
            customer_already_exists = true;
            break
        }
        customer_index+=1;
    }

    println!("------- BOOKS AVAILABLE -------");
    for book in &mut *book_vec{
        println!{"   [{}] {:?} by {:?} - {}\n", book.book_id, book.book_title, book.author, book.price};
    }

    println!("Enter book ID to buy:");
    io::stdin().read_line(&mut id).expect("Error");
    let id:u32 = id.trim().parse().expect("Error");

    for book in &mut *book_vec{
        if book.book_id == id {
            let order_string = format!("{}_{:?}_{:?}", id, book.book_title, book.author);
            id_doesnt_exists = false;

            if book.stock == 0{
                println!("Book is out of stock!");
                break;
            }

            if customer_already_exists {
                customer_vec[customer_index].orders.push(order_string);
                customer_vec[customer_index].total_cost += book.price;
            } else {
                customer_orders.push(order_string);
                customer_vec.push(create_customer(name, customer_orders, book.price));
            }
            let order_string = format!("{}_{:?}_{:?}", id, book.book_title, book.author);
            book.stock -= 1;
            println!("Successfully order book {:?}", order_string);
            break;
        }
    }

    if id_doesnt_exists{
        println!("Book ID not found!");
    }
}

fn add_book_information(book_vec:&mut Vec<Book>){
    let mut id = String::new();
    let mut id_doesnt_exists = true;

    println!("Enter book id:");
    io::stdin().read_line(&mut id).expect("Error");
    let id:u32 = id.trim().parse().expect("Error");

    for book in &mut *book_vec {
        if id == book.book_id{
            println!("Book ID already exists!");
            id_doesnt_exists = false;
            break;
        }
    }

    if id_doesnt_exists{
        let mut title = String::new();
        let mut author = String::new();
        let mut price = String::new();
        let mut stock = String::new();

        println!("Enter book title:");
        io::stdin().read_line(&mut title).expect("Error");
        title.pop();

        println!("Enter author:");
        io::stdin().read_line(&mut author).expect("Error");
        author.pop();

        println!("Enter price:");
        io::stdin().read_line(&mut price).expect("Error");
        let price:f64 = price.trim().parse().expect("Error");

        println!("Enter stock:");
        io::stdin().read_line(&mut stock).expect("Error");
        let stock:u32 = stock.trim().parse().expect("Error");

        book_vec.push(create_book(id, title, author, price, stock));
        println!("Successfully added book!");
    }
}

fn menu() -> isize {
    let mut choice = String::new();

    println!("============ MENU ============");
    println!("[1] Add Book Information");
    println!("[2] Buy a Book");
    println!("[3] Edit Book Information");
    println!("[4] Delete Book Information");
    println!("[5] View All Books");
    println!("[6] View All Customers");
    println!("[7] Exit\n");
    println!("Enter choice:");
    io::stdin().read_line(&mut choice).expect("Error");

    let choice:isize = choice.trim().parse().expect("Error");
    choice
}

fn main() {
    let mut customer_vector:Vec<Customer> = Vec::new();
    let mut book_vector:Vec<Book> = Vec::new();
    let mut choice = 0;
    
    while choice != 7{
        choice = menu();

        if choice == 1 {
            add_book_information(&mut book_vector);
        } else if choice ==2 {
            if book_vector.len() == 0{
                println!("There are no books available!");
            }else{
                buy_a_book(&mut book_vector, &mut customer_vector);
            }

        } else if choice == 3 {
            if book_vector.len() == 0{
                println!("There are no books available!");
            }else{
                edit_book_information(&mut book_vector);
            }

        } else if choice == 4 {
            if book_vector.len() == 0{
                println!("There are no books available!");
            }else{
                delete_book_information(&mut book_vector);
            }

        } else if choice == 5{
            if book_vector.len() == 0{
                println!("There are no books available!");
            }else{
                view_all_books(&mut book_vector);
            }

        } else if choice == 6{
            if book_vector.len() == 0{
                println!("There are no books available!");
            }else{
                view_all_customers(&mut customer_vector);
            }
            
        } else if choice == 7 {
            println!("Thank you for shopping, goodbye!");
        }else{
            println!("Invalid input\n");
        }
    }
}
