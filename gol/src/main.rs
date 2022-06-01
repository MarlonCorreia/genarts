use nannou::color;
use nannou::prelude::*;
use std::{thread, time::Duration};
mod logic;

fn main() {
    nannou::app(model).update(update).run();
}

fn gen_color(r: u8, g: u8, b: u8) -> color::rgb::Srgb<u8> {
    return color::rgb::Srgb {
        red: r,
        green: g,
        blue: b,
        standard: ::core::marker::PhantomData,
    };
}

struct Model {
    grid: [[i32; 15]; 15],
    bg_color: color::rgb::Srgb<u8>,
    grid_color: color::rgb::Srgb<u8>,
    block_color: color::rgb::Srgb<u8>,
    angle: f32,
    _window_id: window::Id
}

fn model(_app: &App) -> Model {
    let _window = _app.new_window().size(500, 500).view(view).build().unwrap();

    Model {
        grid: logic::random_grid(),
        bg_color: gen_color(104, 56, 168),
        grid_color: gen_color(166, 105, 245),
        block_color: gen_color(187, 135, 255),
        angle: 0.0,
        _window_id: _window    
    
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.grid = logic::update_grid(_model.grid);
    _model.angle += 5.0;
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(_model.bg_color);
    let r = _app.window(_model._window_id).unwrap().rect();

    let margin = 1.0;
    let block_qtd = _model.grid.len() as f32 + 1.0;

    let mut col_count = 1.0;
    let mut row_count = 1.0;
    let mut more = true;

    let dr = Rect::from_w_h(r.w() / 1.2, r.h() / 1.2);
    draw.rect().w_h(dr.w(), dr.h()).color(_model.grid_color);

    let (r_w, r_h) = dr.w_h();
    let w_size = r_w / block_qtd;
    let h_size = r_h / block_qtd;

    for v in _model.grid {
        for v in v {
            if more == true {
                let color = {
                    let mut c = _model.bg_color;
                    if v == 1 {
                        c = _model.block_color;
                    }
                    c
                };

                draw.rect()
                    .x(dr.x.start + (w_size * col_count))
                    .y(dr.y.end - (h_size * row_count))
                    .width(w_size - margin)
                    .height(h_size - margin)
                    .color(color);

                col_count += 1.0;
                if (w_size * col_count) + margin >= r_w {
                    row_count += 1.0;
                    col_count = 1.0;
                }
                if (h_size * row_count) + margin >= r_h {
                    more = false;
                }
            } else {
                break;
            }
        }
    }

    thread::sleep(Duration::from_millis(200));
    draw.to_frame(_app, &frame).unwrap();
}