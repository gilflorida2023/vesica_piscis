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

    // Calculate tangent points
    // For a circle, tangent points are perpendicular to the radius at that point
    // The tangent points are at y = ±radius for vertical lines through centers
    let point_e = pt2(-distance/2.0, radius);  // Tangent point E
    let point_f = pt2(-distance/2.0, -radius); // Tangent point F
    let point_g = pt2(distance/2.0, radius);   // Tangent point G
    let point_h = pt2(distance/2.0, -radius);  // Tangent point H

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

    // Draw and label tangent points
    for (point, label, offset) in [
        (point_e, "E", vec2(-15.0, 10.0)),
        (point_f, "F", vec2(-15.0, -25.0)),
        (point_g, "G", vec2(15.0, 10.0)),
        (point_h, "H", vec2(15.0, -25.0)),
    ] {
        draw.ellipse()
            .xy(point)
            .radius(point_radius)
            .color(BLACK)
            .w_h(point_radius * 2.0, point_radius * 2.0);

        draw.text(label)
            .xy(point + offset)
            .color(BLACK)
            .font_size(20);
    }

    // Calculate Point R (where arc from G centered at O meets x-axis)
    // Since we're using center O, we need to calculate where the arc intersects x-axis
    let angle_g = (radius/distance/2.0).acos(); // angle of point G from vertical
    let r_distance = radius * angle_g.sin(); // x-coordinate of R
    let point_r = pt2(r_distance, 0.0);   // Point R

    // Draw yellow line from O to R
    draw.line()
        .start(center)
        .end(point_r)
        .color(YELLOW)
        .stroke_weight(2.0);

    // Draw point R
    draw.ellipse()
        .xy(point_r)
        .radius(point_radius)
        .color(BLACK)
        .w_h(point_radius * 2.0, point_radius * 2.0);

    draw.text("R")
        .xy(point_r + vec2(15.0, -15.0))
        .color(BLACK)
        .font_size(20);

    // Draw connecting lines (thin black)
    // Original lines
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

    // Vertical tangent lines through A and B
    draw.line()
        .start(point_e)
        .end(point_f)
        .color(BLACK)
        .stroke_weight(1.0);

    draw.line()
        .start(point_g)
        .end(point_h)
        .color(BLACK)
        .stroke_weight(1.0);

    // Horizontal tangent lines
    draw.line()
        .start(point_e)
        .end(point_g)
        .color(BLACK)
        .stroke_weight(1.0);

    draw.line()
        .start(point_f)
        .end(point_h)
        .color(BLACK)
        .stroke_weight(1.0);

    // Lines showing square root relationships
    // GA for √2
    draw.line()
        .start(point_g)
        .end(center_left)  // to point A
        .color(BLACK)
        .stroke_weight(1.0);

    // GF for √5
    draw.line()
        .start(point_g)
        .end(point_f)
        .color(BLACK)
        .stroke_weight(1.0);

    // Add square root labels, each positioned to appear only on their specific line
    
    // √2 label - halfway between A and GA-CD intersection
    let ga_direction = (center_left - point_g).normalize();  // Direction from G to A
    let label_position = point_g + ga_direction * 250.0;  // Position further down towards A
    draw.text("√2")
        .xy(label_position)  // Position along GA line
        .color(BLACK)
        .font_size(20);

    // √3 label - on CD line, below origin to avoid 'O' label
    let cd_midpoint = (top_point + bottom_point) / 2.0;
    draw.text("√3")
        .xy(cd_midpoint + vec2(0.0, 30.0))  // Below origin
        .color(BLACK)
        .font_size(20);

    // √5 label - on GF line, between O and F
    let gf_direction = (point_f - point_g).normalize();  // Direction from G to F
    let label_position = center + gf_direction * 100.0;  // Position from O towards F
    draw.text("√5")
        .xy(label_position)  // Position along GF line between O and F
        .color(BLACK)
        .font_size(20);

    draw.to_frame(app, &frame).unwrap();
}
