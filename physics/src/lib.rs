extern crate core;
use Shape::*;

pub struct Vector2d{
    pub x:f32,
    pub y:f32
}
pub struct Circle{
    pub center:Vector2d,
    pub r:f32
}
pub struct AABB{
    pub center:Vector2d,
    pub rx:f32,
    pub ry:f32
}
pub struct Rect{
    pub center:Vector2d,
    pub rx:f32,
    pub ry:f32,
    pub a:f32
}

pub enum Shape{
    Point(Vector2d),
    Circle(Circle),
    AABB(AABB),
    Rect(Rect),
}

/// sier om to former er inni hverandre
pub fn intersection(a:Shape, b:Shape)->bool{
    match (a, b) {
        (Point(a), Point(b))=>{
            todo!();
        }
        (Point(p), Circle(c))|
        (Circle(c), Point(p))=>{
            todo!();
        }
        (Point(p), AABB(a))|
        (AABB(a), Point(p))=>{
            // todo!();
        }
        (Point(p), Rect(r))|
        (Rect(r), Point(p))=>{
            todo!();
        }
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
                b.center.x - b.rx < a.center.x + a.rx &&
                a.center.x - b.rx < b.center.x + a.rx &&
                b.center.y - b.ry < a.center.y + a.ry &&
                a.center.x - b.ry < b.center.x + a.ry
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
pub fn collision_normal_and_overlap(a:Shape, b:Shape)->(Vector2d, f32){
    todo!();
}

#[cfg(test)]
mod tests{
    
    use crate::*;
    
    #[test]
    fn intersection_point_point(){
        assert_eq!(
            intersection(
                Point(Vector2d{x:69.0, y:21.0}),
                Point(Vector2d{x:20.0, y:50.0})
            ), false);
        assert_eq!(
            intersection(
                Point(Vector2d{x:69.0, y:21.0}),
                Point(Vector2d{x:69.0, y:21.0}),
            ),true);
    }
    
    #[test]
    fn intersection_point_aabb(){
        assert_eq!(
            intersection(
                Point(Vector2d{x: 40.0, y:50.0}),
                AABB(AABB{center:Vector2d{x:-20.0, y:-50.0}, rx:30.0, ry:30.0})
            ), false);
        assert_eq!(
            intersection(
                Point(Vector2d{x: 40.0, y:50.0}),
                AABB(AABB{center:Vector2d{x:20.0, y:50.0}, rx:40.0, ry:40.0})
            ), true);
        assert_eq!(
            intersection(
                Point(Vector2d{x: 10.0, y:20.0}),
                AABB(AABB{center:Vector2d{x:100.0, y:50.0}, rx:40.0, ry:40.0})
            ), false);
        assert_eq!(
            intersection(
                Point(Vector2d{x: 70.0, y:80.0}),
                AABB(AABB{center:Vector2d{x:100.0, y:50.0}, rx:40.0, ry:40.0})
            ), true);
    }
    
    #[test]
    fn intersection_aabb_aabb(){
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:0.0, y:0.0}, rx: 10.0, ry: 10.0 }),
                AABB(AABB{center:Vector2d{x:25.0, y:0.0}, rx: 10.0, ry: 10.0})
            ), false);
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:0.0, y:0.0}, rx: 10.0, ry: 10.0 }),
                AABB(AABB{center:Vector2d{x:18.0, y:0.0}, rx: 10.0, ry: 10.0})
            ), true);
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:100.0, y:100.0}, rx: 10.0, ry: 10.0 }),
                AABB(AABB{center:Vector2d{x:40.0, y:40.0}, rx: 10.0, ry: 10.0})
            ), false);
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:100.0, y:100.0}, rx: 20.0, ry: 20.0 }),
                AABB(AABB{center:Vector2d{x:110.0, y:90.0}, rx: 20.0, ry: 20.0})
            ), true);
    }
    
    #[test]
    fn intersection_circle_circle(){
        assert_eq!(
            intersection(
                Circle(Circle{center:Vector2d{x:40.0, y:30.0}, r:10.0}),
                Circle(Circle{center:Vector2d{x:70.0, y:100.0}, r:12.0})
            ), true);
        assert_eq!(
            intersection(
                Circle(Circle{center:Vector2d{x:40.0, y:30.0}, r:10.0}),
                Circle(Circle{center:Vector2d{x:50.0, y:20.0}, r:40.0})
            ), true);
        assert_eq!(
            intersection(
                Circle(Circle{center:Vector2d{x:40.0, y:100.0}, r:10.0}),
                Circle(Circle{center:Vector2d{x:50.0, y:20.0}, r:20.0})
            ), false);
        assert_eq!(
            intersection(
                Circle(Circle{center:Vector2d{x:40.0, y:100.0}, r:100.0}),
                Circle(Circle{center:Vector2d{x:70.0, y:60.0}, r:20.0})
            ), true);
    }
}
