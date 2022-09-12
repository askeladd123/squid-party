extern crate core;
use Shape::*;

// selvforklarende, tar inn x og y posisjonen til noe på skjermen
#[derive(Copy, Clone)]
pub struct Vector2d{
    pub x:f32,
    pub y:f32
}
// class for alle sirkler, tar inn en vector2d + radiusen
#[derive(Copy, Clone)]
pub struct Circle{
    pub center:Vector2d,
    pub r:f32
}
#[derive(Copy, Clone)]
pub struct AABB{
    pub center:Vector2d,
    pub rx:f32,
    pub ry:f32
}
//class for alle rektangler, tar inn en vector2d + hvor mye ut til siden / opp + vinkel
#[derive(Copy, Clone)]
pub struct Rect{
    pub center:Vector2d,
    pub rx:f32,
    pub ry:f32,
    /** vinkel på boksen */
    pub a:f32
}
#[derive(Copy, Clone)]
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
            todo!(); // betyr dette gjøres senere?
        }
        (Point(p), Circle(c))|
        (Circle(c), Point(p))=>{
            return
            f32::sqrt((c.center.x - p.x) * (c.center.x - p.x)
                + (c.center.y - p.y) * (c.center.y - p.y)) < c.r

        }
        (Point(p), AABB(a))|
        (AABB(a), Point(p))=>{
            return
            p.x > a.center.x - a.rx &&
                p.x < a.center.x + a.rx &&
                p.y < a.center.y + a.ry &&
                p.y > a.center.y - a.ry
        }
        (Point(p), Rect(r))|
        (Rect(r), Point(p))=>{
            todo!();
        }
        (Circle(a), Circle(b))=>{
            return
                a.r + b.r > f32::sqrt((b.center.x - a.center.x) *  (b.center.x - a.center.x)+
                (b.center.y - a.center.y) * (b.center.y - a.center.y))

        }
        (Circle(c), AABB(a))|
        (AABB(a), Circle(c))=>{
            if c.center.x <= a.center.x + a.rx && c.center.x >= a.center.x - a.rx{
                return if
                c.center.y > a.center.y + a.ry {
                    //nede i midten
                    if a.ry + c.r > c.center.y - a.center.y {
                        true
                    } else { false }
                } else {
                    //oppe i midten
                    if a.ry + c.r > a.center.y - c.center.y {
                        true
                    } else { false }
                }
            }
            else if c.center.x <= a.center.x - a.rx{
                //oppe i venstre hjørnet
                return if c.center.y < a.center.y - a.ry {
                    if c.r >
                        f32::sqrt((((a.center.y - a.ry) - c.center.y) * ((a.center.y - a.ry) - c.center.y)) +
                            (((a.center.x - a.rx) - c.center.x) * ((a.center.x - a.rx) - c.center.x)))
                        {
                        true
                    } else { false }
                } else if c.center.y >= (a.center.y + a.ry) {
                    //nede til venstre
                    if c.r >
                        f32::sqrt(((c.center.y - (a.center.y + a.ry)) * (c.center.y - (a.center.y + a.ry)) ) +
                            (((a.center.x - a.rx) - c.center.x) * ((a.center.x - a.rx) - c.center.x)))
                    {
                        true
                    } else { false }
                } else {
                    //venstre midt
                    if a.rx + c.r >= a.center.x - c.center.x {
                        true
                    } else { false }
                }
            }
            else if c.center.x >= a.center.x + a.rx{
                return if c.center.y < a.center.y - a.ry {
                    //oppe i høyre hjørnet
                    if c.r >
                        f32::sqrt((((a.center.y - a.ry) - c.center.y) * ((a.center.y - a.ry) - c.center.y)) +
                            ((c.center.x - (a.center.x + a.rx)) * (c.center.x - (a.center.x + a.rx))))
                    {
                        true
                    } else { false }
                } else if c.center.y > (a.center.y + a.ry) {
                    //nede til høyre
                    if c.r >
                        f32::sqrt(((c.center.y - (a.center.y + a.ry)) * (c.center.y - (a.center.y + a.ry)))+
                            ((c.center.x - (a.center.x + a.rx)) * (c.center.x - (a.center.x + a.rx))))
                    {
                        true
                    } else { false }
                } else {
                    //høyre i midten
                    if a.rx + c.r >= c.center.x - a.center.x {
                        true
                    } else { false }
                }
            }

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
    fn intersection_aabb_circle(){
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:0.0, y:0.0}, rx: 10.0, ry: 10.0 }),
                Circle(Circle{center:Vector2d{x:70.0, y:100.0}, r:12.0})
            ), false);

        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:0.0, y:0.0}, rx: 10.0, ry: 10.0 }),
                Circle(Circle{center:Vector2d{x:10.0, y:12.0}, r:12.0})
            ), true);
        /*
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:100.0, y:100.0}, rx: 10.0, ry: 10.0 }),
                Circle(Circle{center:Vector2d{x:98.0, y:100.0}, r:12.0})
            ), true);
        assert_eq!(
            intersection(
                AABB(AABB{center:Vector2d{x:100.0, y:100.0}, rx: 20.0, ry: 20.0 }),
                Circle(Circle{center:Vector2d{x:98.0, y:100.0}, r:12.0})
            ), true);

         */
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
