//! Show a custom window frame instead of the default OS window chrome decorations.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::string::ToString;
// hide console window on Windows in release
use eframe::egui;
use crate::egui::{Rect, Vec2, vec2};

fn main() {
    let win_h = 364.0;
    let win_w = 235.0;
    
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: false,
        // To have rounded corners we need transparency:
        transparent: true,
        max_window_size: Some(egui::vec2(win_w, win_h)),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    first_num: i32,
    second_num: i32,
    op: String,
    s: String,
}

impl MyApp {
    fn calculate_answer(a: i32, b: i32, op: String) -> i32 {
        if op == "+" {
            return add_two_numbers(a, b);
        }
        else if op == "-" {
            return sub_two_numbers(a, b);
        }
        else if op == "/" {
            return div_two_numbers(a, b);
        }
        else if op == "*" {
            return mul_two_numbers(a, b);
        }
        else { return 0; }
    }
}


impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        custom_window_frame(ctx, frame, "Rusty Calculator", |ui| {
            // ui.label("Choose Mode");
            ui.horizontal(|ui| {
                // ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
            //let mut s: String = String::from("");
            ui.horizontal(|ui| {
                ui.add_sized(vec2(225.0, 35.0), egui::widgets::TextEdit::singleline(&mut self.s));
            });
            for mut i in (1..=9).step_by(3) {
                //ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new(i.to_string())).clicked() {
                        print!("{}", i);
                        self.s.push_str(&(i.to_string()));
                    }
                    if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new((i+1).to_string())).clicked() {
                        print!("{}", i+1);
                        self.s.push_str(&((i+1).to_string()));
                    }
                    if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new((i+2).to_string())).clicked() {
                        print!("{}", i+2);
                        self.s.push_str(&((i+2).to_string()));
                    }
                    if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("")).clicked() {
                        print!("");
                        self.s.push_str("");
                    }
                });
            }
            ui.horizontal(|ui| {
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("+")).clicked() {
                    print!("+");
                    self.first_num = self.s.parse::<i32>().expect("cannot convert");
                    self.op.clear();
                    self.op.push('+');
                    self.s.clear();
                }
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("0")).clicked() {
                    print!("{}", 0);
                    self.s.push_str(&(0.to_string()));
                }
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("/")).clicked() {
                    print!("/");
                    self.first_num = self.s.parse::<i32>().expect("cannot convert");
                    self.op.clear();
                    self.op.push('/');
                    self.s.clear();
                }
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("=")).clicked() {
                    print!("=");
                    self.second_num = self.s.parse::<i32>().expect("cannot convert");
                    // self.s.push_str("=");
                    let equals = MyApp::calculate_answer(self.first_num, self.second_num, self.op.clone());
                    print!("{}", equals);
                    self.s.clear();
                    self.s.push_str(&(equals.to_string()));
                }
            });

            ui.horizontal(|ui| {
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("-")).clicked() {
                    print!("-");
                    self.first_num = self.s.parse::<i32>().expect("cannot convert");
                    self.op.clear();
                    self.op.push('-');
                    self.s.clear();
                }
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("")).clicked() {
                    print!("");
                }
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("*")).clicked() {
                    print!("*");
                    self.first_num = self.s.parse::<i32>().expect("cannot convert");
                    self.op.clear();
                    self.op.push('*');
                    self.s.clear();
                }
                if ui.add_sized(vec2(50.0, 50.0), egui::widgets::Button::new("")).clicked() {
                    print!("");
                }
            });
        });
    }
}

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    // Height of the title bar
    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height * 0.8),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            // Add the close button:
            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.close();
            }

            // Interact with the title bar (drag to move window):
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
                .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}

// add two numbers
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// subtracts two numbers
fn sub_two_numbers(a: i32, b: i32) -> i32 {
    a - b
}

// multiplies two numbers
fn mul_two_numbers(a: i32, b: i32) -> i32 {
    a * b
}

// divides two numbers
fn div_two_numbers(a: i32, b: i32) -> i32 {
    a / b
}