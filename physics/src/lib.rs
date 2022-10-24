//! **Physics XXL MEGA 3000**
//! ---
//!Funksjonalitet for kollisjon, litt matte og etterhvert fysikk.
//!
//! Biblioteket inneholder funksjoner som opererer på forskjellige structs,
//! blant annet: *Point, Circle, AABB* (Axis Aligned Bounding Box), *Rect*,
//! men også alt annet som implementerer et *Trait* som heter *Shape*

pub trait Shape{
    fn get_shape(& self)->Shapes;
    
    fn intersects(& self, shape:impl Shape)->bool{
        intersection(self.get_shape(), shape)
    }
}

#[derive(Copy, Clone)]
pub enum Shapes {
    Point(Vec2d),
    Circle(Circle),
    AABB(AABB),
    Rect(Rect),
}

impl Shape for Shapes{
    fn get_shape(&self) -> Shapes {
        *self
    }
}

/**
Todimensjonal vektor: for å gjøre matteoperasjoner enklere.

Tips: se funksjonene:
* `len`,
* `from`: konvertere fra tuple,
* `add`, `mul` osv.: operator overloading: du kan bruke vektor med "+", "*" osv.
 */
#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x:f32,
    pub y:f32
}

impl From<(f32, f32)> for Vec2d{
    fn from((x, y): (f32, f32)) -> Self {
        Vec2d { x, y }
    }
}

impl Shape for Vec2d {
    fn get_shape(&self) -> Shapes {
        Shapes::Point(*self)
    }
}

impl Vec2d {
    fn len()->f32{
        todo!() // euclidian distance
    }
}

impl core::ops::Add for Vec2d{
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        todo!() // eks: new_vec2d = this_vec2d + other_vec2d;
    }
}

impl core::ops::Sub for Vec2d{
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        todo!() // eks: new_vec2d = this_vec2d - other_vec2d;
    }
}

impl core::ops::AddAssign for Vec2d{
    fn add_assign(&mut self, rhs: Self) {
        todo!() // eks: this_vec2d += other_vec2d;
    }
}

impl core::ops::SubAssign for Vec2d{
    fn sub_assign(&mut self, rhs: Self) {
        todo!() // eks: this_vec2d -= other_vec2d
    }
}

impl core::ops::Mul<f32> for Vec2d{
    type Output = Self;
    
    fn mul(self, rhs: f32) -> Self::Output {
        todo!() // eks: let new_vec2d = old_vec2d * 15.0;
    }
}

impl core::ops::Div<f32> for Vec2d{
    type Output = Self;
    
    fn div(self, rhs: f32) -> Self::Output {
        todo!() // eks: let new_vec2d = old_vec2d / 15.0;
    }
}

impl core::ops::MulAssign<f32> for Vec2d{
    fn mul_assign(&mut self, rhs: f32) {
        todo!() // eks: let new_vec2d *= 15.0;
    }
}

impl core::ops::DivAssign<f32> for Vec2d{
    fn div_assign(&mut self, rhs: f32) {
        todo!() // eks: let new_vec2d /= 15.0;
    }
}

// class for alle sirkler, tar inn en vector2d + radiusen
#[derive(Copy, Clone)]
pub struct Circle{
    pub center: Vec2d,
    pub r:f32
}

impl Shape for Circle{
    fn get_shape(&self) -> Shapes {
        Shapes::Circle(*self)
    }
}

#[derive(Copy, Clone)]
pub struct AABB{
    pub center: Vec2d,
    pub rx:f32,
    pub ry:f32
}

impl Shape for AABB{
    fn get_shape(&self) -> Shapes {
        Shapes::AABB(*self)
    }
}

//class for alle rektangler, tar inn en vector2d + hvor mye ut til siden / opp + vinkel
#[derive(Copy, Clone)]
pub struct Rect{
    pub center: Vec2d,
    pub rx:f32,
    pub ry:f32,
    /** vinkel på boksen */
    pub a:f32
}

impl Shape for Rect{
    fn get_shape(&self) -> Shapes {
        Shapes::Rect(*self)
    }
}

/// Roterer en vektor til en gitt vinkel a
pub fn rotation_fun(r:Rect, corner:(f32,f32)) -> (f32, f32) {
    // The inputs for rotating a vector in a 2d plane
    let angle = (r.a * (std::f32::consts::PI)/180.0);
    let rot_2d = (f32::cos(angle), - f32::sin(angle), f32::sin(angle), f32::cos(angle));

    let corner =  ((f32::cos(angle) * corner.0  - f32::sin(angle) * corner.1),
                            ( f32::sin(angle) * corner.0 +  f32::cos(angle) * corner.1));
    return corner;
}

/// retunerer en linjer funksjon m, b
pub fn lin_funk(p1: (f32,f32), p2: (f32,f32)) -> (f32, f32) {
    let m = (p1.1 - p2.1) / (p1.0 - p2.0);
    let b = p1.1 - (m * p1.0);
    //println!("m and b {} {}",m, b);
    return (m,b);
}


/// sier om to former er inni hverandre
pub fn intersection(a: impl Shape, b: impl Shape) ->bool{
    
    use Shapes::*;
    
    match (a.get_shape(), b.get_shape()) {
        (Point(a), Point(b))=>{
            todo!(); // betyr dette gjøres senere
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
        (Point(p), Rect(mut r))|
        (Rect(mut r), Point(p))=>{
            r.ry = r.ry/2.0;
            let mut corners_tl = ((- r.rx), (-r.ry));
            let mut corners_bl = ((- r.rx), ( r.ry));
            let mut corners_tr = ((r.rx), (-r.ry));
            let mut corners_br = ((r.rx), ( r.ry));

            //println!("c1 {} {}",corners_tl.0, corners_tl.1);

            corners_tl = rotation_fun(r, corners_tl);
            corners_bl = rotation_fun(r, corners_bl);
            corners_tr = rotation_fun(r, corners_tr);
            corners_br = rotation_fun(r, corners_br);



            corners_tl = (r.center.x + corners_tl.0 , r.center.y + corners_tl.1);
            corners_bl = (r.center.x + corners_bl.0 , r.center.y + corners_bl.1);
            corners_tr = (r.center.x + corners_tr.0 , r.center.y + corners_tr.1);
            corners_br = (r.center.x + corners_br.0 , r.center.y + corners_br.1);


            let line1 = lin_funk(corners_tl, corners_tr);
            let line2 = lin_funk(corners_bl, corners_br);
            let line3 = lin_funk(corners_tl, corners_bl);
            let line4 = lin_funk(corners_tr, corners_br);


            return if
            line1.0 * p.x + line1.1 > p.y &&
                line2.0 * p.x + line2.1 < p.y
                ||
                line1.0 * p.x + line1.1 < p.y &&
                    line2.0 * p.x + line2.1 > p.y {
                if line3.0 * p.x + line3.1 < p.y &&
                    line4.0 * p.x + line4.1 > p.y
                    ||
                    line4.0 * p.x + line4.1 < p.y &&
                        line3.0 * p.x + line3.1 > p.y {
                    true
                } else { false }
            } else { false }
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
pub fn collision_normal_and_overlap<A: Shape, B:Shape>(a: A, b: B) ->(Vec2d, f32){
    todo!();
}
