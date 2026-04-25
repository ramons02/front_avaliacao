use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::router::Route;
use crate::components::auth_context::use_auth;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let auth = use_auth();
    let email = use_state(String::new);
    let senha = use_state(String::new);
    let erro = use_state(|| Option::<String>::None);
    let carregando = use_state(|| false);
    let navigator = use_navigator().unwrap();


    let on_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            email.set(input.value());
        })
    };

    let on_senha = {
        let senha = senha.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            senha.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let senha = senha.clone();
        let erro = erro.clone();
        let carregando = carregando.clone();
        let navigator = navigator.clone();
        let fazer_login = auth.fazer_login.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *carregando { return; }

            let email_val = (*email).clone();
            let senha_val = (*senha).clone();
            let erro = erro.clone();
            let carregando = carregando.clone();
            let navigator = navigator.clone();
            let fazer_login = fazer_login.clone();

            carregando.set(true);
            erro.set(None);

            spawn_local(async move {
                match api::login(&email_val, &senha_val).await {
                    Ok(resp) => {
                        fazer_login.emit(resp.token);
                        navigator.push(&Route::Pacientes);
                    }
                    Err(msg) => {
                        erro.set(Some(msg));
                        carregando.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="min-vh-100 d-flex align-items-center justify-content-center"
             style="background: linear-gradient(135deg, #1a365d 0%, #2c5282 50%, #2b6cb0 100%);">
            <div class="card shadow-lg border-0" style="width: 100%; max-width: 420px; border-radius: 16px;">
                <div class="card-body p-5">
                    <div class="text-center mb-4">
                        <i class="bi bi-activity text-primary" style="font-size: 3rem;"></i>
                        <h3 class="fw-bold text-primary mt-2">{"Hups Teste"}</h3>
                        <p class="text-muted small">{"Avaliação Funcional de Retorno ao Esporte"}</p>
                    </div>

                    if let Some(msg) = (*erro).clone() {
                        <div class="alert alert-danger py-2 small">
                            <i class="bi bi-exclamation-triangle-fill me-2"></i>{ msg }
                        </div>
                    }

                    <form onsubmit={on_submit}>
                        <div class="mb-3">
                            <label class="form-label fw-bold text-secondary small">{"E-mail"}</label>
                            <input
                                type="email"
                                class="form-control"
                                placeholder="seuemail@exemplo.com"
                                value={(*email).clone()}
                                oninput={on_email}
                                required=true
                            />
                        </div>
                        <div class="mb-4">
                            <label class="form-label fw-bold text-secondary small">{"Senha"}</label>
                            <input
                                type="password"
                                class="form-control"
                                placeholder="••••••••"
                                value={(*senha).clone()}
                                oninput={on_senha}
                                required=true
                            />
                        </div>
                        <button
                            type="submit"
                            class="btn btn-primary w-100 py-2 fw-bold"
                            disabled={*carregando}
                        >
                            if *carregando {
                                <>
                                    <span class="spinner-border spinner-border-sm me-2"></span>
                                    {"Entrando..."}
                                </>
                            } else {
                                <>
                                    <i class="bi bi-box-arrow-in-right me-2"></i>
                                    {"Entrar"}
                                </>
                            }
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}
