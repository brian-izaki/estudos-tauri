use tauri::{command};

#[command]
pub fn message_in_terminal() {
  println!("Mensagem no terminal");
}

// o command pode ser passado dessa forma tbm
#[tauri::command]
pub fn send_message_to_terminal(msg: String) {
  println!("Mensagem vindo do JavaScript: {}", msg);
}

// o parametro em snake_case aqui pode ser utilizado como camelCase no JS
#[command]
pub fn handle_error(is_error: bool) -> Result<String, String> {
  
  // o .into() faz conversão de tipo. o tipo a ser convertido irá depender do local.
  // no exemplo abaixo precisa de um String sendo q "texto" é um &str
  if is_error {
    Err("Ocorreu uma falha".into())
  } else {
    Ok("Sucesso".into())
  }
}