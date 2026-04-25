use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "auth_token";

pub fn salvar_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token);
}

pub fn obter_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_KEY).ok()
}

pub fn remover_token() {
    LocalStorage::delete(TOKEN_KEY);
}

pub fn esta_logado() -> bool {
    obter_token().map(|t| !t.is_empty()).unwrap_or(false)
}