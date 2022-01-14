#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  println!("Iniciando aplicação");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      cmd::message_in_terminal,
      cmd::send_message_to_terminal
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  println!("Aplicação inicializada! \\O/");
}
