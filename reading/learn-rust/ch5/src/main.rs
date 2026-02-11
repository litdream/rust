fn main() {
    basic();
    method1();
    
}

fn basic() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        func_area(width1, height1)
    );

}
fn func_area(width: u32, height: u32) -> u32 {
    width * height
}

//

#[derive(Debug)]
struct Retangle {
    width: u32,
    height: u32,
}
impl Retangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }
}

fn method1() {
    let rect1 = Retangle {
	width: 30,
	height: 50,
    };

    println!(
	"The area of the rectangle is {} square pixels.",
	rect1.area()
    );
}
