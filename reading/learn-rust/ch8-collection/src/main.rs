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

fn err_fix() {
    let mut v = Vec::with_capacity(10);
    v.extend([1,2,3,4,5]);
    
    let first = &v[0];
    v.push(6);   // vector won't copy.  But, still  mut/immut mixed.
    println!("The first elm is: {first}");    
}

fn main() {
    func1();
    println!("---------");
    func2();
    println!("---------");

    // mixing mut and const(immut).
    // err_why();

    // Then, fix this?
    err_fix();
}
