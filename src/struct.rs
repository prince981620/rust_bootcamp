struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        2*(self.width+self.height) //statement withut ; is considered as return
    }
}

fn main(){
    let rect = Rect {
        width: 30,
        height: 40
    };
    println!("The area of Rectangle is {}",rect.area());
    println!("The perimeter of Rectangle is {}",rect.perimeter());
}