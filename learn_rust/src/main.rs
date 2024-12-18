macro_rules! greet {
    ($name: expr) =>{
        let name = $name;
        println!("Hello, {0}!", name);
    }
}

fn main() {
    println!("Hello, world!");
    println!("-----------------------------------------");

    use_optionenum_iterator_patternmatching();
    println!("-----------------------------------------");

    sum_product_type_example();
    println!("-----------------------------------------");

    nested_enum_example();
    println!("-----------------------------------------");

    greet!("John");
    greet!("Jane");
    println!("-----------------------------------------");
}

fn use_optionenum_iterator_patternmatching() {
    let numbers = vec![5, 2, 8, 1, 3, 7];

    let max_element = numbers.iter().max();

    match max_element {
        Some(&max) => println!("The maximum element is {0}", max),
        None => println!("The vector is empty"),
    }
}

enum Message {
    Text(String),
    Photo { url: String, size: u32 },
    Video { url: String, duration: u32 },
    Reaction(char),
}

fn sum_product_type_example() {
    process_msg(Message::Text(String::from("Hello")));
    process_msg(Message::Photo {
        url: String::from("https://example.com/photos"),
        size: 270,
    });
    process_msg(Message::Video {
        url: String::from("https://example.com/videos"),
        duration: 10,
    });
    process_msg(Message::Reaction('👍'));
}

fn process_msg(msg: Message) {
    match msg {
        Message::Text(content) => {
            println!("The content is {0}", content);
        }
        Message::Photo { url, size } => {
            println!("The url is {0} and the size is {1}", url, size);
        }
        Message::Video { url, duration } => {
            println!("The url is {0} and the duration is {1}", url, duration);
        }
        Message::Reaction(reaction) => {
            println!("The reaction is {0}", reaction);
        }
    }
}

fn nested_enum_example() {
    enum Employee {
        Manager {
            name: String,
            employees: Vec<Box<Employee>>,
        },
        Worker {
            name: String,
            manager: String,
        },
    }

    let employee = Employee::Worker {
        name: String::from("John"),
        manager: String::from("Jane"),
    };

    match employee {
        Employee::Manager { name, employees } => {
            println!("The manager's name is {0}", name);
            println!("The manager has {0} employees", employees.len());
        }
        Employee::Worker { name, manager } => {
            println!("The worker's name is {0}", name);
            println!("The worker's manager is {0}", manager);
        }
    }
}


