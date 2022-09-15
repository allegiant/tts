use super::controller::CopyCutPasteController;
use super::tts;
use druid::{
    widget::{Align, Button, Flex, TextBox},
    AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, Lens)]
struct HelloState {
    play_url: String,
    play_btn: String,
    play_btn_status: bool,
    name: String,
}

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("语音播报");

pub fn init() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((480.0, 250.0));

    let initial_state = HelloState {
        name: "".into(),
        play_url: "".into(),
        play_btn: "开始".into(),
        play_btn_status: false,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    let play_textbox = TextBox::new()
        .with_placeholder("请输入播报地址")
        .fix_width(400.0)
        .lens(HelloState::play_url);

    let textbox = TextBox::new()
        .with_placeholder("请输入测试内容")
        .fix_width(400.0)
        .lens(HelloState::name);

    let button = Button::new("测试").on_click(|_ctx, data: &mut HelloState, _env| {
        tts::speak(&data.name);
    });

    let play_button = Button::new(|data: &HelloState, _env: &Env| format!("{}", data.play_btn))
        .on_click(|_ctx, data: &mut HelloState, _env| {
            if data.play_btn_status {
                data.play_btn_status = false;
                data.play_btn = "开始".into();
                tts::stop_loop_speak();
            } else {
                data.play_btn_status = true;
                data.play_btn = "结束".into();
                if data.play_url.len() > 0 {
                    {
                        tts::loop_speak(data.play_url.clone());
                    };
                } else {
                    data.play_btn_status = false;
                    data.play_btn = "开始".into();
                    println!("未获取到url");
                }
            }
        });

    let test_row = Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Center)
        .must_fill_main_axis(true)
        .with_child(textbox)
        .with_spacer(8.0)
        .with_flex_child(button, 1.0);

    let play_row = Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Center)
        .must_fill_main_axis(true)
        .with_child(play_textbox)
        .with_spacer(8.0)
        .with_flex_child(play_button, 1.0);

    let layout = Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Center)
        .must_fill_main_axis(true)
        .with_spacer(120.0)
        .with_child(test_row)
        .with_spacer(20.0)
        .with_child(play_row);

    Align::centered(layout).controller(CopyCutPasteController)
}
