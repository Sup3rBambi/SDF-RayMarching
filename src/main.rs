#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use glfw::{Action, Context, Key};
use gl;

mod vertex_array;

type Vertex = [f32; 2];

const RES_FOLDER: &str = "res";

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("GLFW init failed");
    let (mut window, events) = glfw.create_window(800, 600, "Default window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let mut vao = 0;
    let mut vbo = 0;
    const VERTICES: [Vertex; 4] = [[-1., -1.], [-1., 1.], [1., -1.], [1., 1.]];

    unsafe {
        gl::ClearColor(0.2, 0.2, 0.3, 1.0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&VERTICES) as isize, 
            VERTICES.as_ptr().cast(), gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size_of::<Vertex>().try_into().unwrap(), 0 as *const _);
        gl::EnableVertexAttribArray(0);
        let vertex = load_vertex_shader();
        let fragment = load_fragment_shader();
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);
        check_program_link_err(program);
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);
    }

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

fn load_vertex_shader() -> u32 {
    let vertex_shader;
    let vert_shader = fs::read_to_string(format!("{RES_FOLDER}/shaders/vertex.glsl")).expect("Cannot read vertex.glsl");
    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, 
            &(vert_shader.as_bytes().as_ptr().cast()), 
            &(vert_shader.len().try_into().unwrap()));
        gl::CompileShader(vertex_shader);
        check_shader_compil_err(vertex_shader);
    }
    return vertex_shader;
}

fn load_fragment_shader() -> u32 {
    let fragment_shader;
    let frag_shader = fs::read_to_string(format!("{RES_FOLDER}/shaders/fragment.glsl")).expect("Cannot read fragment.glsl");
    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, 
            &(frag_shader.as_bytes().as_ptr().cast()), 
            &(frag_shader.len().try_into().unwrap()));
        gl::CompileShader(fragment_shader);
        check_shader_compil_err(fragment_shader);
    }
    return fragment_shader;
}

fn check_shader_compil_err(shader: u32) {
    unsafe {
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(shader, 1024, &mut log_len, log.as_mut_ptr().cast());
            log.set_len(log_len.try_into().unwrap());
            panic!("Shader compile error: {}", String::from_utf8_lossy(&log));
        }
    }
}

fn check_program_link_err(prog: u32) {
    unsafe {
        let mut success = 0;
        gl::GetShaderiv(prog, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut log: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(prog, 1024, &mut log_len, log.as_mut_ptr().cast());
            log.set_len(log_len.try_into().unwrap());
            panic!("Program link error: {}", String::from_utf8_lossy(&log));
        }
    }
}
