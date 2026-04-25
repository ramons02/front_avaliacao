use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub logado: bool,
    pub fazer_login: Callback<String>,
    pub fazer_logout: Callback<()>,
}

#[hook]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext não encontrado — envolva a árvore com ContextProvider<AuthContext>")
}