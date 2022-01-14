use tauri::{command};

#[command]
pub fn message_in_terminal() {
  println!("Mensagem no terminal");
}

// o command pode ser passado dessa forma tbm
#[tauri::command]
pub fn send_message_to_terminal(msg: String) {
  println!("Mensagem vindo do JavaScript: {}", msg)
}