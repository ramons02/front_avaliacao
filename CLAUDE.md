# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Visão Geral do Projeto

**Hups Teste - Avaliação Funcional** é um frontend Yew (Rust/WebAssembly) para um sistema de fisioterapia que acompanha avaliações funcionais de retorno ao esporte. A métrica central é o **LSI (Índice de Simetria de Membros)**: `(membro mais fraco / membro mais forte) × 100%` — o paciente é considerado apto (`APTO`) quando LSI ≥ 90%.

## Comandos

```bash
# Servidor de desenvolvimento (porta 3001, proxy /api/v1 → localhost:8080)
trunk serve

# Build de produção (WASM + assets em dist/)
trunk build --release

# Verificar tipos sem compilar o WASM
cargo check

# Linter (obrigatório usar o target WASM)
cargo clippy --target wasm32-unknown-unknown

# Formatar código
cargo fmt

# Instalar o Trunk (v0.21.14 — mesma versão usada no CI)
cargo install trunk
```

> A aplicação requer o backend em `http://localhost:8080`. Em produção, o Vercel proxeia `/api/v1/*` para o Railway.

## Arquitetura

### Pilha Tecnológica

| Camada | Ferramenta |
|---|---|
| Framework | Yew 0.21 (Rust → WASM, CSR) |
| Build | Trunk |
| Roteamento | yew-router 0.18 |
| HTTP | gloo-net 0.6 |
| Estado de autenticação | gloo-storage 0.3 (LocalStorage) |
| Gráficos | Chart.js 4.4 (JS inline via `wasm_bindgen(inline_js)`) + Plotters 0.3 (SVG para PDF) |
| PDF | html2pdf.js 0.10 invocado via `js_sys::Function::new_with_args` |
| Interface | Bootstrap 5.3 + Bootstrap Icons |

### Fluxo de Dados

1. Login → JWT armazenado no LocalStorage via `auth.rs` (chave: `"auth_token"`)
2. Toda chamada em `api.rs` lê o token e injeta `Authorization: Bearer <token>`
3. URL base da API é `/api/v1` (proxy do Trunk em dev; reescrita do Vercel em prod)
4. Componentes são stateful via hooks `use_state`/`use_effect` do Yew

### Convenções Críticas

**Tratamento de erros da API:** Respostas 401/403 retornam `Err("UNAUTHORIZED")`. Os componentes detectam essa string e chamam `redirecionar_login()` (limpa o token e redireciona via `window.location.set_href`).

**Deserialização numérica:** O backend Java pode enviar campos numéricos como JSON number ou como string formatada. O deserializador customizado `de_numero_ou_string` em `models.rs` lida com ambos os casos para `peso` e `altura` do `Paciente`.

**Convenção de entrada decimal:** O componente `NumericInput` impõe o formato `X,XX`. A função `parse_decimal()` em `models.rs` converte vírgula em ponto antes de operações aritméticas. A função `mascara_decimal()` em `paciente_list.rs` é usada para campos não-`NumericInput` (peso, altura no formulário de cadastro).

**Cálculo de LSI:** Existem duas variantes com semânticas diferentes:
- `calcular_lsi(a, b)` em `models.rs` → `(min/max) × 100` — simétrica, usada nos formulários e gráficos
- Cálculo no relatório (`pdf.rs`) → `(operado/sadio) × 100` — direcional, usa o membro operado para identificar qual lado é o operado

**Gráfico LSI:** `lsi_chart.rs` embute o código JS via `#[wasm_bindgen(inline_js)]`. O Chart.js precisa estar carregado no `index.html` *antes* da inicialização do WASM, caso contrário a função `renderLsiChart` não existe quando é chamada.

### Deployment

- **Vercel (`vercel.json`):** `buildCommand` aponta para `vercel-build.sh`, que instala Rust + target wasm32 + Trunk v0.21.14 no CI.
- **Proxy de produção:** `/api/v1/*` é reescrito para `https://hupstesteback-production.up.railway.app/api/v1/$1`.
- **SPA routing:** `rewrite /.*` → `/index.html` garante que rotas do yew-router funcionem no reload.
- **Assets estáticos:** O diretório `public/` é copiado para `dist/` pelo Trunk (ex: `_redirects`).

### Mapeamento de Campos JSON ↔ Rust (`Avaliacao`)

Os campos JSON usam `camelCase` com sufixos `Direita`/`Esquerda` enquanto Rust usa `_dir`/`_esq`:

| JSON | Rust |
|---|---|
| `singleHopDireita` | `single_hop_dir` |
| `singleHopEsquerda` | `single_hop_esq` |
| `tripleHopDireita` | `triple_hop_dir` |
| `tripleHopEsquerda` | `triple_hop_esq` |
| `crossoverHopDireita` | `crossover_dir` |
| `crossoverHopEsquerda` | `crossover_esq` |
| `sixMeterDireita` | `six_meter_dir` |
| `sixMeterEsquerda` | `six_meter_esq` |

`Paciente` também usa `dataCirugia` e `diasPosOperatorio` como nomes JSON.

## Regras
- Toda a documentação em Português-BR.
