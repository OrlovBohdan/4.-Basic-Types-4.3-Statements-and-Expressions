#[test]

/*
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {

}*/


fn main() {
    println!("Success!");

    let result = get_option(2);
    println!("{:?}", result);
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42), // Например, возвращаем 42, если tp равно 1.
        _ => {
            // Вызываем функцию, которая не возвращает значение.
            never_return_fn();
        }
    }
}

/*// Способ 1: Используя panic!
fn never_return_fn() -> ! {
    panic!("This function never returns!");
}*/

// Или

// Способ 2: Используя бесконечный цикл

fn never_return_fn() -> ! {
    loop {}
}


// Или

// Способ 3: Используя std::process::exit

/*use std::process;

fn never_return_fn() -> ! {
    process::exit(1);
}*/

/*Способ 1 завершает программу с помощью паники, и функция не возвращает значение.
Способ 2 использует бесконечный цикл, который никогда не выходит из функции.
Способ 3 завершает программу с указанным кодом выхода, также не возвращая значение.*/