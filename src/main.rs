extern crate gl;
extern crate sdl2;

pub mod gl_program;
pub mod gl_shader;
pub mod gl_utils;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("SDF Renderer", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::ffi::CString;

    let vert_shader = gl_shader::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = gl_shader::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = gl_program::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    let create_field_vertex_shader = gl_shader::Shader::from_vert_source(
        &CString::new(include_str!("create_field.vert")).unwrap()
    ).unwrap();

    let create_field_fragment_shader = gl_shader::Shader::from_frag_source(
        &CString::new(include_str!("create_field.frag")).unwrap()
    ).unwrap();

    let field_shader_program = gl_program::Program::from_shaders(
        &[create_field_vertex_shader, create_field_fragment_shader]
    ).unwrap();

    let modify_field_vertex_shader = gl_shader::Shader::from_vert_source(
        &CString::new(include_str!("modify_field.vert")).unwrap()
    ).unwrap();

    let modify_field_fragment_shader = gl_shader::Shader::from_frag_source(
        &CString::new(include_str!("modify_field.frag")).unwrap()
    ).unwrap();

    let modify_shader_program = gl_program::Program::from_shaders(
        &[modify_field_vertex_shader, modify_field_fragment_shader]
    ).unwrap();

    let mut fbo: gl::types::GLuint = 0;
    unsafe{
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
    }

    let mut volume_tex: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut volume_tex);
        gl::BindTexture(gl::TEXTURE_3D, volume_tex);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);

        gl::TexImage3D(gl::TEXTURE_3D, 0, gl::R16F as i32, 900 as i32, 700 as i32, 1920.0 as i32, 0, gl::RED, gl::UNSIGNED_BYTE, std::ptr::null_mut());
        gl::FramebufferTexture3D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_3D, volume_tex, 0, 0);
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("BORKEN");
        }

        gl::BindTexture(gl::TEXTURE_3D, 0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    let mut fbo2: gl::types::GLuint = 0;
    unsafe{
        gl::GenFramebuffers(1, &mut fbo2);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo2);
    }

    let mut volume_tex2: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut volume_tex2);
        gl::BindTexture(gl::TEXTURE_3D, volume_tex2);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);

        gl::TexImage3D(gl::TEXTURE_3D, 0, gl::R16F as i32, 900 as i32, 700 as i32, 1920.0 as i32, 0, gl::RED, gl::UNSIGNED_BYTE, std::ptr::null_mut());
        gl::FramebufferTexture3D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_3D, volume_tex2, 0, 0);
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("BORKEN");
        }

        gl::BindTexture(gl::TEXTURE_3D, 0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    let vertices: Vec<f32> = vec![
        -1.0, -1.0, 0.0,
        3.0, -1.0, 0.0,
        -1.0, 3.0, 0.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
        0, // index of the generic vertex attribute ("layout (location = 0)")
        3, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        std::ptr::null() // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let uniform_layer_location: gl::types::GLint;
    let uniform_layer_name = std::ffi::CString::new("u_layer").expect("CString::new failed");
    unsafe{
        uniform_layer_location = gl::GetUniformLocation(field_shader_program.id(), uniform_layer_name.as_ptr());
    }

    field_shader_program.set_used();

    unsafe{
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
    }
    for i in 0..1920 {
        unsafe{
            gl::FramebufferTexture3D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_3D, volume_tex, 0, i);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::Uniform1f(uniform_layer_location, i as gl::types::GLfloat);
            gl::DrawArrays(
            gl::TRIANGLES, // mode
            0, // starting index in the enabled arrays
            3 // number of indices to be rendered
            );
        }
    }

    let mut sampler: gl::types::GLuint = 0;
    unsafe{
        gl::GenSamplers(1, &mut sampler);

        gl::SamplerParameteri(sampler, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::SamplerParameteri(sampler, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    }

    let time_location: gl::types::GLint;
    let cname = std::ffi::CString::new("time").expect("CString::new failed");

    let mouse_uniform_location: gl::types::GLint;
    let cname_mouse = std::ffi::CString::new("u_mouse_pos").expect("CString::new failed");

    unsafe{
        time_location = gl::GetUniformLocation(shader_program.id(), cname.as_ptr());
        mouse_uniform_location = gl::GetUniformLocation(shader_program.id(), cname_mouse.as_ptr());
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;
    let mut cam_z = 0.0;

    let mut cam_rot_x = 0.0;
    let mut cam_rot_y = 0.0;
    let mut cam_rot_z = 0.0;

    let mut is_zoomed = false;

    use std::time::Instant;
    let start = Instant::now();
    let mut first = true;
    let mut is_in = 3;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::MouseWheel { timestamp, window_id, which, x, y, direction } =>
                {
                    cam_z += y as f32 * -0.01;
                    let cam_uniform_location: gl::types::GLint;
                    let cname_cam = std::ffi::CString::new("u_cam_pos").expect("CString::new failed");
                    unsafe{
                        cam_uniform_location = gl::GetUniformLocation(shader_program.id(), cname_cam.as_ptr());
                        gl::Uniform3f(cam_uniform_location, cam_x, cam_y, cam_z);
                    }
                },
                _ => {},
            }
        } // End SDL event

        let mouse_state = event_pump.mouse_state();

        is_in += 1;
        if mouse_state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left)
        {
            is_in = 0;
            let uniform_layer_location: gl::types::GLint;
            let uniform_layer_name = std::ffi::CString::new("u_layer").expect("CString::new failed");
            unsafe{
                uniform_layer_location = gl::GetUniformLocation(modify_shader_program.id(), uniform_layer_name.as_ptr());
            }

            let mouse_uniform_location: gl::types::GLint;
            let cname_mouse = std::ffi::CString::new("u_mouse_pos").expect("CString::new failed");

            unsafe{
                mouse_uniform_location = gl::GetUniformLocation(modify_shader_program.id(), cname_mouse.as_ptr());
            }

            modify_shader_program.set_used();

            if first == true
            {
                unsafe{
                    gl::BindFramebuffer(gl::FRAMEBUFFER, fbo2);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                for i in 0..1920 {
                    unsafe{
                        gl::FramebufferTexture3D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_3D, volume_tex2, 0, i);
                        gl::BindVertexArray(vao);
                        gl::BindTexture(gl::TEXTURE_3D, volume_tex);
                        gl::Uniform1f(uniform_layer_location, i as gl::types::GLfloat);
                        gl::Uniform2f(mouse_uniform_location, mouse_state.x() as f32 / 900.0, (700.0 - mouse_state.y() as f32) / 700.0);
                        gl::DrawArrays(
                        gl::TRIANGLES, // mode
                        0, // starting index in the enabled arrays
                        3 // number of indices to be rendered
                        );
                    }
                }

                first = false;
            }
            else
            {
                unsafe{
                    gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                for i in 0..1920 {
                    unsafe{
                        gl::FramebufferTexture3D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_3D, volume_tex, 0, i);
                        gl::BindVertexArray(vao);
                        gl::BindTexture(gl::TEXTURE_3D, volume_tex2);
                        gl::Uniform1f(uniform_layer_location, i as gl::types::GLfloat);
                        gl::Uniform2f(mouse_uniform_location, mouse_state.x() as f32 / 900.0, (700.0 - mouse_state.y() as f32) / 700.0);
                        gl::DrawArrays(
                        gl::TRIANGLES, // mode
                        0, // starting index in the enabled arrays
                        3 // number of indices to be rendered
                        );
                    }
                }

                first = true;
            }
        }
        
        let state = event_pump.relative_mouse_state();
        if mouse_state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Right)
        {
            let cam_uniform_location: gl::types::GLint;
            let cname_cam = std::ffi::CString::new("u_cam_pos").expect("CString::new failed");

            unsafe{
                cam_uniform_location = gl::GetUniformLocation(shader_program.id(), cname_cam.as_ptr());
                cam_x += state.x() as f32 / 900.0;
                cam_y += state.y() as f32 / 700.0;
                gl::Uniform3f(cam_uniform_location, cam_x, cam_y, cam_z);
            }
        }

       if mouse_state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Middle)
        {
            let cam_uniform_rotation: gl::types::GLint;
            let cname_cam_rot = std::ffi::CString::new("u_cam_rot").expect("CString::new failed");

            unsafe{
                cam_uniform_rotation = gl::GetUniformLocation(shader_program.id(), cname_cam_rot.as_ptr());
                cam_rot_x += state.x() as f32 / 900.0;
                cam_rot_y += state.y() as f32 / 700.0;
                gl::Uniform3f(cam_uniform_rotation, cam_rot_x, cam_rot_y, cam_rot_z);
            }
        }

        shader_program.set_used();
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            if first == true
            {
                gl::BindTexture(gl::TEXTURE_3D, volume_tex2);
            }
            else
            {
                gl::BindTexture(gl::TEXTURE_3D, volume_tex);
            }
            gl::BindSampler(0, sampler);
            gl::Uniform1f(time_location, start.elapsed().as_secs_f32() as gl::types::GLfloat);
            gl::Uniform2f(mouse_uniform_location, mouse_state.x() as f32 / 900.0, (700.0 - mouse_state.y() as f32) / 700.0);
            gl::BindVertexArray(vao);
            gl::DrawArrays(
            gl::TRIANGLES, // mode
            0, // starting index in the enabled arrays
            3 // number of indices to be rendered
            );
            gl::BindSampler(0, 0);
        }

        window.gl_swap_window();
    } // End main
}
