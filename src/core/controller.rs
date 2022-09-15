use druid::{Widget, widget::Controller, EventCtx, Event, Env, HotKey, SysMods, commands, Target, Data};

pub struct CopyCutPasteController;

impl<T: Data, W: Widget<T>> Controller<T, W> for CopyCutPasteController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::KeyDown(key_event) => match key_event {
                k_e if (HotKey::new(SysMods::Cmd, "c")).matches(k_e) => {
                    ctx.submit_command(commands::COPY.to(Target::Auto));
                    ctx.set_handled();
                }
                k_e if (HotKey::new(SysMods::Cmd, "x")).matches(k_e) => {
                    ctx.submit_command(commands::CUT.to(Target::Auto));
                    ctx.set_handled();
                }
                k_e if (HotKey::new(SysMods::Cmd, "v")).matches(k_e) => {
                    ctx.submit_command(commands::PASTE.to(Target::Auto));
                    ctx.set_handled();
                }
                _ => (),
            },
            _ => (),
        }
        child.event(ctx, event, data, env);
    }
}
