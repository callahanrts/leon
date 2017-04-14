// UI Backend.
// The Paint module is responsible for painting pixels on the screen.

use conrod;

use render::css::{Color};
use render::layout::{Rect};
use conrod::widget::{Button, Canvas, Circle, Line, Oval, PointPath, Polygon, Rectangle};

pub fn draw_rect(ui: &mut conrod::UiCell, bounds: Rect, color: Color) {
    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use conrod::{color, widget, Colorable, Positionable, Scalar, Sizeable, Widget};
    // Generate a unique const `WidgetId` for each widget.
    widget_ids!{
        struct Ids {
            master,
            left_col,
            middle_col,
            right_col,
            left_text,
            middle_text,
            right_text,
        }
    }

    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    const DEMO_TEXT: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
        finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
        fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
        Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
        Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
        magna est, efficitur suscipit dolor eu, consectetur consectetur urna.";

    // A unique identifier for each widget.
    conrod::widget::Text::new(DEMO_TEXT)
        .color(color::LIGHT_RED)
        .left_justify()
        .line_spacing(10.0)
        .set(ids.left_text, ui);
}
