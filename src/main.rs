use std::f32::consts::PI;

use macroquad::prelude::*;

#[macroquad::main("AnglePlayer")]
async fn main() {
    let mut p1 = (
        rand::gen_range(0.0, 2.0 * PI),
        rand::gen_range(50, 250) as f32,
    );
    let mut p2 = (
        rand::gen_range(0.0, 2.0 * PI),
        rand::gen_range(50, 250) as f32,
    );

    let mut ans = false;
    let t = (p1.0 - p2.0).abs() * 180.0 / PI;
    let mut angle = if t > 180.0 { 360.0 - t } else { t };

    let o_x = screen_width() / 2.0;
    let o_y = screen_height() / 2.0;

    let mut p1_x = p1.1 * p1.0.cos() + o_x;
    let mut p1_y = p1.1 * p1.0.sin() + o_y;
    let mut p2_x = p2.1 * p2.0.cos() + o_x;
    let mut p2_y = p2.1 * p2.0.sin() + o_y;

    loop {
        clear_background(LIGHTGRAY);
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 10.0, RED);
        draw_circle(o_x, o_y, 20.0, RED);
        draw_circle(p1_x, p1_y, 10.0, GOLD);
        draw_circle(p2_x, p2_y, 10.0, GOLD);

        if ans {
            draw_text(format!("{angle}").as_str(), 10., 60., 60., DARKGRAY);
        }

        if is_key_pressed(KeyCode::Enter) {
            if !ans {
                ans = true;
            } else {
                p1 = (
                    rand::gen_range(0.0, 2.0 * PI),
                    rand::gen_range(50, 250) as f32,
                );
                p2 = (
                    rand::gen_range(0.0, 2.0 * PI),
                    rand::gen_range(50, 250) as f32,
                );

                ans = false;
                let t = (p1.0 - p2.0).abs() * 180.0 / PI;
                angle = if t > 180.0 { 360.0 - t } else { t };

                p1_x = p1.1 * p1.0.cos() + o_x;
                p1_y = p1.1 * p1.0.sin() + o_y;
                p2_x = p2.1 * p2.0.cos() + o_x;
                p2_y = p2.1 * p2.0.sin() + o_y;
            }
        }

        next_frame().await;
    }
}
