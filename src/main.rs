use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {note: String},
    Refuse,
    Probation,
}

fn what_is_your_name() -> String {
    let mut your_name: String = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("derek", "Hello Derek, enjoy your treehouse."),
        Visitor::new("fang", "Nihao, Ni chi le ma?"),
        Visitor::new("isaac", "Isaac! It's bouncin' time!"),
    ];
    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name: String = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    println!("The final list of visitors: ");
                    println!("{:#?}", visitor_list);
                    break;
                } else {
                    println!("{} is not on the visitor list. ", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
}
