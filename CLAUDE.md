# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Visão Geral do Projeto

**Hups Teste - Avaliação Funcional** é um frontend Yew (Rust/WebAssembly) para um sistema de fisioterapia que acompanha avaliações funcionais de retorno ao esporte. A métrica central é o **LSI (Índice de Simetria de Membros)**: `(membro mais fraco / membro mais forte) × 100%` — o paciente é considerado apto (`APTO`) quando LSI ≥ 90%.

## Comandos

```bash
# Instalar o Trunk (ferramenta de build), se não estiver instalado
cargo install trunk

# Servidor de desenvolvimento (porta 3001, proxy /api → localhost:8080)
trunk serve

# Build de produção (WASM + assets em dist/)
trunk build --release

# Verificar tipos sem compilar o WASM
cargo check

# Linter
cargo clippy --target wasm32-unknown-unknown

# Formatar código
cargo fmt
```

> A aplicação requer o backend rodando em `http://localhost:8080` para que as chamadas de API funcionem em desenvolvimento.

## Arquitetura

### Pilha Tecnológica

| Camada | Ferramenta |
|---|---|
| Framework | Yew 0.21 (Rust → WASM) |
| Build | Trunk |
| Roteamento | yew-router 0.18 |
| HTTP | gloo-net 0.6 |
| Estado de autenticação | gloo-storage 0.3 (LocalStorage) |
| Gráficos | Chart.js 4.4 (JS inline via wasm-bindgen) + Plotters (SVG para PDF) |
| PDF | html2pdf.js 0.10 (gerado no cliente) |
| Interface | Bootstrap 5.3 + Bootstrap Icons |

### Mapa de Módulos

```
src/
├── main.rs          # Ponto de entrada Yew + inicialização do wasm-logger
├── router.rs        # Enum de rotas e mapeamento do Switch
├── models.rs        # Todas as structs de dados + helpers de LSI
├── api.rs           # Todas as chamadas HTTP (autenticação Bearer)
├── auth.rs          # Token JWT no LocalStorage (chave: "auth_token")
├── pdf.rs           # Geração do relatório HTML+SVG via Plotters
└── components/
    ├── mod.rs
    ├── app.rs           # Componente raiz, navbar, ProtectedRoute
    ├── login.rs         # Formulário de autenticação → salva token → redireciona
    ├── paciente_list.rs # Tabela de pacientes + formulário de cadastro + geração de PDF
    ├── avaliacao_form.rs# Formulário bilateral dos 4 testes + exibição de LSI em tempo real
    ├── numeric_input.rs # Input reutilizável: apenas dígitos, formata como "X,XX"
    └── lsi_chart.rs     # Gráfico Chart.js com evolução do LSI entre avaliações
```

### Rotas

| Caminho | Componente | Autenticação |
|---|---|---|
| `/login` | `LoginPage` | Não |
| `/pacientes` | `PacienteListPage` | Sim |
| `/avaliacoes/novo/:id` | `AvaliacaoFormPage` | Sim |
| `/` | Redireciona para `/login` | — |

Acesso não autenticado às rotas protegidas redireciona para `/login` via o wrapper `ProtectedRoute` em `app.rs`.

### Fluxo de Dados

1. Login → JWT armazenado no LocalStorage via `auth.rs`
2. Toda chamada em `api.rs` lê o token e adiciona `Authorization: Bearer <token>`
3. URL base da API é `/api/v1` (proxy do Trunk em dev, servido pelo backend em prod)
4. Componentes são stateful via hooks `use_state`/`use_effect` do Yew
5. O relatório PDF é construído como string HTML em `pdf.rs` (com gráficos SVG embutidos do Plotters), depois repassado ao `html2pdf.js` via interop JavaScript

### Lógica de Domínio Principal (`models.rs`)

- `calcular_lsi(a, b)` → `(a.min(b) / a.max(b)) * 100.0`
- `media_lsi(avaliacao)` → média dos valores de LSI dos 4 testes bilaterais
- `parse_decimal(s)` → substitui `,` por `.` antes de converter para float (locale brasileiro)
- `definir_cores_por_membro(membro_operado)` → atribui classes de cor do Bootstrap; membro operado = vermelho, membro sadio = azul

### Os 4 Testes Bilaterais (struct `Avaliacao`)

Cada teste possui duas medições (membro sadio / membro operado):

1. Single Hop for Distance
2. Triple Hop for Distance
3. Crossover Hop for Distance
4. 6-meter Timed Hop

### Interoperabilidade com JavaScript

`lsi_chart.rs` chama o Chart.js diretamente usando `wasm_bindgen::JsValue` e `web_sys`. Ao modificar o comportamento dos gráficos, as alterações devem estar alinhadas com a API do Chart.js 4.x carregada no `index.html`.

`pdf.rs` aciona o `html2pdf()` a partir de `paciente_list.rs` via função JavaScript chamada pelo `wasm_bindgen`. O ID do elemento DOM gerado por `pdf.rs` deve corresponder ao que o código espera.

### Convenção de Entrada Decimal

O componente `NumericInput` impõe o formato decimal brasileiro (`X,XX`). Internamente, `parse_decimal()` em `models.rs` converte vírgulas em pontos antes das operações aritméticas. Todos os novos campos numéricos devem seguir o mesmo padrão.

## Regras
- Toda a documentação em Português-BR.