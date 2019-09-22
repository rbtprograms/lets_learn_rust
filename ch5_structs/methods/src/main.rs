#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//this is how you add a method onto a struct
//known as an implementation block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height 
    }
    //this is an assocaited function, the above two are methods. Called such because
    //it does not reference self but is on the struct implementation
    fn create_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 20 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!(
        "can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );
    println!(
        "can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );
    let sq = Rectangle::create_square(30);
    println!("{:#?}", sq);
}
