use tauri::Window;

#[tauri::command]
pub fn pcommand_help(args: &str, window: Window) {
    if args.len() == 0 {
        super::tell_player(
            "You are looking for help but this section has not been written yet.".to_string(),
            window,
        );
    } else {
        super::tell_player(format!("We can't find help on \"{}\".", args), window);
    }
}
