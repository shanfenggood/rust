pub trait ShapeTools{
    fn area(&self)->Option<u32>;
}

pub struct Rectangle{
    pub width:u32,
    pub high:u32
}
impl ShapeTools for Rectangle{
    fn area(&self)->Option<u32>{
        self.width.checked_mul(self.high)
    }
}

pub struct Square{
    pub side_length:u32
}
impl ShapeTools for Square{
    fn area(&self)->Option<u32>{
        self.side_length.checked_mul(self.side_length)
    }
}

pub struct Triangle{
    pub bottom_edge: u32,
    pub high:u32
}

impl ShapeTools for Triangle{
    fn area(&self)->Option<u32>{
        match self.bottom_edge.checked_mul(self.high) {
            Some(ret) => ret.checked_div(2),
            None => None
        }
    }
}