#[test]
/*fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x, y: i32) {
    x + y;
}*/


fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 { // Указан тип возвращаемого значения
    x + y // Возвращаем результат
}

/*В функции sum добавлен тип возвращаемого значения -> i32, что означает, что функция возвращает целое число.
Теперь выражение x + y в конце функции возвращает сумму, и программа будет работать корректно.*/