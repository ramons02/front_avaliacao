use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::models::{Avaliacao, calcular_lsi};

#[wasm_bindgen(inline_js = "
export function renderLsiChart(canvasId, labels, meta, single, triple, crossover, six) {
    const ctx = document.getElementById(canvasId);
    if (!ctx) return;
    if (ctx._chart) { ctx._chart.destroy(); }
    ctx._chart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                { label: 'Meta (90%)', data: meta, borderColor: '#e74c3c', borderDash: [5,5], pointRadius: 0, fill: false },
                { label: 'Single Hop', data: single, borderColor: '#3498db', fill: false },
                { label: 'Triple Hop', data: triple, borderColor: '#2ecc71', fill: false },
                { label: 'Crossover', data: crossover, borderColor: '#e67e22', fill: false },
                { label: '6 Metros', data: six, borderColor: '#9b59b6', fill: false }
            ]
        },
        options: {
            responsive: true,
            plugins: { legend: { position: 'bottom' } },
            scales: { y: { min: 0, max: 100, ticks: { callback: v => v + '%' } } }
        }
    });
}
")]
extern "C" {
    fn renderLsiChart(
        canvas_id: &str,
        labels: JsValue,
        meta: JsValue,
        single: JsValue,
        triple: JsValue,
        crossover: JsValue,
        six: JsValue,
    );
}

#[derive(Properties, PartialEq)]
pub struct LsiChartProps {
    pub avaliacoes: Vec<Avaliacao>,
    pub canvas_id: String,
}

#[function_component(LsiChart)]
pub fn lsi_chart(props: &LsiChartProps) -> Html {
    let canvas_id = props.canvas_id.clone();
    let avaliacoes = props.avaliacoes.clone();

    use_effect_with(avaliacoes.clone(), move |avs| {
        let labels = js_sys::Array::new();
        let meta_arr = js_sys::Array::new();
        let single_arr = js_sys::Array::new();
        let triple_arr = js_sys::Array::new();
        let crossover_arr = js_sys::Array::new();
        let six_arr = js_sys::Array::new();

        for av in avs.iter() {
            labels.push(&JsValue::from_str(&av.data_avaliacao));
            meta_arr.push(&JsValue::from_f64(90.0));
            single_arr.push(&JsValue::from_f64(calcular_lsi(av.single_hop_dir, av.single_hop_esq)));
            triple_arr.push(&JsValue::from_f64(calcular_lsi(av.triple_hop_dir, av.triple_hop_esq)));
            crossover_arr.push(&JsValue::from_f64(calcular_lsi(av.crossover_dir, av.crossover_esq)));
            six_arr.push(&JsValue::from_f64(calcular_lsi(av.six_meter_dir, av.six_meter_esq)));
        }

        renderLsiChart(
            &canvas_id,
            labels.into(),
            meta_arr.into(),
            single_arr.into(),
            triple_arr.into(),
            crossover_arr.into(),
            six_arr.into(),
        );
    });

    html! {
        <canvas id={props.canvas_id.clone()} style="max-height: 300px;"></canvas>
    }
}