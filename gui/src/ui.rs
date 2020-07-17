use songbird_core::cpu::Cpu;
use songbird_core::io::Buttons;
use songbird_core::utils::{SCREEN_HEIGHT, SCREEN_WIDTH};

use imgui::{Condition, Context, Image, TextureId, Window};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{ElementState, Event, KeyboardInput, WindowEvent, VirtualKeyCode};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;

use glium::BlitTarget;
use glium::Display;
use glium::Surface;
use glium::texture::{MipmapsOption, RawImage2d, Texture2d, UncompressedFloatFormat};
use glium::uniforms::MagnifySamplerFilter;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::rc::Rc;

// Constants
const SCALE: usize = 5;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH * SCALE) as u32;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT * SCALE) as u32;
const IMGUI_MARGIN: u32 = 16;

// Imgui interface taken from here: https://gist.github.com/RainbowCookie32/7e5d76acf33d88f2145d5ebc047a5799
pub struct ImguiSystem {
    pub event_loop: EventLoop<()>,
    pub display: Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
}

impl ImguiSystem {
    pub fn new(title: &str) -> ImguiSystem {
        let event_loop = EventLoop::new();
        let cb = ContextBuilder::new().with_vsync(true);
        let wb = WindowBuilder::new().with_inner_size(LogicalSize {
                        width: WINDOW_WIDTH + IMGUI_MARGIN,
                        height: WINDOW_HEIGHT + IMGUI_MARGIN,
                    }).with_title(title);
        let display = Display::new(wb, cb, &event_loop).unwrap();
        let mut imgui = Context::create();
        let mut platform = WinitPlatform::init(&mut imgui);
        // Limit scope to appease borrow checker
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
        }
        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        ImguiSystem {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
        }
    }

    /// ```
    /// Main event loop
    ///
    /// Passes control of emulation to the event loop, which runs forever
    ///
    /// Inputs:
    ///     Game boy CPU (Cpu)
    ///     Filename of ROM (String)
    /// ```
    pub fn main_loop(self, mut gb: Cpu, filename: String) {
        // After 'move', code cannot access self, but it can access individual variables
        // Someday I might understand this well enough to make it cleaner
        let ImguiSystem {
            event_loop,
            display,
            mut imgui,
            platform,
            mut renderer
        } = self;

        let mut texture_id = None;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    // Exit program if specified
                    *control_flow = ControlFlow::Exit;
                },
                Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(keycode), state, ..}, ..}, ..} => {
                        // Send keyboard inputs to emulator core
                        if let Some(btn) = key2btn(keycode) {
                            gb.toggle_button(btn, state == ElementState::Pressed);
                        }
                },
                Event::MainEventsCleared => {
                    let gl_window = display.gl_window();
                    platform.prepare_frame(imgui.io_mut(), &gl_window.window()).unwrap();
                    gl_window.window().request_redraw();
                },
                Event::RedrawRequested(_) => {
                    tick_until_draw(&mut gb, &filename);
                    let disp_arr = gb.render();
                    // I'd someday like to figure out how to not pass so many items in
                    draw_screen(&disp_arr, &display, &mut imgui, &platform, &mut renderer, &mut texture_id);
                },
                _ => {}
            }
        });
    }
}

/// ```
/// Draw screen
///
/// Takes RGBA pixels from a frame and renders them onto a window
///
/// Inputs:
///     Array of RGBA pixel data (&[u8])
///     Glium display (&Display)
///     Imgui context (&Context)
///     Winit platform (&WinitPlatform)
///     Rendering context (&Renderer)
///     Texture ID (&Option<TextureId>)
/// ```
fn draw_screen(
    disp_arr: &[u8],
    display: &Display,
    imgui: &mut Context,
    platform: &WinitPlatform,
    renderer: &mut Renderer,
    texture_id: &mut Option<TextureId>
) {
    let ui = imgui.frame();

    let dest_texture = Texture2d::empty_with_format(
        display,
        UncompressedFloatFormat::U8U8U8U8,
        MipmapsOption::NoMipmap,
        WINDOW_WIDTH,
        WINDOW_HEIGHT
    ).unwrap();

    // Copy our RGBA pixel data into openGL texture
    let image = RawImage2d::from_raw_rgba(disp_arr.to_vec(), (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
    let source_texture = Texture2d::new(display, image).unwrap();

    let dest_rect = BlitTarget {
        left: 0,
        bottom: 0,
        width: WINDOW_WIDTH as i32,
        height: WINDOW_HEIGHT as i32,
    };

    // Blit pixel data onto destination surface
    source_texture.as_surface().blit_whole_color_to(
        &dest_texture.as_surface(),
        &dest_rect,
        MagnifySamplerFilter::Nearest
    );

    // Keep track of texture ID, needed for imgui UI
    if texture_id.is_some() {
        renderer.textures().replace(texture_id.unwrap(), Rc::new(dest_texture));
    } else {
        *texture_id = Some(renderer.textures().insert(Rc::new(dest_texture)));
    }

    // This is the actual window that displays the emulation
    Window::new(im_str!("Songbird"))
        // The right/bottom parts of the window get cutoff, and require a somewhat arbitrary buffer so they show up correctly on screen, for some reason
        .size([(WINDOW_WIDTH + 16) as f32, (WINDOW_HEIGHT + 16) as f32], Condition::Once)
        .position([0.0, 0.0], Condition::Once)
        .title_bar(false)
        .resizable(false)
        .movable(false)
        .scroll_bar(false)
        .draw_background(false)
        .build(&ui, || {
        Image::new(texture_id.unwrap(), [WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32]).build(&ui);
    });

    let gl_window = display.gl_window();
    let mut target = display.draw();
    platform.prepare_render(&ui, gl_window.window());

    let draw_data = ui.render();
    renderer.render(&mut target, draw_data).unwrap();
    target.finish().unwrap();
}

/// ```
/// Tick until draw
///
/// Repeatedly runs until it is time to render a frame
///
/// Inputs:
///     Game Boy CPU (&Cpu)
///     Filename of game ROM (&String)
/// ```
fn tick_until_draw(gb: &mut Cpu, filename: &String) {
    let mut draw_time = false;
    while !draw_time {
        draw_time = gb.tick();

        if gb.is_battery_dirty() {
            write_battery_save(gb, &filename);
        }
    }
}

/// ```
/// Key to Button
///
/// Converts keycode into GameBoy button
///
/// Input:
///     Glium keybode keycode (VirtualKeyCode)
///
/// Output:
///     Gameboy button (Option<Buttons>)
/// ```
fn key2btn(key: VirtualKeyCode) -> Option<Buttons> {
    match key {
        VirtualKeyCode::Down =>    { Some(Buttons::Down)   },
        VirtualKeyCode::Up =>      { Some(Buttons::Up)     },
        VirtualKeyCode::Right =>   { Some(Buttons::Right)  },
        VirtualKeyCode::Left =>    { Some(Buttons::Left)   },
        VirtualKeyCode::Return =>  { Some(Buttons::Start)  },
        VirtualKeyCode::Back =>    { Some(Buttons::Select) },
        VirtualKeyCode::X =>       { Some(Buttons::A)      },
        VirtualKeyCode::Z =>       { Some(Buttons::B)      },
        _ =>                       { None                  }
    }
}

/// ```
/// Write Battery save
///
/// Updates save file to latest contents of battery RAM
///
/// Inputs:
///     Game Boy CPU object (Cpu)
///     Name of ROM file (&str)
/// ```
fn write_battery_save(gb: &mut Cpu, gamename: &str) {
    if gb.has_battery() {
        let ram_data = gb.get_ext_ram();
        let mut filename = gamename.to_owned();
        filename.push_str(".sav");

        let mut file = OpenOptions::new().write(true).create(true).open(filename).expect("Error opening save file");
        file.write(ram_data).unwrap();
        gb.clean_battery_flag();
    }
}
