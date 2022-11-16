use ansi_term::Colour::Green;
use tauri::Window;

const WHOIS_INTRO: &str = "Welcome to the WHOIS system, which allows you to determine who is who based on their names or short descriptions. If you would like to know the name of a player, just type #WHOIS <short description>.";

fn tell_player_whois(input: &str, window: &Window) {
    let tagline = String::from(format!("{}", Green.paint("[WHOIS] ")));
    let tagline = tagline.as_bytes();
    let message = [b"\r\n", tagline, input.as_bytes(), b"\r\n"].concat();
    window.emit("telnet-message", message).unwrap();
}

fn show_whois_intro(window: &Window) {
    tell_player_whois(WHOIS_INTRO, &window);
}

fn whois(args: &str, window: &Window) {}

#[tauri::command]
pub fn pcommand_whois(args: &str, window: Window) {
    if args.len() == 0 {
        show_whois_intro(&window);
    } else {
        whois(args, &window);
    }
}
