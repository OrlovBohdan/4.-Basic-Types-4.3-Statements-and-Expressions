#[test]
/*
// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures

}*/
/* //1st
// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Не должно быть напечатано!");
}

fn never_return() -> ! {
    panic!("Эта функция никогда не возвращается!"); // Вызывает панику, предотвращая дальнейшее выполнение.
}*/

//2
fn main() {
    never_return();

    //println!("Не должно быть напечатано!");
}

fn never_return() -> ! {
    loop {} // Бесконечный цикл, который никогда не завершится, предотвращая дальнейшее выполнение.
}

/*В Способе 1 вызов panic! завершит программу, и строка println!("Не должно быть напечатано!"); не будет выполнена.
В Способе 2 бесконечный цикл предотвратит выполнение строки println!, эффективно удерживая программу в работе бесконечно.*/
