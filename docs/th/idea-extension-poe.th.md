# ส่วนขยาย Poe Bot ภาษา Rust สำหรับ Gemini CLI (Poe Bot Rust Extension for Gemini CLI)

```rust
// การพึ่งพาใน Cargo.toml (Cargo.toml dependencies):
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// async-trait = "0.1"
// thiserror = "1.0"

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Error, Debug)]
pub enum PoeExtensionError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Script execution failed: {0}")]
    ExecutionError(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoeConfig {
    pub bot_name: String,
    pub api_key: Option<String>,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoeToolInput {
    pub action: String,
    pub bot_name: Option<String>,
    pub script_path: Option<String>,
    pub config: Option<PoeConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoeToolOutput {
    pub success: bool,
    pub message: String,
    pub script_path: Option<String>,
    pub output: Option<String>,
}

/// ผู้จัดการส่วนขยาย Poe Bot (Poe Bot Extension Manager)
pub struct PoeExtension {
    extension_path: PathBuf,
    scripts_dir: PathBuf,
}

impl PoeExtension {
    pub fn new(extension_path: PathBuf) -> Self {
        let scripts_dir = extension_path.join("poe_scripts");
        Self {
            extension_path,
            scripts_dir,
        }
    }

    /// สร้าง Poe bot script ใหม่
    pub async fn create_bot_script(
        &self,
        config: &PoeConfig,
    ) -> Result<PoeToolOutput, PoeExtensionError> {
        // สร้างโฟลเดอร์ scripts ถ้ายังไม่มี
        tokio::fs::create_dir_all(&self.scripts_dir).await?;

        let script_path = self.scripts_dir.join(format!("{}.py", config.bot_name));

        // สร้างเนื้อหา Poe bot script
        let script_content = self.generate_poe_script(config);

        // เขียนไฟล์ script
        let mut file = File::create(&script_path).await?;
        file.write_all(script_content.as_bytes()).await?;
        file.sync_all().await?;

        Ok(PoeToolOutput {
            success: true,
            message: format!("สร้าง Poe bot script สำเร็จ: {}", config.bot_name),
            script_path: Some(script_path.to_string_lossy().to_string()),
            output: None,
        })
    }

    /// เรียกใช้ Poe bot script
    pub async fn run_bot_script(
        &self,
        bot_name: &str,
    ) -> Result<PoeToolOutput, PoeExtensionError> {
        let script_path = self.scripts_dir.join(format!("{}.py", bot_name));

        if !script_path.exists() {
            return Err(PoeExtensionError::ExecutionError(format!(
                "ไม่พบสคริปต์: {}",
                bot_name
            )));
        }

        // เรียกใช้ Python script
        let output = Command::new("python3")
            .arg(&script_path)
            .current_dir(&self.scripts_dir)
            .output()
            .map_err(|e| PoeExtensionError::ExecutionError(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(PoeExtensionError::ExecutionError(format!(
                "การรันสคริปต์ล้มเหลว: {}",
                stderr
            )));
        }

        Ok(PoeToolOutput {
            success: true,
            message: format!("รัน Poe bot script สำเร็จ: {}", bot_name),
            script_path: Some(script_path.to_string_lossy().to_string()),
            output: Some(stdout),
        })
    }

    /// ลบ Poe bot script
    pub async fn delete_bot_script(
        &self,
        bot_name: &str,
    ) -> Result<PoeToolOutput, PoeExtensionError> {
        let script_path = self.scripts_dir.join(format!("{}.py", bot_name));

        if !script_path.exists() {
            return Err(PoeExtensionError::ExecutionError(format!(
                "ไม่พบสคริปต์: {}",
                bot_name
            )));
        }

        tokio::fs::remove_file(&script_path).await?;

        Ok(PoeToolOutput {
            success: true,
            message: format!("ลบ Poe bot script แล้ว: {}", bot_name),
            script_path: None,
            output: None,
        })
    }

    /// แสดงรายการ Poe bot scripts
    pub async fn list_bot_scripts(&self) -> Result<PoeToolOutput, PoeExtensionError> {
        if !self.scripts_dir.exists() {
            return Ok(PoeToolOutput {
                success: true,
                message: "ไม่พบสคริปต์".to_string(),
                script_path: None,
                output: Some("[]".to_string()),
            });
        }

        let mut scripts = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.scripts_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("py") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    scripts.push(name.to_string());
                }
            }
        }

        let output = serde_json::to_string(&scripts)?;

        Ok(PoeToolOutput {
            success: true,
            message: format!("พบ {} Poe bot scripts", scripts.len()),
            script_path: None,
            output: Some(output),
        })
    }

    /// สร้างเนื้อหา Poe bot script template
    fn generate_poe_script(&self, config: &PoeConfig) -> String {
        format!(
            r#"""
from poe_api_wrapper import PoeApi

class {}Bot:
    """
    Poe Bot: {}
    Version: {}
    Description: {}
    """
    
    def __init__(self):
        self.name = "{}"
        self.version = "{}"
        self.description = "{}"
        {}
    
    def initialize(self):
        """Initialize the bot"""
        print(f"Initializing {{self.name}} bot...")
        return True
    
    def handle_message(self, message: str) -> str:
        """
        Handle incoming message
        
        Args:
            message: User message
            
        Returns:
            Bot response
        """
        print(f"Received message: {{message}}")
        # Add your bot logic here
        return f"Response from {{self.name}}: {{message}}"
    
    def run(self):
        """Run the bot"""
        if self.initialize():
            print(f"{{self.name}} bot is running...")
            # Add your bot execution logic here
            return True
        return False

if __name__ == "__main__":
    bot = {}Bot()
    bot.run()
"""#,
            config.bot_name,
            config.bot_name,
            config.version,
            config.description,
            config.bot_name,
            config.version,
            config.description,
            if config.api_key.is_some() {
                'self.api_key = "***"'.to_string()
            } else {
                "self.api_key = None".to_string()
            },
            config.bot_name,
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extension = PoeExtension::new(PathBuf::from("."));

    // ตัวอย่าง: สร้าง Poe bot script
    let config = PoeConfig {
        bot_name: "my_poe_bot".to_string(),
        api_key: Some("BL1Z_API_KEY".to_string()),
        version: "1.0.0".to_string(),
        description: "A custom Poe bot created by Rust extension".to_string(),
    };

    match extension.create_bot_script(&config).await {
        Ok(output) => {
            println!("✓ {}", output.message);
            if let Some(path) = output.script_path {
                println!("  Script path: {}", path);
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // ตัวอย่าง: เรียกใช้ script
    match extension.run_bot_script("my_poe_bot").await {
        Ok(output) => {
            println!("✓ {}", output.message);
            if let Some(out) = output.output {
                println!("  Output: {}", out);
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    // ตัวอย่าง: แสดงรายการ scripts
    match extension.list_bot_scripts().await {
        Ok(output) => {
            println!("✓ {}", output.message);
            if let Some(scripts) = output.output {
                println!("  Scripts: {}", scripts);
            }
        }
        Err(e) => eprintln!("✗ Error: {}", e),
    }

    Ok(())
}
```

## คำอธิบาย

ฉันได้สร้าง **Rust Extension** สำหรับ Gemini CLI ที่มีฟีเจอร์หลักดังนี้:

### 🎯 ฟีเจอร์หลัก

1. **`create_bot_script()`** - สร้าง Poe bot script ใหม่
   - รับการกำหนดค่า (configuration) ของบอท
   - สร้างไฟล์ Python script โดยอัตโนมัติ
   - ใช้งาน poepython API wrapper

2. **`run_bot_script()`** - เรียกใช้ Poe bot script
   - ค้นหาสคริปต์ตามชื่อ
   - เรียกใช้ผ่าน Python3
   - ดักจับผลลัพธ์ (output) และข้อผิดพลาด (error)

3. **`delete_bot_script()`** - ลบ Poe bot script
   - ลบไฟล์สคริปต์ที่ไม่ต้องการ

4. **`list_bot_scripts()`** - แสดงรายการสคริปต์ทั้งหมด
   - ค้นหาไฟล์ `.py` ทั้งหมด
   - คืนค่าในรูปแบบ JSON

### 📦 โครงสร้าง (Structure)

- **การจัดการข้อผิดพลาด (Error Handling)**: ใช้งาน `thiserror` สำหรับการจัดการข้อผิดพลาด
- **Async/Await**: ใช้งาน `tokio` สำหรับการประมวลผลแบบอะซิงโครนัส (async operations)
- **ความปลอดภัยของชนิดข้อมูล (Type Safety)**: ใช้งานระบบชนิดข้อมูลของ Rust เพื่อความปลอดภัย
- **การแปลงข้อมูล (Serialization)**: ใช้งาน `serde` สำหรับการจัดการ JSON

### 🚀 การใช้งาน (Usage)

```bash
cargo build --release
./target/release/poe_extension
```

ส่วนขยายนี้สามารถรวมเข้ากับ Gemini CLI ได้โดยการเพิ่มข้อมูลใน `gemini-extension.json` และสร้าง MCP server wrapper!
