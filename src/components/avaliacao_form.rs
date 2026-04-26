use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::models::{Avaliacao, Paciente, calcular_lsi, parse_decimal};
use crate::router::Route;
use crate::components::numeric_input::NumericInput;
use crate::components::lsi_chart::LsiChart;

#[derive(Properties, PartialEq)]
pub struct AvaliacaoFormProps {
    pub paciente_id: String,
}

fn media_lsi_form(single: f64, triple: f64, crossover: f64, six: f64) -> f64 {
    let vals = [single, triple, crossover, six];
    let pos: Vec<f64> = vals.iter().copied().filter(|&v| v > 0.0).collect();
    if pos.is_empty() { return 0.0; }
    pos.iter().sum::<f64>() / pos.len() as f64
}

fn badge(lsi: f64) -> Html {
    if lsi >= 90.0 {
        html! { <span class="badge bg-success">{"APTO"}</span> }
    } else {
        html! { <span class="badge bg-danger">{"NÃO APTO"}</span> }
    }
}

#[function_component(AvaliacaoFormPage)]
pub fn avaliacao_form_page(props: &AvaliacaoFormProps) -> Html {
    let paciente: UseStateHandle<Option<Paciente>> = use_state(|| None);
    let single_dir = use_state(String::new);
    let single_esq = use_state(String::new);
    let triple_dir = use_state(String::new);
    let triple_esq = use_state(String::new);
    let crossover_dir = use_state(String::new);
    let crossover_esq = use_state(String::new);
    let six_dir = use_state(String::new);
    let six_esq = use_state(String::new);
    let salvando = use_state(|| false);
    let mensagem_sucesso = use_state(|| Option::<String>::None);
    let erro_msg = use_state(|| Option::<String>::None);
    let avaliacoes_hist = use_state(Vec::<Avaliacao>::new);
    let navigator = use_navigator().unwrap();

    // Busca paciente pelo ID da rota
    {
        let paciente_id = props.paciente_id.clone();
        let paciente = paciente.clone();
        use_effect_with(paciente_id.clone(), move |id| {
            let id = id.clone();
            let paciente = paciente.clone();
            spawn_local(async move {
                match api::buscar_paciente(&id).await {
                    Ok(p) => paciente.set(Some(p)),
                    Err(e) => log::error!("Erro ao buscar paciente: {}", e),
                }
            });
        });
    }

    // Busca histórico de avaliações
    {
        let paciente_id = props.paciente_id.clone();
        let avaliacoes_hist = avaliacoes_hist.clone();
        use_effect_with(paciente_id.clone(), move |id| {
            let id = id.clone();
            let avaliacoes_hist = avaliacoes_hist.clone();
            spawn_local(async move {
                if let Ok(lista) = api::buscar_avaliacoes_paciente(&id).await {
                    avaliacoes_hist.set(lista);
                }
            });
        });
    }

    // Cálculos LSI em tempo real
    let lsi_single = calcular_lsi(parse_decimal(&single_dir), parse_decimal(&single_esq));
    let lsi_triple = calcular_lsi(parse_decimal(&triple_dir), parse_decimal(&triple_esq));
    let lsi_crossover = calcular_lsi(parse_decimal(&crossover_dir), parse_decimal(&crossover_esq));
    let lsi_six = calcular_lsi(parse_decimal(&six_dir), parse_decimal(&six_esq));
    let media = media_lsi_form(lsi_single, lsi_triple, lsi_crossover, lsi_six);
    let esta_apto = media >= 90.0;

    let on_salvar = {
        let paciente_id = props.paciente_id.clone();
        let single_dir = single_dir.clone();
        let single_esq = single_esq.clone();
        let triple_dir = triple_dir.clone();
        let triple_esq = triple_esq.clone();
        let crossover_dir = crossover_dir.clone();
        let crossover_esq = crossover_esq.clone();
        let six_dir = six_dir.clone();
        let six_esq = six_esq.clone();
        let salvando = salvando.clone();
        let mensagem_sucesso = mensagem_sucesso.clone();
        let erro_msg = erro_msg.clone();
        let navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            if *salvando { return; }

            let data_hoje = {
                let d = js_sys::Date::new_0();
                let ano = d.get_full_year();
                let mes = d.get_month() + 1;
                let dia = d.get_date();
                format!("{:04}-{:02}-{:02}", ano, mes, dia)
            };

            let av = Avaliacao {
                id: None,
                paciente_id: paciente_id.clone(),
                data_avaliacao: data_hoje,
                single_hop_dir: parse_decimal(&single_dir),
                single_hop_esq: parse_decimal(&single_esq),
                triple_hop_dir: parse_decimal(&triple_dir),
                triple_hop_esq: parse_decimal(&triple_esq),
                crossover_dir: parse_decimal(&crossover_dir),
                crossover_esq: parse_decimal(&crossover_esq),
                six_meter_dir: parse_decimal(&six_dir),
                six_meter_esq: parse_decimal(&six_esq),
            };

            let salvando = salvando.clone();
            let mensagem_sucesso = mensagem_sucesso.clone();
            let erro_msg = erro_msg.clone();
            let navigator = navigator.clone();

            salvando.set(true);
            spawn_local(async move {
                match api::salvar_avaliacao(&av).await {
                    Ok(_) => {
                        mensagem_sucesso.set(Some("Avaliação salva com sucesso!".into()));
                        salvando.set(false);
                        let navigator = navigator.clone();
                        gloo_utils::window().set_timeout_with_callback_and_timeout_and_arguments_0(
                            &wasm_bindgen::closure::Closure::once_into_js(move || {
                                navigator.push(&Route::Pacientes);
                            }).unchecked_ref(),
                            2000,
                        ).unwrap();
                    }
                    Err(e) => {
                        erro_msg.set(Some(e));
                        salvando.set(false);
                    }
                }
            });
        })
    };

    let fundo_resultado = if esta_apto { "bg-success text-white" } else { "bg-danger text-white" };

    html! {
        <div class="container mt-4 bg-white p-4 shadow rounded border">
            <h2 class="text-primary border-bottom pb-2">{"Nova Avaliação Funcional"}</h2>

            // ── Paciente ──
            <div class="row mt-3 mb-4">
                <div class="col-md-12">
                    <label class="fw-bold text-secondary">{"Paciente"}</label>
                    <div class="form-control bg-light border-primary fw-bold text-primary">
                        { (*paciente).as_ref().map(|p| p.nome.clone()).unwrap_or_else(|| "Carregando...".into()) }
                    </div>
                </div>
            </div>

            // ── 4 Testes ──
            <div class="row mt-3">
                { input_teste("1. Single Leg Hop (cm)", &single_dir, &single_esq, "Ex: 150,0") }
                { input_teste("2. Triple Hop (cm)", &triple_dir, &triple_esq, "Ex: 420,0") }
                { input_teste("3. Crossover Hop (cm)", &crossover_dir, &crossover_esq, "Ex: 380,0") }
                { input_teste("4. 6 Meter Timed (seg)", &six_dir, &six_esq, "Ex: 3,45") }
            </div>

            // ── LSI Individual ──
            <div class="row mt-2 p-3 bg-light rounded border mx-0">
                <div class="col-12">
                    <h5 class="text-secondary border-bottom pb-2 mb-3">{"Simetria Individual (LSI)"}</h5>
                    { lsi_row("Simetria Single Hop", lsi_single) }
                    { lsi_row("Simetria Triple Hop", lsi_triple) }
                    { lsi_row("Simetria Crossover", lsi_crossover) }
                    { lsi_row("Simetria 6 Meter Timed", lsi_six) }
                </div>
            </div>

            // ── Resultado Geral ──
            <div class={format!("mt-4 p-4 rounded text-center shadow-sm {}", fundo_resultado)}>
                <h4 class="mb-1">{"Simetria Média (LSI)"}</h4>
                <h2 class="display-5 fw-bold">{ format!("{:.1}%", media) }</h2>
                <hr class="border-white opacity-50" />
                <h3 class="fw-bold text-uppercase">
                    { if esta_apto { "Paciente Liberado" } else { "Manter Fisioterapia" } }
                </h3>
            </div>

            // ── Evolução do LSI ──
            if !(*avaliacoes_hist).is_empty() {
                <div class="mt-4 p-3 bg-light rounded border">
                    <h5 class="text-secondary border-bottom pb-2 mb-3">{"Evolução do LSI"}</h5>
                    <LsiChart avaliacoes={(*avaliacoes_hist).clone()} canvas_id={"lsi-evolution-chart"} />
                </div>
            }

            // ── Mensagens ──
            if let Some(msg) = (*mensagem_sucesso).clone() {
                <div class="alert alert-success mt-3 py-2 text-center shadow-sm">
                    <i class="bi bi-check-circle-fill me-2"></i>{ msg }
                </div>
            }
            if let Some(msg) = (*erro_msg).clone() {
                <div class="alert alert-danger mt-3 py-2 text-center">{ msg }</div>
            }

            // ── Botões ──
            <div class="mt-4 d-flex gap-3 justify-content-center">
                <button
                    class="btn btn-success btn-lg px-5 shadow"
                    onclick={on_salvar}
                    disabled={*salvando}
                >
                    if *salvando {
                        <>
                            <span class="spinner-border spinner-border-sm me-2"></span>
                            {"Salvando..."}
                        </>
                    } else {
                        <>
                            <i class="bi bi-save me-2"></i>
                            {"Salvar Avaliação"}
                        </>
                    }
                </button>
            </div>
        </div>
    }
}

fn input_teste(
    label: &str,
    dir: &UseStateHandle<String>,
    esq: &UseStateHandle<String>,
    placeholder: &str,
) -> Html {
    let dir_val = (**dir).clone();
    let esq_val = (**esq).clone();
    let on_dir = {
        let dir = dir.clone();
        Callback::from(move |v: String| dir.set(v))
    };
    let on_esq = {
        let esq = esq.clone();
        Callback::from(move |v: String| esq.set(v))
    };
    let label = label.to_string();
    let placeholder = placeholder.to_string();

    html! {
        <div class="col-md-6 mb-4">
            <label class="fw-bold text-secondary">{ label }</label>
            <div class="input-group">
                <span class="input-group-text">{"Dir"}</span>
                <NumericInput value={dir_val} on_change={on_dir} placeholder={placeholder.clone()} />
                <span class="input-group-text">{"Esq"}</span>
                <NumericInput value={esq_val} on_change={on_esq} placeholder={placeholder} />
            </div>
        </div>
    }
}

fn lsi_row(label: &str, lsi: f64) -> Html {
    html! {
        <div class="d-flex justify-content-between mb-2">
            <span>{ format!("{}: ", label) }<strong>{ format!("{:.1}%", lsi) }</strong></span>
            { badge(lsi) }
        </div>
    }
}