extern crate core;
use Shape::*;

pub struct Vector2d{
    pub x:f32,
    pub y:f32
}
pub struct Circle{x:f32, y:f32, r:f32}
pub struct AABB{x:f32, y:f32, rx:f32, ry:f32}
pub struct Rect{x:f32, y:f32, rx:f32, ry:f32, a:f32}

pub enum Shape{
    Circle(Circle),
    AABB(AABB),
    Rect(Rect),
}

/// sier om to former er inni hverandre
pub fn intersection(a:Shape, b:Shape)->bool{
    match (a, b) {
        (Circle(a), Circle(b))=>{
            todo!();
        }
        (Circle(c), AABB(a))|
        (AABB(a), Circle(c))=>{
            todo!();
        }
        (Circle(c),Rect(r))|
        (Rect(r), Circle(c))=>{
            todo!();
        }
        (AABB(a), AABB(b))=>{
            return
                b.x - b.rx < a.x + a.rx &&
                a.x - b.x < b.x + a.rx &&
                b.x - b.rx < a.x + a.rx &&
                a.x - b.x < b.x + a.rx;
        }
        (AABB(a), Rect(r))|
        (Rect(r), AABB(a))=>{
            todo!();
        }
        (Rect(a), Rect(b))=>{
            todo!();
        }
    }
    panic!("missing implementation in function");
}

/// Gir normalen til kollisjons-overflaten mellom to former,
/// og hvor mye de er inni hverandre
pub fn collision_normal_and_overlap()->(Vector2d, f32){
    todo!();
}

#[cfg(test)]
mod tests{
    use crate::{AABB, intersection};
    
    #[test]
    fn intersection_aabb_aabb(){
        assert_eq!(
            intersection(
                AABB(AABB{x:0.0, y:0.0, rx: 10.0, ry: 10.0 }),
                AABB(AABB{x:25.0, y:0.0, rx: 10.0, ry: 10.0})
            ), false);
        assert_eq!(
            intersection(
                AABB(AABB{x:0.0, y:0.0, rx: 10.0, ry: 10.0 }),
                AABB(AABB{x:18.0, y:0.0, rx: 10.0, ry: 10.0})
            ), true);
    }
}
