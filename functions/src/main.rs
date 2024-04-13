

// in rust you can leave out the return statement
fn abs(x:f64)-> f64{
    if x > 0.0{
        x
    }else {
        -x
    }
}


fn sqr(x:f64)-> f64{
    return x * x;
}


// pass by ref
fn add_one(x: &i32)-> i32{
    *x +1
}
fn add_one_mut(x: &mut i32){
    *x += 1;
}
fn main(){
    let res = sqr(2.0);
    let res_abs = abs(-2.0);

    println!("square is {}",res);
    println!("abs is {}",res_abs);

    let num = 1;
    let res_one = add_one(&num);
    println!("num is {} {}",num,res_one);

    // to mut a args you must pass mut into the arg even if its mut already
    // note you can pass mut into a function but if it doesn't accept mut. it will not change the value
    let mut mut_num = 1;
    add_one_mut(&mut mut_num);
    println!("mut_num is {}",mut_num);

}
