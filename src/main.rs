use druid::widget::{Controller, ControllerHost, Flex, Image, ImageData, WidgetExt};
use druid::{AppLauncher, Env, Event, EventCtx, Widget, WindowDesc};
use std::process::Command;

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
            Command::new("shutdown").spawn().expect("Can't shutdown");
        }),
    );
    let reboot = ControllerHost::new(
        Image::new(reboot_data).fix_width(100.).center(),
        Clickable::new(|| {
            Command::new("reboot").spawn().expect("Can't shutdown");
        }),
    );
    let logout = ControllerHost::new(
        Image::new(logout_data).fix_width(100.).center(),
        Clickable::new(|| {
            let mut cmd = Command::new("i3-msg");
            cmd.arg("exit");
            cmd.spawn().expect("Can't logout of i3");
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
