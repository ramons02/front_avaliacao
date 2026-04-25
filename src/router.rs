use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/pacientes")]
    Pacientes,
    #[at("/avaliacoes/novo/:id")]
    AvaliacaoNovo { id: String },
    #[at("/")]
    #[not_found]
    Raiz,
}
