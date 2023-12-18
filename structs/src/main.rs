#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn my_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //This is an associated function as it does not &self
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }

}

fn main() {
    let rect1 = (30, 50);
    let rect2 = Rectangle { width:30, height:50 };
    let rect3 = Rectangle {width: 12 , height:12};
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!("we passed the demensions as a tuple and the area is {}",tup_area(rect1));
    println!("rect2 is:{:?}", rect2);
    println!("we passed the demensions as a struct and the area is {}", struct_area(&rect2));
    println!("this is to demonstrate the use of a implementation block on a struct the areas is {}", rect2.my_area());
    println!("This demonstrates a 2nd function in the impl block which sees if another rectangle can fit in ours\n");
    println!("rect3 is: {:?}  does it fit in our rectangle?: {} ", rect3, rect2.can_hold(&rect3) );
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tup_area (demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
