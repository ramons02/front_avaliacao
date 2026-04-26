use plotters::prelude::*;
use crate::models::{Avaliacao, Paciente};

// ── Gráfico de barras (Protegido contra erro 101 e ciente do membro operado) ────
pub fn draw_comparison_bars(
    _paciente: &Paciente,
    ultima: &Avaliacao,
    membro_op: &str,
) -> String {
    let testes = ["Single Hop", "Triple Hop", "Crossover", "6m Timed"];

    // Identifica qual lado é o operado para organizar as barras (Azul = Sadio, Vermelho = Operado)
    let op_is_dir = membro_op.to_lowercase().contains("dir");

    let pares: [(f64, f64); 4] = if op_is_dir {
        [
            (ultima.single_hop_esq, ultima.single_hop_dir),
            (ultima.triple_hop_esq, ultima.triple_hop_dir),
            (ultima.crossover_esq,  ultima.crossover_dir),
            (ultima.six_meter_esq,  ultima.six_meter_dir),
        ]
    } else {
        [
            (ultima.single_hop_dir, ultima.single_hop_esq),
            (ultima.triple_hop_dir, ultima.triple_hop_esq),
            (ultima.crossover_dir,  ultima.crossover_esq),
            (ultima.six_meter_dir,  ultima.six_meter_esq),
        ]
    };

    let lsi: [f64; 4] = pares.map(|(sadio, operado)| {
        if sadio <= 0.0 { 0.0 } else { (operado / sadio) * 100.0 }
    });

    let max_val = pares.iter().map(|(s, o)| s.max(*o)).fold(0.0f64, f64::max) * 1.3;
    let azul     = RGBColor(44, 82, 130);
    let vermelho = RGBColor(197, 48, 48);

    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (620, 260)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Simetria Biomecânica: Sadio vs Operado", ("sans-serif", 16).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0f64..12.0f64, 0.0f64..max_val)
            .unwrap();

        chart.configure_mesh()
            .disable_x_mesh()
            .disable_x_axis()
            .y_desc("Desempenho (cm / s)")
            .draw().unwrap();

        for (i, (sadio, operado)) in pares.iter().enumerate() {
            let g = i as f64 * 3.0;
            chart.draw_series(std::iter::once(Rectangle::new([(g + 0.2, 0.0), (g + 1.2, *sadio)], azul.filled()))).unwrap();
            chart.draw_series(std::iter::once(Rectangle::new([(g + 1.4, 0.0), (g + 2.4, *operado)], vermelho.filled()))).unwrap();

            let lsi_val = lsi[i];
            chart.draw_series(std::iter::once(Text::new(format!("{:.1}% LSI", lsi_val), (g + 0.9, max_val * 0.85), ("sans-serif", 10).into_font()))).unwrap();
            chart.draw_series(std::iter::once(Text::new(testes[i], (g + 0.9, -max_val * 0.05), ("sans-serif", 11).into_font()))).unwrap();
        }
        root.present().unwrap();
    }
    svg
}

// ── Gerador de Relatório Profissional (Agora com Membro Operado dinâmico) ──────
pub fn gerar_html_relatorio(paciente: &Paciente, avaliacoes: &[Avaliacao], membro_op: &str) -> String {
    let (ultima, anterior) = match avaliacoes.len() {
        0 => return html_sem_avaliacoes(paciente),
        1 => (&avaliacoes[0], None),
        n => (&avaliacoes[n-1], Some(&avaliacoes[n-2])),
    };

    // Lógica LSI baseada no membro operado informado
    let op_is_dir = membro_op.to_lowercase().contains("dir");
    let calc_lsi_clinico = |dir: f64, esq: f64| {
        if op_is_dir { (dir / esq) * 100.0 } else { (esq / dir) * 100.0 }
    };

    let lsi_atual = [
        calc_lsi_clinico(ultima.single_hop_dir, ultima.single_hop_esq),
        calc_lsi_clinico(ultima.triple_hop_dir, ultima.triple_hop_esq),
        calc_lsi_clinico(ultima.crossover_dir,  ultima.crossover_esq),
        calc_lsi_clinico(ultima.six_meter_dir,  ultima.six_meter_esq),
    ];

    let media = lsi_atual.iter().sum::<f64>() / 4.0;
    let cor_status = if media >= 90.0 { "#2F855A" } else { "#C53030" };
    let texto_status = if media >= 90.0 { "APTO PARA RETORNO ESPORTIVO" } else { "MANTER PROTOCOLO DE REABILITAÇÃO" };

    let svg_grafico = draw_comparison_bars(paciente, ultima, membro_op);

    // Renderização das linhas (Pré-processadas para estabilidade do compilador)
    let mut linhas = String::new();
    let nomes = ["Single Leg Hop", "Triple Hop", "Crossover Hop", "6m Timed Hop"];
    let v_dir = [ultima.single_hop_dir, ultima.triple_hop_dir, ultima.crossover_dir, ultima.six_meter_dir];
    let v_esq = [ultima.single_hop_esq, ultima.triple_hop_esq, ultima.crossover_esq, ultima.six_meter_esq];

    for i in 0..4 {
        let (sadio, operado) = if op_is_dir { (v_esq[i], v_dir[i]) } else { (v_dir[i], v_esq[i]) };
        let lsi_at = lsi_atual[i];

        let lsi_ant = anterior.map(|a| calc_lsi_clinico(
            if i == 0 { a.single_hop_dir } else if i == 1 { a.triple_hop_dir } else if i == 2 { a.crossover_dir } else { a.six_meter_dir },
            if i == 0 { a.single_hop_esq } else if i == 1 { a.triple_hop_esq } else if i == 2 { a.crossover_esq } else { a.six_meter_esq }
        ));

        let evolucao = match lsi_ant {
            Some(ant) => {
                let diff = lsi_at - ant;
                let cor = if diff >= 0.0 { "#2F855A" } else { "#C53030" };
                let seta = if diff >= 0.0 { "▲" } else { "▼" };
                format!(r#"<span style="color:{cor}; font-weight:bold;">{seta} {diff:+.1}%</span>"#)
            },
            None => r#"<span style="color:#A0AEC0;">--</span>"#.to_string(),
        };

        let status_badge = if lsi_at >= 90.0 {
            r#"<span style="background:#C6F6D5; color:#22543D; padding:3px 8px; border-radius:12px; font-size:10px; font-weight:bold;">APTO</span>"#
        } else {
            r#"<span style="background:#FED7D7; color:#822727; padding:3px 8px; border-radius:12px; font-size:10px; font-weight:bold;">N/APTO</span>"#
        };

        linhas.push_str(&format!(
            r#"<tr>
                <td style="font-weight:600;">{}</td>
                <td class="text-center">{:.1}</td>
                <td class="text-center">{:.1}</td>
                <td class="text-center"><strong>{:.1}%</strong></td>
                <td class="text-center" style="font-size:11px;">{}</td>
                <td class="text-center">{}</td>
            </tr>"#, nomes[i], sadio, operado, lsi_at, evolucao, status_badge
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8"/>
    <style>
        @page {{ size: A4; margin: 1.2cm; }}
        body {{ font-family: sans-serif; color: #2D3748; line-height: 1.4; margin: 0; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 2px solid #2C5282; padding-bottom: 10px; margin-bottom: 20px; }}
        .header h1 {{ font-size: 22px; margin: 0; color: #2C5282; text-transform: uppercase; letter-spacing: 1px; }}
        .patient-info {{ display: grid; grid-template-columns: repeat(3, 1fr); gap: 15px; margin-bottom: 20px; background: #F7FAF9; padding: 15px; border-radius: 8px; border: 1px solid #E2E8F0; }}
        .info-box {{ display: flex; flex-direction: column; }}
        .info-label {{ font-size: 9px; font-weight: bold; color: #718096; text-transform: uppercase; }}
        .info-value {{ font-size: 13px; font-weight: 600; color: #2D3748; }}
        .op-highlight {{ color: #C53030; font-weight: bold; }}
        .chart-section {{ text-align: center; border: 1px solid #EDF2F7; padding: 10px; border-radius: 8px; margin-bottom: 20px; }}
        table {{ width: 100%; border-collapse: collapse; margin-bottom: 20px; }}
        th {{ background: #2D3748; color: #FFF; padding: 10px; font-size: 10px; text-transform: uppercase; text-align: left; }}
        td {{ padding: 10px; border-bottom: 1px solid #EDF2F7; font-size: 12px; }}
        .text-center {{ text-align: center; }}
        .conclusion-area {{ display: grid; grid-template-columns: 1fr 2fr; gap: 20px; align-items: center; padding: 20px; border: 2px solid {cor_status}; border-radius: 12px; background: #FFF; }}
        .lsi-circle {{ text-align: center; border-right: 2px solid #EDF2F7; }}
        .lsi-circle big {{ font-size: 40px; font-weight: bold; color: {cor_status}; display: block; }}
        .status-text h2 {{ margin: 0; color: {cor_status}; font-size: 18px; }}
        @media print {{ body {{ -webkit-print-color-adjust: exact; }} }}
    </style>
</head>
<body>
    <div class="header">
        <div>
            <h1>Laudo de Avaliação Funcional</h1>
            <span style="font-size: 11px; color: #718096;">HupsTeste — Fisioterapia Especializada</span>
        </div>
        <div style="text-align:right; font-size: 11px;">
            <span>Data: {data_av}</span><br>
            <strong>ID: #00{dias}</strong>
        </div>
    </div>

    <div class="patient-info">
        <div class="info-box"><span class="info-label">Paciente</span><span class="info-value">{nome}</span></div>
        <div class="info-box"><span class="info-label">Membro Operado</span><span class="info-value op-highlight">{membro_op}</span></div>
        <div class="info-box"><span class="info-label">Pós-Operatório</span><span class="info-value">{dias} Dias</span></div>
        <div class="info-box"><span class="info-label">Data da Cirurgia</span><span class="info-value">{data_cir}</span></div>
        <div class="info-box"><span class="info-label">Peso / Altura</span><span class="info-value">{peso}kg / {altura}m</span></div>
        <div class="info-box"><span class="info-label">Clínica</span><span class="info-value">Fisio Joelho</span></div>
    </div>

    <div class="chart-section">{svg_grafico}</div>

    <table>
        <thead>
            <tr>
                <th>Teste Efetuado</th>
                <th class="text-center">Sadio (cm/s)</th>
                <th class="text-center">Operado (cm/s)</th>
                <th class="text-center">LSI Atual</th>
                <th class="text-center">Evolução</th>
                <th class="text-center">Resultado</th>
            </tr>
        </thead>
        <tbody>{linhas}</tbody>
    </table>

    <div class="conclusion-area">
        <div class="lsi-circle">
            <small style="font-size: 10px; font-weight: bold; color: #718096; text-transform: uppercase;">LSI MÉDIO</small>
            <big>{media:.1}%</big>
        </div>
        <div class="status-text">
            <h2>{status_texto}</h2>
            <p style="margin: 5px 0 0; font-size: 12px; color: #4A5568;"><strong>Critério Clínico:</strong> Índice de Simetria Limítrofe ≥ 90% para redução de risco de relesão.</p>
        </div>
    </div>

    <script>window.onload = () => window.print();</script>
</body>
</html>"#,
        nome = paciente.nome,
        membro_op = membro_op,
        data_av = ultima.data_avaliacao,
        data_cir = paciente.data_cirugia,
        dias = paciente.dias_pos_operatorio.unwrap_or(0),
        peso = paciente.peso,
        altura = paciente.altura,
        svg_grafico = svg_grafico,
        cor_status = cor_status,
        status_texto = texto_status,
        media = media,
        linhas = linhas
    )
}

fn html_sem_avaliacoes(p: &Paciente) -> String {
    format!("<html><body style='font-family:sans-serif; padding:50px;'><h2>Nenhuma avaliação encontrada para {}</h2></body></html>", p.nome)
}
