use yew::prelude::*;
use yew::html::ChildrenProps;
use yew_router::prelude::*;
use crate::auth::{esta_logado, remover_token};
use crate::router::Route;
use crate::components::login::LoginPage;
use crate::components::paciente_list::PacienteListPage;
use crate::components::avaliacao_form::AvaliacaoFormPage;

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <LoginPage /> },
        Route::Pacientes => html! { <ProtectedRoute><PacienteListPage /></ProtectedRoute> },
        Route::AvaliacaoNovo { id } => html! { <ProtectedRoute><AvaliacaoFormPage paciente_id={id} /></ProtectedRoute> },
        Route::Raiz => html! { <Redirect<Route> to={Route::Login} /> },
    }
}

#[function_component(ProtectedRoute)]
fn protected_route(props: &ChildrenProps) -> Html {
    if !esta_logado() {
        return html! { <Redirect<Route> to={Route::Login} /> };
    }
    html! { <>{ props.children.clone() }</> }
}

#[function_component(App)]
pub fn app() -> Html {
    let logado = esta_logado();

    let sair = Callback::from(|_: MouseEvent| {
        remover_token();
        web_sys::window()
            .unwrap()
            .location()
            .set_href("/login")
            .unwrap();
    });

    html! {
        <BrowserRouter>
            if logado {
                <nav class="navbar navbar-expand-lg navbar-dark bg-primary shadow-sm mb-0">
                    <div class="container">
                        <span class="navbar-brand fw-bold">
                            <i class="bi bi-activity me-2"></i>{"Hups Teste"}
                        </span>
                        <div class="navbar-nav me-auto">
                            <Link<Route> to={Route::Pacientes} classes="nav-link text-white">
                                <i class="bi bi-people me-1"></i>{"Pacientes"}
                            </Link<Route>>
                        </div>
                        <button class="btn btn-outline-light btn-sm" onclick={sair}>
                            <i class="bi bi-box-arrow-right me-1"></i>{"Sair"}
                        </button>
                    </div>
                </nav>
            }
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
