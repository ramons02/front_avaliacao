use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NumericInputProps {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub class: String,
}

fn formatar_decimal(raw: &str) -> String {
    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { return String::new(); }
    // Remove zeros à esquerda excedentes, mantém pelo menos 1 dígito antes da vírgula
    let num: u64 = digits.parse().unwrap_or(0);
    let inteiro = num / 100;
    let decimal = num % 100;
    if inteiro == 0 {
        format!("0,{:02}", decimal)
    } else {
        format!("{},{:02}", inteiro, decimal)
    }
}

#[function_component(NumericInput)]
pub fn numeric_input(props: &NumericInputProps) -> Html {
    let on_input = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let formatado = formatar_decimal(&input.value());
            // Atualiza o campo visualmente
            input.set_value(&formatado);
            on_change.emit(formatado);
        })
    };

    let on_keydown = Callback::from(|e: KeyboardEvent| {
        let key = e.key();
        let ctrl = e.ctrl_key() || e.meta_key();
        let permitido = matches!(
            key.as_str(),
            "Backspace" | "Delete" | "Tab" | "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" | "Home" | "End"
        ) || ctrl
            || key.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);

        if !permitido {
            e.prevent_default();
        }
    });

    html! {
        <input
            type="text"
            inputmode="decimal"
            class={format!("form-control {}", props.class)}
            value={props.value.clone()}
            placeholder={props.placeholder.clone()}
            oninput={on_input}
            onkeydown={on_keydown}
        />
    }
}