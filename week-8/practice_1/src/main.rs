fn main() {
    // Using Vec::new()
    let cv : Vec<i64> = Vec::new();

    // Printing the size of vextor
    println!("\nThe length of Vec::new is {}",cv.len());

    //using macro 
    let v = vec!["Grace", "Effiong","Basil", "Kareem", "Susan"];

    // printing the size of vector
    println!("\nThe length of vec macro is: { }",v.len());


}