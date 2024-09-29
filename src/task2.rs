#[test]

/*fn main() {
    print();
}

// Replace i32 with another type
fn print() -> i32 {
    println!("Success!");
}*/

fn main() {
    print();
}

// Replace i32 with another type
fn print() -> () {
    println!("Success!");
}

/*Теперь функция print возвращает тип (), что указывает на отсутствие возвращаемого значения.
Функция выполняет действие println! и возвращает (), что соответствует ее новому типу.*/