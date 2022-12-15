// 题目三：实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。
trait Shape {
    fn area(&self) -> f64;             // 定义泛型约束函数，用于计算面积，返回一个f64类型的浮点数
}
 
struct Circle {
    radius: f64, // 半径
}
 
impl Shape for Circle {               // 向Circle实现Shape接口，覆写area函数以计算圆形面积
    fn area(&self) -> f64 {           // 此处需要使用到 self，即当前对象本身。所以fn定义中加入self参数，此处不需要传参数。另外返回值是浮点数f64
    3.14 * self.radius * self.radius  // 返回πR²的浮点值   
    } 
}

struct Rectangle {   // 定义矩形
    width: f64,      // 长度   
    height: f64,     // 宽度 
} 

impl Shape for Rectangle {            // Rectangle对象向Shape接口实现area方法，计算任意2条直角边构成的长方形（rectangle）面积   
    fn area(&self) -> f64 {
        self.width * self.height     
    }     
}      

fn print_shape_area<T>(shape: T) where T: 
    Shape {
        println!("该图形的面积等于:{}",shape.area());     
    }      
        
fn main() {    
    let circle = Circle { radius: 3.0 };   
    let rectangle = Rectangle{ width: 6.4, height: 8.2 };         
    print_shape_area(circle);         
    print_shape_area(rectangle);   
}