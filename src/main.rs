fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Max (i32): {:?}", find_max(&numbers));

    let floats = vec![1.1, 2.2, 3.3, 0.5];
    println!("Max (f64): {:?}", find_max(&floats));

    // TODO For problem 10
    let person = Person {
        name: "Alex".to_string(),
        age: 21,
    };

    let animal = Animal {
        nickname: "Negr".to_string(),
        species: "Nigeria".to_string(),
    };

    let car = Car {
        brand: "Toyota".to_string(),
        color: "red".to_string(),
        number: 123,
    };

    print_describtion(person);
    print_describtion(animal);
    print_describtion(car);
}

// fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
//     if slice.is_empty() {
//         return None;
//     }

//     let mut max = &slice[0]
// }
// !learning

// ? Задача 10
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }

    let mut max = &slice[0];

    for item in slice.iter().skip(1) {
        if item > max {
            max = item;
        }
    }

    Some(max)
}

// ? Задача 10
trait Describe {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: i32,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("Name: {}, age: {}", self.name, self.age)
    }
}

struct Animal {
    nickname: String,
    species: String,
}

impl Describe for Animal {
    fn describe(&self) -> String {
        format!("Nickname: {}, species: {}", self.nickname, self.species)
    }
}

struct Car {
    brand: String,
    color: String,
    number: i32,
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!(
            "Brand: {}, color: {}, number: {}",
            self.brand, self.color, self.number
        )
    }
}

fn print_describtion<T: Describe>(item: T) {
    println!("{}", item.describe());
}
