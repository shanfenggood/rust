mod traffic_light;

fn main() {
    println!("red traffic light  time: {}", traffic_light::LightColor::light_show_time(traffic_light::LightColor::Red));
}

