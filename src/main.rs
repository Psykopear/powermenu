use druid::widget::{Controller, ControllerHost, Flex, Image, ImageData, WidgetExt};
use druid::{AppLauncher, Env, Event, EventCtx, Widget, WindowDesc};

struct Clickable {
    action: Box<dyn Fn()>,
}

impl Clickable {
    pub fn new(action: impl Fn() + 'static) -> Self {
        Clickable {
            action: Box::new(action),
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for Clickable {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseUp(_) = event {
            (self.action)();
        };
        child.event(ctx, event, data, env)
    }
}

fn ui_builder() -> impl Widget<u32> {
    let shutdown_data = ImageData::from_file("assets/iconmonstr-power-on-off-8-240.png").unwrap();
    let reboot_data = ImageData::from_file("assets/iconmonstr-power-on-off-3-240.png").unwrap();
    let logout_data = ImageData::from_file("assets/iconmonstr-log-out-9-240.png").unwrap();

    let shutdown = ControllerHost::new(
        Image::new(shutdown_data).fix_width(100.).center(),
        Clickable::new(|| {
            println!("SHUTDOWN!");
        }),
    );
    let reboot = ControllerHost::new(
        Image::new(reboot_data).fix_width(100.).center(),
        Clickable::new(|| {
            println!("REBOOT!");
        }),
    );
    let logout = ControllerHost::new(
        Image::new(logout_data).fix_width(100.).center(),
        Clickable::new(|| {
            println!("LOGOUT!");
        }),
    );

    Flex::row()
        .with_child(shutdown, 1.0)
        .with_child(reboot, 1.0)
        .with_child(logout, 1.0)
}

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .resizable(false)
        .show_titlebar(false)
        .window_size((500., 150.00));
    // Looks like I can't init an app without data
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}
