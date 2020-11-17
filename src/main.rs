use crate::shape::ShapeTools;

mod traffic_light;
mod shape;
fn main() {
    //第一题 traffic light consonle
    println!("第一题:");
    println!("red traffic light  time: {}", traffic_light::LightColor::light_show_time(traffic_light::LightColor::Red));

    //第二题
    println!("第二题:");
    let params = vec![234,23432535,4366];
    match sum(&params) {

        Some(v) =>println!("total value: {} " , v),
        None =>println!("overflow")
    }

    //第三题
    let rectangle_shape = shape::Rectangle{
        width: 20,
        high: 76
    };
    let square_shape = shape::Square{
        side_length:80
    };
    let triangle_shape = shape::Triangle{
        high:12,
        bottom_edge:17
    };

    println!("第三题:");
    print_area(rectangle_shape);
    print_area(square_shape);
    print_area(triangle_shape);

}

//第三题
fn print_area<T:ShapeTools> (s:T){
    match s.area() {
        Some(v) =>println!("area: {}",v),
        None => println!("overflow")
    }

}


//第二题
fn sum(numbers: &[u32])-> Option<u32>{

    let mut total:u32 = 0;

    for val in numbers.iter(){

        match total.checked_add(*val){
            Some(v)=> {total = v},
            None => {return None}
        }
    }
    Some(total)
}


