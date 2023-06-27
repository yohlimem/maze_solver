use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

mod agent;
use crate::agent::Sides;


struct Model {
    // window: Window,
    egui: Egui,
    grid: agent::Grid,
    agent: agent::Agent,
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let agent = agent::Agent::new([Sides::Right,Sides::Downward,Sides::Upward,Sides::Left]);
    let mut grid = agent::Grid{
        size: 10,
        filled: vec![],
    };

    grid.generate_grid();
    Model {
        egui,
        agent,
        grid
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        if ui.add(egui::Button::new("rum")).clicked() {
            model.agent.step(&mut model.grid);
        }
    });

    // println!("{}", model.num);

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    
    model.grid.draw_grid(&draw);
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}