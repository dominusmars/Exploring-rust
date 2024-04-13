fn main() {

    // adding mut makes it so the value of value can change. without it, default is immutable
    let mut value = 40;

    for i in 0..5 {
        println!("the number is {}", value + i);
        value += 1;
    }
    // conditions do not require brackets
    // just like js, semi colon doesn't matter too much 
    for i in 0..10{
        if i % 2 == 0{
            println!("even {}", i)
        } else{
            println!("odd {}", i)

        }
    }

    // Ternary Operation can also be written as 
    for i in 0..10{
        let is_even = if i % 2 == 0 {"Even"} else {"Odd"};
        println!("{} is {}",i, is_even);
    }

    let mut sum = 0;

    for i in 0..10{
        sum += i;
    }
    println!("Sum is {}", sum);

    let mut sum = 0.0;

    for i in 0..10{
        // changing data types simpler to typescript
        sum += i as f64;
    }
    println!("Sum is {}", sum)

}
