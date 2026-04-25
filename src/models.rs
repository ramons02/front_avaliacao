use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paciente {
    pub id: Option<String>,
    pub nome: String,
    pub peso: String,
    pub altura: String,
    #[serde(rename = "dataCirugia")]
    pub data_cirugia: String,

    // AQUI ESTAVA O ERRO: Mudei de 'menbro' para 'membro' (com M)
    #[serde(rename = "membro_operado")]
    pub membro_operado: Option<String>,

    #[serde(rename = "diasPosOperatorio")]
    pub dias_pos_operatorio: Option<i64>,
}

impl Default for Paciente {
    fn default() -> Self {
        Self {
            id: None,
            nome: String::new(),
            peso: String::new(),
            altura: String::new(),
            data_cirugia: String::new(),
            membro_operado: Some("Direito".to_string()),
            dias_pos_operatorio: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Avaliacao {
    pub id: Option<String>,
    #[serde(rename = "pacienteId")]
    pub paciente_id: String,
    #[serde(rename = "dataAvaliacao")]
    pub data_avaliacao: String,
    #[serde(rename = "singleHopDireita")]
    pub single_hop_dir: f64,
    #[serde(rename = "singleHopEsquerda")]
    pub single_hop_esq: f64,
    #[serde(rename = "tripleHopDireita")]
    pub triple_hop_dir: f64,
    #[serde(rename = "tripleHopEsquerda")]
    pub triple_hop_esq: f64,
    #[serde(rename = "crossoverHopDireita")]
    pub crossover_dir: f64,
    #[serde(rename = "crossoverHopEsquerda")]
    pub crossover_esq: f64,
    #[serde(rename = "sixMeterDireita")]
    pub six_meter_dir: f64,
    #[serde(rename = "sixMeterEsquerda")]
    pub six_meter_esq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

// ── LÓGICA DE CORES DEFINITIVA ──
pub fn definir_cores_por_membro(membro_op: &str) -> (String, String) {
    if membro_op.to_lowercase().contains("dir") {
        ("Esquerda (Azul)".to_string(), "Direita (Vermelho)".to_string())
    } else {
        ("Direita (Azul)".to_string(), "Esquerda (Vermelho)".to_string())
    }
}

pub fn calcular_lsi(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 { return 0.0; }
    (a.min(b) / a.max(b)) * 100.0
}

pub fn media_lsi(av: &Avaliacao) -> f64 {
    let valores = [
        calcular_lsi(av.single_hop_dir, av.single_hop_esq),
        calcular_lsi(av.triple_hop_dir, av.triple_hop_esq),
        calcular_lsi(av.crossover_dir, av.crossover_esq),
        calcular_lsi(av.six_meter_dir, av.six_meter_esq),
    ];
    let positivos: Vec<f64> = valores.iter().copied().filter(|&v| v > 0.0).collect();
    if positivos.is_empty() { return 0.0; }
    positivos.iter().sum::<f64>() / positivos.len() as f64
}

pub fn parse_decimal(s: &str) -> f64 {
    s.replace(',', ".").parse::<f64>().unwrap_or(0.0)
}