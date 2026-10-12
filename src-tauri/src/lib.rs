use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn convert(_app: tauri::AppHandle, url: String, format: String) -> Result<String, String> {
    let yt_dlp = find_ytdlp().await?;
    let output_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).to_string_lossy().to_string();
    if format == "mp3" {
        let output = format!("{}/%(title)s.%(ext)s", output_dir);
        let args = vec!["-x", "--audio-format", "mp3", "--audio-quality", "0", "-o", &output, &url];
        run_command(&yt_dlp, &args).await?;
        let title = get_title(&yt_dlp, &url).await.unwrap_or_else(|_| "download".to_string());
        let filename = format!("{}.mp3", sanitize(&title));
        let full = PathBuf::from(&output_dir).join(&filename);
        return Ok(full.to_string_lossy().to_string());
    } else if format == "mp4" {
        let output = format!("{}/%(title)s.%(ext)s", output_dir);
        let args = vec!["-f", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best", "-o", &output, &url];
        run_command(&yt_dlp, &args).await?;
        let title = get_title(&yt_dlp, &url).await.unwrap_or_else(|_| "download".to_string());
        let filename = format!("{}.mp4", sanitize(&title));
        let full = PathBuf::from(&output_dir).join(&filename);
        return Ok(full.to_string_lossy().to_string());
    }
    Err("Unsupported format".to_string())
}

async fn find_ytdlp() -> Result<String, String> {
    if let Ok(p) = which::which("yt-dlp") { return Ok(p.to_string_lossy().to_string()); }
    if let Ok(p) = which::which("yt-dlp.exe") { return Ok(p.to_string_lossy().to_string()); }
    let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let candidates = vec!["yt-dlp", "yt-dlp.exe", "bin/yt-dlp", "bin/yt-dlp.exe"];
    for c in &candidates { let full = current.join(c); if full.exists() { return Ok(full.to_string_lossy().to_string()); } }
    Err("yt-dlp not found".to_string())
}

async fn run_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    let mut child = Command::new(cmd).args(args).spawn().map_err(|e| format!("Failed: {}", e))?;
    let status = child.wait().map_err(|e| format!("Failed: {}", e))?;
    if status.success() { Ok(()) } else { Err(format!("Failed status: {}", status)) }
}

async fn get_title(cmd: &str, url: &str) -> Result<String, String> {
    let output = Command::new(cmd).args(&["--get-title", url]).output().map_err(|e| format!("Failed: {}", e))?;
    if output.status.success() { let s = String::from_utf8_lossy(&output.stdout); Ok(s.trim().to_string()) } else { Err("Failed".to_string()) }
}

fn sanitize(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c { '\\'|'/'|':'|'*'|'?'|'"'|'<'|'>'|'|' => out.push('_'), _ => out.push(c) }
    }
    out
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
