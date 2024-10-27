use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

use crate::LifecycleHandler;

pub struct ApplicationDescriptor {
    pub title: &'static str,
    pub height: u32,
    pub width: u32,
}

pub struct ApplicationContext {
    window: Window,
}

enum ApplicationState {
    Uninitialized(ApplicationDescriptor),
    Initialized(ApplicationContext),
    Exited,
}

pub struct Application<H: LifecycleHandler> {
    state: ApplicationState,
    handler: H,
}

impl<H: LifecycleHandler> Application<H> {
    pub fn run(descriptor: ApplicationDescriptor, handler: H) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let mut application = Application {
            state: ApplicationState::Uninitialized(descriptor),
            handler,
        };
        _ = event_loop.run_app(&mut application);
    }
}

impl<H: LifecycleHandler> ApplicationHandler for Application<H> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match &self.state {
            ApplicationState::Uninitialized(descriptor) => {
                let window_attrs = WindowAttributes::default()
                    .with_title(descriptor.title)
                    .with_inner_size(PhysicalSize {
                        width: descriptor.width,
                        height: descriptor.height,
                    });

                let window = event_loop.create_window(window_attrs).unwrap();
                self.state = ApplicationState::Initialized(ApplicationContext { window });
                self.handler.initialize();
            }
            _ => (),
        };
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                match &self.state {
                    ApplicationState::Initialized(_context) => {
                        self.handler.update();
                    }
                    _ => (),
                };
            }
            WindowEvent::CloseRequested => {
                self.handler.shutdown();
                self.state = ApplicationState::Exited;
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        match &self.state {
            ApplicationState::Initialized(context) => {
                context.window.request_redraw();
            }
            _ => (),
        };
    }
}