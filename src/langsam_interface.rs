use std::process::{Command, Stdio};
use std::io::Write;
use serde_json::json;
use image::{ImageBuffer, Rgba};
use std::str;

pub fn run_langsam_python(image_path: &str, prompt: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
    let input = json!({
        "image_path": image_path,
        "prompt": prompt
    });
    print!("Running LangSAM Python script...");
    let mut child = Command::new("langSAM/env/bin/python")
        .arg("langSAM/api.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn Python process: {}", e))?;
    
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(input.to_string().as_bytes()).map_err(|e| format!("Failed to write to stdin: {}", e))?;
    
    let output = child.wait_with_output().map_err(|e| format!("Failed to read output: {}", e))?;
    
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unable to read stderr");
        return Err(format!("Python script failed: {}", stderr));
    }
    
    // The Python script doesn't return anything, so we don't need to parse stdout
    
    // Wait for a short time to ensure the file is written
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    // Read the image from the known output path
    let output_image_path = "tmp/seg_out.png";
    let image = image::open(output_image_path)
        .map_err(|e| format!("Failed to open output image: {}", e))?;
    
    Ok(image.to_rgba8())
}