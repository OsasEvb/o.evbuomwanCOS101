fn main() {
    let num1 = 10;
    let num2 = 2;
    let mut result:i32;

    result = num1 - num2;
    println!("Sum: {} ",result);

    result = num1 - num2;
    println!("Diiference: {} ",result);

    result = num1*num2;
    println!("Product: {} ",result);

    result = num1/num2;
    println!("Quotient: {} ",result);

    let _ = result - num1%num2 ;
    println!("Remainder: {}",result);
}