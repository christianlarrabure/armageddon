use regex::{Captures, Regex};
use std::borrow::Cow;
use std::str;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
pub struct PromptPayload {
    hp: String,
    max_hp: String,
    mana: String,
    max_mana: String,
    stamina: String,
    max_stamina: String,
    stun: String,
    max_stun: String,
    focus: String,
    max_focus: String,
}

fn emit_prompt(matches: Captures, window: &Window) {
    let hp = matches.name("hp").unwrap().as_str().to_string();
    let max_hp = matches.name("max_hp").unwrap().as_str().to_string();
    let mana = matches.name("mana").unwrap().as_str().to_string();
    let max_mana = matches.name("max_mana").unwrap().as_str().to_string();
    let stamina = matches.name("stamina").unwrap().as_str().to_string();
    let max_stamina = matches.name("max_stamina").unwrap().as_str().to_string();
    let stun = matches.name("stun").unwrap().as_str().to_string();
    let max_stun = matches.name("max_stun").unwrap().as_str().to_string();
    let focus = matches.name("focus").unwrap().as_str().to_string();
    let max_focus = matches.name("max_focus").unwrap().as_str().to_string();

    window
        .emit(
            "armageddon-prompt",
            PromptPayload {
                hp: hp,
                max_hp: max_hp,
                mana: mana,
                max_mana: max_mana,
                stamina: stamina,
                max_stamina: max_stamina,
                stun: stun,
                max_stun: max_stun,
                focus: focus,
                max_focus: max_focus,
            },
        )
        .unwrap()
}

pub fn is_prompt(input: Cow<'_, str>, window: &Window) -> String {
    let prompt_regex =
        Regex::new(r"(?P<hp>[0-9]{1,3})/(?P<max_hp>[0-9]{1,3}) (?P<mana>[0-9]{1,3})/(?P<max_mana>[0-9]{1,3}) (?P<stamina>[0-9]{1,3})/(?P<max_stamina>[0-9]{1,3}) (?P<stun>[0-9]{1,3})/(?P<max_stun>[0-9]{1,3}) (?P<focus>[0-9]{1,3})/(?P<max_focus>[0-9]{1,3})").unwrap();
    let input = input.to_string();
    let _is_prompt = prompt_regex.captures(input.as_str());
    if _is_prompt.is_none() {
        return input;
    }

    let matches = _is_prompt.unwrap();
    emit_prompt(matches, window);

    prompt_regex.replace_all(input.as_str(), "").to_string()
}
