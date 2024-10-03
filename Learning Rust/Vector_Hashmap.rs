use std::collections::HashMap;
fn main(){
    //Vector
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    let vec1 = vec![1,2,3,4];
    println!("{:?}", vec);
    println!("{:?}",vector_even(&vec));
    println!("{:?}", vec);

    //Hashmap
    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("Suraj"),21);
    hashmap.insert(String::from("Ram"),31);
    println!("{:?}", hashmap);
    let age =hashmap.get(&String::from("Suraj"));
    match age{
        Some(T) => println!("Age is {:?}",&age),
        None => println!("None"),
    }
    //Hashmap problem
    let pairs : Vec<(String,i32)> = vec![
        (String::from("Suraj"),21),
        (String::from("kotesh"),20),
        (String::from("Ram"),31),
        (String::from("Kotesh"),30),
    ];
    let grouped_pairs = group_values_by_key(pairs);
    println!("{:?}", grouped_pairs);

}
fn vector_even(vec: &Vec<i32>)->Vec<i32>{
    let mut res = Vec::new();
    for i in vec{
        if i%2 == 0{
            res.push(*i);
        }
    }
    return res;
}
fn group_values_by_key(vec: Vec<(String,i32)>) -> HashMap<String,i32>{
    let mut map = HashMap::new();
    for (key,val) in vec{
       map.insert(key,val+1);
    };
    return map;

}

