#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use glfw::{Action, Context, Key};
use gl;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("GLFW init failed");
    let (mut window, events) = glfw.create_window(800, 600, "Default window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::ClearColor(0.2, 0.2, 0.3, 1.0);
    }

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        check_gl_error();
        
        window_event(&mut window, &mut glfw, &events);
    }
}

fn window_event(window: &mut glfw::Window, glfw: &mut glfw::Glfw, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    window.swap_buffers();
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        handle_window_event(window, event);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

fn check_gl_error() {
    let error: u32 = unsafe {gl::GetError()};
    match error {
        gl::NO_ERROR => {},
        _ => println!("OpenGLError : {error}")
    }
}
