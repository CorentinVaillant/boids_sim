use glium::{
    Display, backend,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler, error::EventLoopError, event_loop::EventLoop,
        window::Window,
    },
};

pub trait AppTrait
where
    Self: ApplicationHandler + Sized,
{
    type InitUserParam;

    fn init(
        event_loop: &mut EventLoop<()>,
        window: Window,
        display: Display<WindowSurface>,
        user_param: Self::InitUserParam,
    ) -> Self;
    fn draw(&mut self);

    fn run(user_param: Self::InitUserParam) -> Result<(), EventLoopError> {
        let mut event_loop = EventLoop::new().unwrap();
        let (window, display) = backend::glutin::SimpleWindowBuilder::new()
            .with_title("Bouncing ball !")
            .build(&event_loop);

        let mut app = Self::init(&mut event_loop, window, display, user_param);

        event_loop.run_app(&mut app)
    }
}
