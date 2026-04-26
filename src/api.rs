use gloo_net::http::Request;
use crate::auth::obter_token;
use crate::models::{Avaliacao, LoginRequest, LoginResponse, Paciente};

const BASE: &str = "/api/v1";






fn bearer() -> String {
    format!("Bearer {}", obter_token().unwrap_or_default())
}

pub async fn login(email: &str, senha: &str) -> Result<LoginResponse, String> {
    let body = LoginRequest { email: email.to_string(), senha: senha.to_string() };
    let resp = Request::post(&format!("{BASE}/auth/login"))
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<LoginResponse>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Credenciais inválidas ({})", resp.status()))
    }
}

pub async fn listar_pacientes() -> Result<Vec<Paciente>, String> {
    let resp = Request::get(&format!("{BASE}/pacientes"))
        .header("Authorization", &bearer())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 401 || resp.status() == 403 {
        return Err("UNAUTHORIZED".to_string());
    }
    if !resp.ok() {
        return Err(format!("Servidor retornou erro {}", resp.status()));
    }
    resp.json::<Vec<Paciente>>().await.map_err(|e| e.to_string())
}

pub async fn buscar_paciente(id: &str) -> Result<Paciente, String> {
    let resp = Request::get(&format!("{BASE}/pacientes/{id}"))
        .header("Authorization", &bearer())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 401 || resp.status() == 403 {
        return Err("UNAUTHORIZED".to_string());
    }
    if !resp.ok() {
        return Err(format!("Servidor retornou erro {}", resp.status()));
    }
    resp.json::<Paciente>().await.map_err(|e| e.to_string())
}

pub async fn salvar_paciente(paciente: &Paciente) -> Result<Paciente, String> {
    if let Ok(json) = serde_json::to_string(paciente) {
        web_sys::console::log_1(&format!("[API] POST /pacientes → {}", json).into());
    }

    let resp = Request::post(&format!("{BASE}/pacientes"))
        .header("Authorization", &bearer())
        .header("Content-Type", "application/json")
        .json(paciente)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<Paciente>().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let corpo = resp.text().await.unwrap_or_default();
        web_sys::console::error_1(&format!("[API] Erro {}: {}", status, corpo).into());
        Err(format!("Erro {} — {}", status, corpo))
    }
}

pub async fn salvar_avaliacao(av: &Avaliacao) -> Result<Avaliacao, String> {
    let resp = Request::post(&format!("{BASE}/avaliacoes"))
        .header("Authorization", &bearer())
        .header("Content-Type", "application/json")
        .json(av)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<Avaliacao>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Erro ao salvar avaliação ({})", resp.status()))
    }
}

pub async fn buscar_avaliacoes_paciente(paciente_id: &str) -> Result<Vec<Avaliacao>, String> {
    let resp = Request::get(&format!("{BASE}/avaliacoes/paciente/{paciente_id}"))
        .header("Authorization", &bearer())
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == 401 || resp.status() == 403 {
        return Err("UNAUTHORIZED".to_string());
    }
    if !resp.ok() {
        return Err(format!("Servidor retornou erro {}", resp.status()));
    }
    resp.json::<Vec<Avaliacao>>().await.map_err(|e| e.to_string())
}

