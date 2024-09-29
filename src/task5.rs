#[test]

/*
fn main() {
    // FILL in the blank
    let b = __;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}*/


fn main() {
    // Заполняем пробел значением false
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!"); // Эта строка будет выполнена
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!"); // Эта строка не будет выполнена
}
/*Присваивая переменной b значение false, программа выполнит ветку false в выражении match.
В этой ветке выводится "Success!", после чего программа вызывает панику, что предотвращает дальнейшее выполнение.
Таким образом, строка "Exercise Failed if printing out this line!" не будет выведена, что соответствует запросу.*/