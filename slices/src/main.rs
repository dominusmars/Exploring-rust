fn main() {
    let ints = [2,3,55];
    let floats = [1.0,12.0,23.12];
    let strings = ["HELLOOOOO", "WORLDSS"];
    let double_ints = [[1,2], [23,42]];


    // the use of {:?} allows you to print out the entire array
    // without gives error `[{integer}; 3]` doesn't implement `std::fmt::Display`
    println!("ints {:?}",ints);
    println!("floats {:?}",floats);
    println!("strings {:?}",strings);
    println!("double_ints {:?}",double_ints);
    

    println!("Hello, world!");
}
