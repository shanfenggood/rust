#[allow(dead_code)]


pub enum LightColor {
    Red,
    Green,
    Yellow
}
impl LightColor {

    pub fn light_show_time(light: LightColor) -> u8 {

        match light {
            LightColor::Red => 5,
            LightColor::Green => 3,
            LightColor::Yellow => 1
        }
    }

}