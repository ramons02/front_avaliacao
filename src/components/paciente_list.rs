use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::models::Paciente;
use crate::router::Route;
use crate::pdf;

fn calcular_dias(data_cirugia: &str) -> Option<i64> {
    if data_cirugia.is_empty() { return None; }
    let parts: Vec<i32> = data_cirugia.split('-')
        .filter_map(|p| p.parse().ok())
        .collect();
    if parts.len() != 3 { return None; }

    let js_date = js_sys::Date::new_0();
    let now_ms = js_date.get_time();
    let cirugia_str = format!("{}T00:00:00", data_cirugia);
    let cirugia_date = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(&cirugia_str));
    let cirugia_ms = cirugia_date.get_time();
    let diff_dias = ((now_ms - cirugia_ms) / (1000.0 * 60.0 * 60.0 * 24.0)).floor() as i64;
    if diff_dias >= 0 { Some(diff_dias) } else { Some(0) }
}

fn mascara_decimal(raw: &str) -> String {
    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { return String::new(); }
    let num: u64 = digits.parse().unwrap_or(0);
    let inteiro = num / 100;
    let decimal = num % 100;
    format!("{}.{:02}", inteiro, decimal)
}

#[function_component(PacienteListPage)]
pub fn paciente_list_page() -> Html {
    let pacientes = use_state(Vec::<Paciente>::new);
    let nome = use_state(String::new);
    let peso = use_state(String::new);
    let altura = use_state(String::new);
    let data_cirugia = use_state(String::new);
    let membro_op_state = use_state(|| "Direito".to_string());

    let salvando = use_state(|| false);
    let carregando_lista = use_state(|| false);
    let carregando_pdf: UseStateHandle<std::collections::HashMap<String, bool>> = use_state(std::collections::HashMap::new);

    {
        let pacientes = pacientes.clone();
        let carregando_lista = carregando_lista.clone();
        use_effect_with((), move |_| {
            let pacientes = pacientes.clone();
            let carregando_lista = carregando_lista.clone();
            spawn_local(async move {
                carregando_lista.set(true);
                if let Ok(lista) = api::listar_pacientes().await {
                    pacientes.set(lista);
                }
                carregando_lista.set(false);
            });
            || ()
        });
    }

    let recarregar = {
        let pacientes = pacientes.clone();
        let carregando_lista = carregando_lista.clone();
        Callback::from(move |_: ()| {
            let pacientes = pacientes.clone();
            let carregando_lista = carregando_lista.clone();
            spawn_local(async move {
                carregando_lista.set(true);
                if let Ok(lista) = api::listar_pacientes().await {
                    pacientes.set(lista);
                }
                carregando_lista.set(false);
            });
        })
    };

    let cadastrar = {
        let nome = nome.clone();
        let peso = peso.clone();
        let altura = altura.clone();
        let data_cirugia = data_cirugia.clone();
        let lado = membro_op_state.clone();
        let salvando = salvando.clone();
        let recarregar = recarregar.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *salvando { return; }

            if data_cirugia.is_empty() || (*nome).trim().is_empty() {
                return;
            }

            let dias = calcular_dias(&data_cirugia);

            let paciente = Paciente {
                id: None,
                nome: (*nome).clone(),
                peso: (*peso).clone(),
                altura: (*altura).clone(),
                data_cirugia: (*data_cirugia).clone(),
                membro_operado: Some((*lado).clone()),
                dias_pos_operatorio: dias,
            };

            let nome = nome.clone();
            let peso = peso.clone();
            let altura = altura.clone();
            let data_cirugia = data_cirugia.clone();
            let lado = lado.clone();
            let salvando = salvando.clone();
            let recarregar = recarregar.clone();

            salvando.set(true);
            spawn_local(async move {
                match api::salvar_paciente(&paciente).await {
                    Ok(_) => {
                        nome.set(String::new());
                        peso.set(String::new());
                        altura.set(String::new());
                        data_cirugia.set(String::new());
                        lado.set("Direito".into());
                        recarregar.emit(());
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Erro ao salvar: {}", e).into());
                    }
                }
                salvando.set(false);
            });
        })
    };

    html! {
        <div class="container mt-4">
            <div class="card shadow-sm mb-4 border-0">
                <div class="card-header bg-primary text-white">
                    <h4 class="mb-0">{"Cadastrar Novo Paciente"}</h4>
                </div>
                <div class="card-body">
                    <form onsubmit={cadastrar}>
                        <div class="row g-3">
                            <div class="col-md-3">
                                <label class="form-label fw-bold">{"Nome Completo"}</label>
                                <input type="text" class="form-control border-primary" value={(*nome).clone()}
                                    oninput={let nome = nome.clone(); Callback::from(move |e: InputEvent| {
                                        nome.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    })} required=true />
                            </div>
                            <div class="col-md-2">
                                <label class="form-label fw-bold">{"Peso (kg)"}</label>
                                <input type="text" class="form-control border-primary" value={(*peso).clone()}
                                    oninput={let peso = peso.clone(); Callback::from(move |e: InputEvent| {
                                        peso.set(mascara_decimal(&e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));
                                    })} required=true />
                            </div>
                            <div class="col-md-2">
                                <label class="form-label fw-bold">{"Altura (m)"}</label>
                                <input type="text" class="form-control border-primary" value={(*altura).clone()}
                                    oninput={let altura = altura.clone(); Callback::from(move |e: InputEvent| {
                                        altura.set(mascara_decimal(&e.target_unchecked_into::<web_sys::HtmlInputElement>().value()));
                                    })} required=true />
                            </div>
                            <div class="col-md-2">
                                <label class="form-label fw-bold">{"Data Cirurgia"}</label>
                                <input type="date" class="form-control border-primary" value={(*data_cirugia).clone()}
                                    onchange={let dc = data_cirugia.clone(); Callback::from(move |e: Event| {
                                        dc.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    })} required=true />
                            </div>
                            <div class="col-md-2">
                                <label class="form-label fw-bold">{"Lado Operado"}</label>
                                <select class="form-select border-primary" onchange={let l = membro_op_state.clone(); Callback::from(move |e: Event| {
                                    l.set(e.target_unchecked_into::<web_sys::HtmlSelectElement>().value());
                                })}>
                                    <option value="Direito" selected={*membro_op_state == "Direito"}>{"Direito"}</option>
                                    <option value="Esquerdo" selected={*membro_op_state == "Esquerdo"}>{"Esquerdo"}</option>
                                </select>
                            </div>
                            <div class="col-md-1 d-flex align-items-end">
                                <button type="submit" class="btn btn-success w-100 shadow-sm" disabled={*salvando}>
                                    if *salvando {
                                        <span class="spinner-border spinner-border-sm"></span>
                                    } else {
                                        <><i class="bi bi-save me-1"></i>{"Salvar"}</>
                                    }
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>

            <h2 class="text-primary border-bottom pb-2">{"Pacientes em Reabilitação"}</h2>
            <div class="table-responsive">
                <table class="table table-hover shadow-sm border bg-white text-center">
                    <thead class="table-dark">
                        <tr>
                            <th class="text-start ps-3">{"Nome"}</th>
                            <th>{"Membro"}</th>
                            <th>{"Pós-Op"}</th>
                            <th>{"Ações"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for (*pacientes).iter().map(|p| {
                            let p = p.clone();
                            let id = p.id.clone().unwrap_or_default();
                            let p_pdf = p.clone();
                            let id_pdf = id.clone();
                            let pdf_loading = carregando_pdf.clone();
                            let pdf_loading_set = carregando_pdf.clone();

                            let gerar_pdf = Callback::from(move |_: MouseEvent| {
                                let id = id_pdf.clone();
                                let paciente = p_pdf.clone();
                                let pdf_loading = pdf_loading.clone();
                                let pdf_loading_set = pdf_loading_set.clone();
                                spawn_local(async move {
                                    let mut mapa = (*pdf_loading).clone();
                                    mapa.insert(id.clone(), true);
                                    pdf_loading_set.set(mapa);

                                    if let Ok(avaliacoes) = api::buscar_avaliacoes_paciente(&id).await {
                                        if avaliacoes.is_empty() {
                                            web_sys::window()
                                                .unwrap()
                                                .alert_with_message(&format!("O paciente {} ainda não possui avaliações cadastradas.", paciente.nome))
                                                .unwrap();
                                        } else {
                                            let membro_op = paciente.membro_operado.clone().unwrap_or_else(|| "Direito".into());
                                            let html_content = pdf::gerar_html_relatorio(&paciente, &avaliacoes, &membro_op);

                                            // LÓGICA DE DOWNLOAD EM PDF (html2pdf)
                                            let window = web_sys::window().unwrap();
                                            let document = window.document().unwrap();

                                            let container = document.create_element("div").unwrap();
                                            container.set_inner_html(&html_content);

                                            let filename = format!("Relatorio_{}.pdf", paciente.nome.replace(" ", "_"));

                                            let js_code = format!(
                                                "html2pdf().from(arguments[0]).set({{ margin: 0.5, filename: '{}', image: {{ type: 'jpeg', quality: 0.98 }}, html2canvas: {{ scale: 2 }}, jsPDF: {{ unit: 'in', format: 'letter', orientation: 'portrait' }} }}).save();",
                                                filename
                                            );

                                            let val = wasm_bindgen::JsValue::from(container);
                                            let _ = js_sys::Function::new_with_args("el", &js_code).call1(&wasm_bindgen::JsValue::NULL, &val);
                                        }
                                    }

                                    let mut mapa = (*pdf_loading).clone();
                                    mapa.remove(&id);
                                    pdf_loading_set.set(mapa);
                                });
                            });

                            html! {
                                <tr class="align-middle" key={p.id.clone().unwrap_or_default()}>
                                    <td class="text-start ps-3 fw-bold text-primary">{ &p.nome }</td>
                                    <td><span class="badge bg-info text-dark">{ p.membro_operado.clone().unwrap_or_else(|| "Direito".into()) }</span></td>
                                    <td>{ format!("{} dias", p.dias_pos_operatorio.unwrap_or(0)) }</td>
                                    <td>
                                        <div class="d-flex gap-2 justify-content-center">
                                            <Link<Route> to={Route::AvaliacaoNovo { id: p.id.clone().unwrap_or_default() }} classes="btn btn-outline-primary btn-sm">{"Avaliar"}</Link<Route>>
                                            <button class="btn btn-sm btn-outline-danger" onclick={gerar_pdf} disabled={(*carregando_pdf).contains_key(&id)}>
                                                if (*carregando_pdf).contains_key(&id) {
                                                    <span class="spinner-border spinner-border-sm me-1"></span>
                                                } else {
                                                    <i class="bi bi-file-pdf me-1"></i>
                                                }
                                                {" PDF"}
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}