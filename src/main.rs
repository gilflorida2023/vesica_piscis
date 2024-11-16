use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    
    // Set white background
    draw.background().color(WHITE);
    
    // Calculate dimensions for the circles
    let radius = 200.0;
    let distance = radius; // Distance between centers equals radius for vesica piscis
    
    // Calculate intersection points
    let height = (radius.powi(2) - (distance/2.0).powi(2)).sqrt();
    
    // Points for the vesica
    let center_left = pt2(-distance/2.0, 0.0);  // Point A
    let center_right = pt2(distance/2.0, 0.0);  // Point B
    let top_point = pt2(0.0, height);          // Point C
    let bottom_point = pt2(0.0, -height);      // Point D
    let center = pt2(0.0, 0.0);               // Point O
    
    // Draw first circle (blue outline)
    draw.ellipse()
        .xy(center_left)
        .radius(radius)
        .no_fill()
        .stroke(BLUE)
        .stroke_weight(2.0);
    
    // Draw second circle (red outline)
    draw.ellipse()
        .xy(center_right)
        .radius(radius)
        .no_fill()
        .stroke(RED)
        .stroke_weight(2.0);

    // Draw points
    let point_radius = 4.0;
    
    // Draw and label center points
    draw.ellipse()
        .xy(center_left)
        .radius(point_radius)
        .color(BLACK)
        .w_h(point_radius * 2.0, point_radius * 2.0);

    draw.text("A")
        .xy(center_left + vec2(-15.0, -15.0))
        .color(BLACK)
        .font_size(20);

    draw.ellipse()
        .xy(center_right)
        .radius(point_radius)
        .color(BLACK)
        .w_h(point_radius * 2.0, point_radius * 2.0);

    draw.text("B")
        .xy(center_right + vec2(15.0, -15.0))
        .color(BLACK)
        .font_size(20);

    // Draw and label intersection points
    draw.ellipse()
        .xy(top_point)
        .radius(point_radius)
        .color(BLACK)
        .w_h(point_radius * 2.0, point_radius * 2.0);

    draw.text("C")
        .xy(top_point + vec2(-15.0, 10.0))
        .color(BLACK)
        .font_size(20);

    draw.ellipse()
        .xy(bottom_point)
        .radius(point_radius)
        .color(BLACK)
        .w_h(point_radius * 2.0, point_radius * 2.0);

    draw.text("D")
        .xy(bottom_point + vec2(-15.0, -25.0))
        .color(BLACK)
        .font_size(20);

    // Draw and label center point
    draw.ellipse()
        .xy(center)
        .radius(point_radius)
        .color(BLACK)
        .w_h(point_radius * 2.0, point_radius * 2.0);

    draw.text("O")
        .xy(center + vec2(15.0, -5.0))
        .color(BLACK)
        .font_size(20);

    // Draw connecting lines (thin black)
    draw.line()
        .start(center_left)
        .end(center_right)
        .color(BLACK)
        .stroke_weight(1.0);

    draw.line()
        .start(top_point)
        .end(bottom_point)
        .color(BLACK)
        .stroke_weight(1.0);

    draw.to_frame(app, &frame).unwrap();
}
