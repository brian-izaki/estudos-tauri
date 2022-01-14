#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

mod cmd;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let splashscreen_window = app.get_window("splashScreen").unwrap();
      let main_window = app.get_window("main").unwrap();

      tauri::async_runtime::spawn(async move {
        // initialize your app here instead of sleeping :)
        println!("Iniciando aplicação");
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("Aplicação inicializada! \\O/");

        // After it's done, close the splashscreen and display the main window
        splashscreen_window.close().unwrap();
        main_window.show().unwrap();
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      cmd::message_in_terminal,
      cmd::send_message_to_terminal,
      cmd::handle_error
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
