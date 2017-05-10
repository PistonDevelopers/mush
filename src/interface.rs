use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin;
use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, TouchPhase};
use imgui::{ImGui, Ui, ImGuiKey};
use imgui_glium_renderer::Renderer;
use std::time::Instant;

pub struct Interface {
    display: GlutinFacade,
    imgui: ImGui,
    renderer: Renderer,
    last_frame: Instant,
    mouse_pos: (i32, i32),
    mouse_pressed: (bool, bool, bool),
    mouse_wheel: f32,
}

impl Interface {
    pub fn init() -> Interface {
        let display = glutin::WindowBuilder::new()
            .with_title("mush")
            .with_dimensions(1280,720)
            .build_glium().expect("ERROR: Unable to create GL Window context");

        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display)
            .expect("ERROR: Unable to create GL Renderer context");

        // map key codes from glutin to imgui
        imgui.set_imgui_key(ImGuiKey::Tab, VirtualKeyCode::Tab as u8);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, VirtualKeyCode::Left as u8);
        imgui.set_imgui_key(ImGuiKey::RightArrow, VirtualKeyCode::Right as u8);
        imgui.set_imgui_key(ImGuiKey::UpArrow, VirtualKeyCode::Up as u8);
        imgui.set_imgui_key(ImGuiKey::DownArrow, VirtualKeyCode::Down as u8);
        imgui.set_imgui_key(ImGuiKey::PageUp, VirtualKeyCode::PageUp as u8);
        imgui.set_imgui_key(ImGuiKey::PageDown, VirtualKeyCode::PageDown as u8);
        imgui.set_imgui_key(ImGuiKey::Home, VirtualKeyCode::Home as u8);
        imgui.set_imgui_key(ImGuiKey::End, VirtualKeyCode::End as u8);
        imgui.set_imgui_key(ImGuiKey::Delete, VirtualKeyCode::Delete as u8);
        imgui.set_imgui_key(ImGuiKey::Backspace, VirtualKeyCode::Back as u8);
        imgui.set_imgui_key(ImGuiKey::Enter, VirtualKeyCode::Return as u8);
        imgui.set_imgui_key(ImGuiKey::Escape, VirtualKeyCode::Escape as u8);

        Interface {
            display: display,
            imgui: imgui,
            renderer: renderer,
            last_frame: Instant::now(),
            mouse_pos: (0, 0),
            mouse_pressed: (false, false, false),
            mouse_wheel: 0.0,
        }
    }

    fn update_mouse(&mut self) {
        let scale = self.imgui.display_framebuffer_scale();
        self.imgui
            .set_mouse_pos(self.mouse_pos.0 as f32 / scale.0,
                           self.mouse_pos.1 as f32 / scale.1);
        self.imgui
            .set_mouse_down(&[self.mouse_pressed.0,
                              self.mouse_pressed.1,
                              self.mouse_pressed.2,
                              false,
                              false]);
        self.imgui.set_mouse_wheel(self.mouse_wheel / scale.1);
        self.mouse_wheel = 0.0;
    }

    pub fn render<F: FnMut(&Ui)>(&mut self, clear_color: (f32, f32, f32, f32), mut run_ui: F) -> bool {
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        let mut target = self.display.draw();
        target.clear_color(clear_color.0, clear_color.1, clear_color.2, clear_color.3);

        if let Some(window) = self.display.get_window() {
            let size_points = window.get_inner_size_points().unwrap();
            let size_pixels = window.get_inner_size_pixels().unwrap();

            let ui = self.imgui.frame(size_points, size_pixels, delta_s);
            run_ui(&ui);

            let _ = self.renderer.render(&mut target, ui);
        }

        let _ = target.finish();

        self.update_mouse();
        self.update_events()
    }

    fn update_events(&mut self) -> bool {
        for event in self.display.poll_events() {
            match event {
                Event::Closed => return false,
                Event::KeyboardInput(state, _, code) => {
                    let pressed = state == ElementState::Pressed;
                    match code {
                        Some(VirtualKeyCode::LControl) |
                        Some(VirtualKeyCode::RControl) => self.imgui.set_key_ctrl(pressed),
                        Some(VirtualKeyCode::LShift) |
                        Some(VirtualKeyCode::RShift) => self.imgui.set_key_shift(pressed),
                        Some(VirtualKeyCode::LAlt) |
                        Some(VirtualKeyCode::RAlt) => self.imgui.set_key_alt(pressed),
                        Some(VirtualKeyCode::LWin) |
                        Some(VirtualKeyCode::RWin) => self.imgui.set_key_super(pressed),
                        Some(code) => self.imgui.set_key(code as u8, pressed),
                        _ => {}
                    }
                }
                Event::MouseMoved(x, y) => self.mouse_pos = (x, y),
                Event::MouseInput(state, MouseButton::Left) => {
                    self.mouse_pressed.0 = state == ElementState::Pressed
                }
                Event::MouseInput(state, MouseButton::Right) => {
                    self.mouse_pressed.1 = state == ElementState::Pressed
                }
                Event::MouseInput(state, MouseButton::Middle) => {
                    self.mouse_pressed.2 = state == ElementState::Pressed
                }
                Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) |
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => {
                    self.mouse_wheel = y
                }
                Event::ReceivedCharacter(c) => self.imgui.add_input_character(c),
                _ => (),
            }
        }
        true
    }
}
