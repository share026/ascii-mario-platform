use leptos::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

const WIDTH: usize = 36;
const HEIGHT: usize = 9;

#[component]
pub fn AsciiMario(is_vip: ReadSignal<bool>) -> impl IntoView {
    let (mario_x, set_mario_x) = create_signal(3);
    let (mario_y, set_mario_y) = create_signal(0);
    let (score, set_score) = create_signal(0);
    let (screen, set_screen) = create_signal(String::new());

    create_effect(move |_| {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let handle_keydown = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowRight" | "d" | "D" => set_mario_x.update(|x| *x = (*x + 1).min(WIDTH - 2)),
                "ArrowLeft" | "a" | "A" => set_mario_x.update(|x| *x = x.saturating_sub(1).max(1)),
                " " | "ArrowUp" | "w" | "W" => {
                    if mario_y.get() == 0 {
                        set_mario_y.set(3);
                    }
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        let _ = window.add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref());
        handle_keydown.forget();
    });

    spawn_local(async move {
        loop {
            TimeoutFuture::new(100).await;

            if mario_y.get() > 0 {
                set_mario_y.update(|y| *y -= 1);
            }

            let bonus = if is_vip.get() { 2 } else { 1 };
            set_score.update(|s| *s += bonus);

            let mut buffer = vec![vec![' '; WIDTH]; HEIGHT];

            for x in 0..WIDTH {
                buffer[HEIGHT - 1][x] = '=';
            }

            buffer[HEIGHT - 2][14] = '|';
            buffer[HEIGHT - 2][15] = '|';
            buffer[HEIGHT - 3][14] = '[';
            buffer[HEIGHT - 3][15] = ']';

            if is_vip.get() {
                buffer[HEIGHT - 5][8] = '?';
                buffer[HEIGHT - 5][9] = '$';
            }

            let mx = mario_x.get();
            let my = (HEIGHT - 2).saturating_sub(mario_y.get());
            if my < HEIGHT && mx < WIDTH {
                buffer[my][mx] = if is_vip.get() { '★' } else { 'M' };
            }

            let rendered: String = buffer.into_iter()
                .map(|row| row.into_iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n");

            set_screen.set(rendered);
        }
    });

    view! {
        <div class="game-container">
            <div class="hud">
                <span>"SCORE: " {score}</span>
                <span class="vip-badge">
                    {move || if is_vip.get() { " ⭐ VIP ACTIVE (God Mode / 2x Speed)" } else { " (Free Tier)" }}
                </span>
            </div>
            <pre class="ascii-canvas">{screen}</pre>
            <p class="controls">"操作: [A / D] 移動 | [SPACE / W] ジャンプ"</p>
        </div>
    }
}
