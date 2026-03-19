use std::ops::Deref;


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
	MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0    // returns the address
    }
}

fn main() {
    let x =5;
    let y = MyBox::new(x);
    let z = *y;     // *y is same as *(y.deref())
    
    print!("x value:  {x}\n");
    print!("y deref:  {z}\n");

}
