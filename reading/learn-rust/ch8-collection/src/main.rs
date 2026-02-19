use std::collections::HashMap;

fn func1() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
	Some(third) => println!("The third elm is {third}"),
	None => println!("No third??"),
    }
}

fn func2() {
    let v = vec![1,2,3,4,5];
    if let Some(third) = v.get(2) {
	println!("The third elm is {third}");
    } else {
	println!("No third??")
    }

}

/**
fn err_why() {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];

    v.push(6);   // vector copy happens here.
    
    println!("The first elm is: {first}");    
}
 */

/**
fn err_fix() {
    let mut v = Vec::with_capacity(10);
    v.extend([1,2,3,4,5]);
    
    let first = &v[0];
    v.push(6);   // vector won't copy.  But, still  mut/immut mixed.
    println!("The first elm is: {first}");    
}

 */

fn hashmap_own() {
    let mut map1 = HashMap::new();    // Str:i32

    let time = String::from("Morning");
    let hr = 8;
    map1.insert(time, hr);

    // - Note that:  time is not available.
    // println!("I wake up {hr} in the {time}");
    println!("I wake up {hr}.");

    // Similarly
    let mut map2 = HashMap::new();    // Str:Str
    let time = String::from("Afternoon");
    let hr = String::from("three");
    map2.insert(time, hr);    // Now, map2 owns both key,value (time,hr).

    // - Nothing is possible.
    // println!("I come back at {hr} in the {time}")
	
}

fn main() {
    func1();
    println!("---------");
    func2();
    println!("---------");

    // mixing mut and const(immut).
    // err_why();

    // Then, fix this?
    // err_fix();

    hashmap_own();
}
