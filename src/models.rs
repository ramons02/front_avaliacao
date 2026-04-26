use serde::{Deserialize, Deserializer, Serialize};

// Aceita tanto String quanto Number do Java (Double/BigDecimal → String formatada)
fn de_numero_ou_string<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    use serde::de::{self, Visitor};
    struct V;
    impl<'de> Visitor<'de> for V {
        type Value = String;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "número ou string")
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<String, E> { Ok(v.to_owned()) }
        fn visit_string<E: de::Error>(self, v: String) -> Result<String, E> { Ok(v) }
        fn visit_f64<E: de::Error>(self, v: f64) -> Result<String, E> { Ok(format!("{:.2}", v)) }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<String, E> { Ok(v.to_string()) }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<String, E> { Ok(v.to_string()) }
    }
    d.deserialize_any(V)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paciente {
    pub id: Option<String>,
    pub nome: String,
    #[serde(deserialize_with = "de_numero_ou_string")]
    pub peso: String,
    #[serde(deserialize_with = "de_numero_ou_string")]
    pub altura: String,
    #[serde(rename = "dataCirurgia")]
    pub data_cirugia: String,
    #[serde(rename = "membroOp")]
    pub membro_operado: Option<String>,
    #[serde(rename = "diasPosRlca")]
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

pub fn calcular_lsi(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 { return 0.0; }
    (a.min(b) / a.max(b)) * 100.0
}

pub fn parse_decimal(s: &str) -> f64 {
    s.replace(',', ".").parse::<f64>().unwrap_or(0.0)
}