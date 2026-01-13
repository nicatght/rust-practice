fn main() {
    // let x = 5; // 報錯 
    let mut x = 5; // 正確，使 x 變 mutable
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is : {x}");

    const TEN: i32 = 10;
    println!("The constant we have in TEN is: {TEN}")

}
