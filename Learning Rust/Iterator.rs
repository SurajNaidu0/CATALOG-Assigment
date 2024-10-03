//Iterators
fn main(){
    //Iterators using for loop
    let mut v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for i in v1_iter{
        println!("{}", i);
    }
    //Iterators using for loop that is mutable
    let v2_iter = v1.iter_mut();
    for i in v2_iter{
        *i = *i + 1;
        println!("{}", i);
    }
    println!("{:?}",v1);
    //Iterators using while loop
    let mut v3_iter = v1.iter();
    while let Some(val) = v3_iter.next(){
        println!("{}", val);
    }
    //Consumer Adaptor
    let v4_iter = v1.iter();
    let sum_1:i32 = v4_iter.sum();
    println!("Sum is {}", sum_1);
    //Iterator Adaptor
    let v5_iter = v1.iter();
    let new_iter = v5_iter.map(|x| x+3);
    let new_iter3 = v1.iter();
    for i in new_iter3{
        println!("{}", i);
    }
    let new_iter1 = new_iter.filter(|x| x % 3 == 0).map(|x| x*x);
    for i in new_iter1{
        println!("hi");
        println!("{}", i);
    }
    //Iterator to a vector
    let v6_iter = v1.iter();
    let new_vec: Vec<i32> = v6_iter.collect();
    println!("{:?}", new_vec);
} 