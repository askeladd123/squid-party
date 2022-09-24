use macroquad::prelude::*;

pub fn draw_shape(shape: impl physics::Shape, color: Color){
    use physics::Shapes;

    match shape.get_shape(){
        Shapes::Point(p) => draw_circle(p.x, p.y, screen_width() * 0.005, color),
        Shapes::Circle(c) => draw_circle(c.center.x, c.center.y, c.r, color),
        Shapes::AABB(a) => draw_rectangle(
            a.center.x - a.rx,
            a.center.y - a.ry,
            a.rx * 2.0,
            a.ry * 2.0,
            color
        ),
        Shapes::Rect(r) => draw_poly_lines(
            r.center.x,
            r.center.y,
            2,
            r.rx,
            r.a,
            r.ry,
            color
        ),
    }
}
