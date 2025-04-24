use app::AppTrait;
use flock::{Flock, boid::Boid};
use glium::{
    glutin::surface::WindowSurface, winit::{
        application::ApplicationHandler, event::{DeviceEvent, ElementState, MouseButton, WindowEvent}, event_loop, keyboard, window::Window
    }, Display, Program, Surface
};
use my_glium_util::{
    canvas::{
        Canvas, CanvasData,
        traits::{CanvasDrawable, Drawable},
    },
    datastruct::aabb::Aabb,
};

mod app;
mod flock;

fn main() {
    App::run(()).unwrap()
}

struct App {
    main_canva: Canvas,

    dt: f32,
    time: std::time::Instant,
    frame_nb_since_startup: u32,
    start_time: std::time::Instant,
    f_pressed_time: std::time::Instant,
    frame_nb_since_f: u32,
    benching_fps: bool,

    display: Display<WindowSurface>,
    _window: Window,

    mouse_position: (f32, f32),
    mouse_cliking: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &event_loop::ActiveEventLoop) {
        println!("resumed !");
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => match event.physical_key {
                keyboard::PhysicalKey::Code(key_code) => match (event.state, key_code) {
                    (ElementState::Pressed, keyboard::KeyCode::Escape) => event_loop.exit(),
                    (ElementState::Pressed, keyboard::KeyCode::KeyF) => self.print_avg_fps(),
                    (ElementState::Pressed, keyboard::KeyCode::KeyD) => self.starting_fps_bench(),
                    (ElementState::Released, keyboard::KeyCode::KeyD) => self.ending_fps_bench(),
                    _ => (),
                },
                keyboard::PhysicalKey::Unidentified(_) => (),
            },

            WindowEvent::Resized(new_size) => {
                println!("Resized");
                self.display.resize(new_size.into());
                self.main_canva.on_window_resized(new_size.into());
            }
            WindowEvent::Moved(pos) => {
                self.main_canva.on_window_moved(pos.into());
            }

            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let new_pos = position.into();
                //draging
                if self.mouse_cliking && self.main_canva.is_absolute_coord_in(self.mouse_position) {
                    self.main_canva.on_drag(self.mouse_position.into(), new_pos);
                }

                self.mouse_position = new_pos.into();
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match (button, state) {
                (MouseButton::Left, ElementState::Pressed) => {
                    self.mouse_cliking = true;
                    if self.main_canva.is_absolute_coord_in(self.mouse_position) {
                        self.main_canva.on_click(self.mouse_position);
                    }
                }
                (MouseButton::Left, ElementState::Released) => {
                    self.mouse_cliking = false;
                    self.main_canva.on_click_release();
                }
                _ => (),
            },

            _ => (),
        };
    }

    fn device_event(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        _device_id: glium::winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            _ => (),
        }
    }

    fn exiting(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        println!("exiting...");
        self.print_avg_fps();
        event_loop.exit();
    }

    fn new_events(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        cause: glium::winit::event::StartCause,
    ) {
        match cause {
            glium::winit::event::StartCause::Init => (),
            glium::winit::event::StartCause::Poll => {
                //dt handling
                let now = std::time::Instant::now();
                self.dt = now.duration_since(self.time).as_secs_f32();
                self.time = now;
                self.frame_nb_since_startup += 1;

                self.main_canva.update(&DUMMY_CANVA_INFO, self.dt);

                //draw
                self.draw();

                if self.benching_fps {
                    self.frame_nb_since_f += 1;
                }
            }
            _ => (),
        }
    }
}

impl AppTrait for App {
    type InitUserParam = ();

    fn init(
        event_loop: &mut event_loop::EventLoop<()>,
        window: Window,
        display: Display<WindowSurface>,
        _user_param: Self::InitUserParam,
    ) -> Self {
        let frag_shad = std::fs::read_to_string("./shaders/boid.frag")
            .expect("could not load ./shaders/ball.frag");
        let vert_shad = std::fs::read_to_string("./shaders/canva.vert")
            .expect("could not load ./shaders/ball.vert");
        let program = Program::from_source(&display, &vert_shad, &frag_shad, None)
            .expect("could not compile shaders");
    

        let mut main_canva = Canvas::new((0., 0.), program);

        let (r1, r2): (f32, f32) = (
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );

        let boids = (0..10).map(|i|Boid::new((
            r1 / 2. + i as f32 * 4. + f32::cos(i as f32) * (r2 / 150.),
            r2 / 2. + i as f32 * 4. + f32::sin(i as f32) * (r1 / 150.),
        ),i)).collect();

        let flock = Box::new(Flock::new(boids, Aabb::from_min_max((0., 0.), (r1, r2))));
        main_canva.push_elem(flock);

        event_loop.set_control_flow(event_loop::ControlFlow::Poll);

        App {
            main_canva,

            dt: 0.,
            time: std::time::Instant::now(),
            frame_nb_since_startup: 0,
            start_time: std::time::Instant::now(),
            f_pressed_time: std::time::Instant::now(),
            frame_nb_since_f: 0,
            benching_fps: false,
            display,
            _window: window,

            mouse_position: (0., 0.),
            mouse_cliking: false,
        }
    }

    fn draw(&mut self) {
        let mut target = self.display.draw();

        target.clear_color(0.03, 0.03, 0.03, 1.);
        self.main_canva.draw(&self.display, &mut target).unwrap();

        target.finish().unwrap()
    }
}

impl App {
    fn print_avg_fps(&self) {
        println!(
            "average fps since startup :{}",
            self.frame_nb_since_startup as f32
                / self.time.duration_since(self.start_time).as_secs_f32()
        );
    }

    fn starting_fps_bench(&mut self) {
        if self.benching_fps {
            return;
        }
        self.benching_fps = true;
        self.f_pressed_time = std::time::Instant::now();
        println!("starting benching fps (release key to stop");
    }

    fn ending_fps_bench(&mut self) {
        println!(
            "average fps :{}",
            self.frame_nb_since_f as f32
                / self.time.duration_since(self.f_pressed_time).as_secs_f32()
        );

        self.benching_fps = false;
        self.frame_nb_since_f = 0;
    }
}

const DUMMY_CANVA_INFO: CanvasData = CanvasData {
    size: (0., 0.),
    position: (0., 0.),
    frame_nb: 0,

    window_resolution: (0, 0),
};
