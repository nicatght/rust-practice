fn main() {
    let x = 3;

    if x == 5 {
        println!("X is equal to 5");
    } else {
        println!("X is not equal to 5");
    }

    // 注意: if 後面必須接 bool，否則會直接 compiler error

    // if x {
    //     println!("Invalid condition");
    // }

    // 'if' is an expression
    // 下面操作 必須確保兩者皆是同個 type (有回傳值)，必須知道 number 的 type
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // we can label the loop
    // 記得要放 '
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // 兩種 print array 方法利用 loop

    let a = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("element: {element}")
    }


}
