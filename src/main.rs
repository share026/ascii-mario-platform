use leptos::*;
use leptos_router::*;
mod game;
use game::AsciiMario;

#[component]
fn App() -> impl IntoView {
    let (is_vip, set_is_vip) = create_signal(false);
    let (user_npub, set_user_npub) = create_signal(String::new());

    let connect_nostr = move |_| {
        let mock_npub = "npub1mockstripevipuser7890abcdef...".to_string();
        set_user_npub.set(mock_npub);
        set_is_vip.set(true);
    };

    view! {
        <Router>
            <header class="navbar">
                <h2>"🍄 Rust ASCII Mario Arcade"</h2>
                <nav>
                    <A href="/">"Home & Games"</A>
                    <A href="/pricing">"有料プラン / VIP"</A>
                    <A href="/about">"About Us"</A>
                </nav>
            </header>

            <main>
                <Routes>
                    <Route path="/" view=move || view! {
                        <div class="arcade-view">
                            <AsciiMario is_vip=is_vip />
                            <div style="margin-top: 1.5rem; text-align: center;">
                                {move || if is_vip.get() {
                                    view! { <p style="color: #58a6ff;">"✅ Nostrホワイトリスト認証済み: " {user_npub.get()}</p> }
                                } else {
                                    view! {
                                        <button on:click=connect_nostr class="btn-nostr">
                                            "⚡ Nostr 鍵を接続して VIP 特権を有効化 (NIP-04/44)"
                                        </button>
                                    }
                                }}
                            </div>
                        </div>
                    }/>
                    <Route path="/pricing" view=move || view! {
                        <div class="pricing-card">
                            <h2>"👑 管理者特権 & 有料サブスクリプション"</h2>
                            <p style="font-size: 1.4rem; color: #ffd700;">"¥500 / 月 (Stripe または Lightning)"</p>
                            <ul>
                                <li>"✔ NIP-44 暗号化された限定ステージデータの復号アクセス"</li>
                                <li>"✔ マリオ無敵モード (God Mode / ★ アバター)"</li>
                                <li>"✔ スコア 2倍 ブースト"</li>
                                <li>"✔ Stripe 決済完了時に npub を自動ホワイトリスト登録"</li>
                            </ul>
                            <button class="btn-stripe">"Stripe でサブスク登録を開始"</button>
                        </div>
                    }/>
                    <Route path="/about" view=|| view! {
                        <div class="about-us">
                            <h2>"About Us"</h2>
                            <p>"100% Rust (Leptos CSR / WebAssembly) と Nostr リレー暗号化で構築された Web3 アーケードです。"</p>
                            <p>"Cloudflare Pages と GitHub Actions によるサーバーレス＆完全自動デプロイで稼働しています。"</p>
                        </div>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
