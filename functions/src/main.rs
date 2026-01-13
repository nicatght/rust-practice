fn main() {
    println!("main function.");

    another_function();
    another_function_with_param(10);

    let x = expression();
    println!("the result of expression is {x}");

    // 觸發 bug，因為 ; 會讓 5 從 expression 變成 statement  
    // let y = statement();
    // println!("the result of statement is {y}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("Another function with parameter.");
    println!("The value of x is {x}");
}

fn expression() -> i32 {
    /*  
     *  The difference between statement and expression is whether they return a value or not.
     *  
     *  Expression (Like the function) will return 'something'
     *  Statement (Like fn, let) will do something but no return value
     */
     5
}

// fn statement() -> i32 {
//     5;
// }
