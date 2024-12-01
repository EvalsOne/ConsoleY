use std::path::PathBuf;
use tokio::fs;
use tauri::Manager;
use tokio::io::AsyncReadExt;

pub async fn extract_docker_image(app: &tauri::AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_dir = app.path().app_local_data_dir()
        .expect("Failed to get app data directory");
    
    let image_dir = app_dir.join("resources");
    let image_path = image_dir.join("desktop.tar");

    // 如果镜像文件已存在，验证其完整性
    if image_path.exists() {
        if let Ok(metadata) = fs::metadata(&image_path).await {
            if metadata.len() > 1000000 { // 确保文件大小合理
                return Ok(image_path);
            }
            // 如果文件太小，删除它
            fs::remove_file(&image_path).await?;
        }
    }

    // 创建资源目录
    fs::create_dir_all(&image_dir).await?;

    // 从应用资源目录中读取 desktop.tar
    let resource_dir = app.path().resource_dir()
        .expect("Failed to get resource directory");

    // 修改这里：检查 resources 子目录
    let bundled_image_path = resource_dir.join("resources").join("desktop.tar");
    println!("Looking for desktop.tar in: {:?}", bundled_image_path);

    if !bundled_image_path.exists() {
        return Err(format!("Bundled desktop.tar not found at {:?}", bundled_image_path).into());
    }

    // 读取并写入文件
    println!("Copying desktop.tar to application directory...");
    let mut bundled_image = fs::File::open(&bundled_image_path).await?;
    let mut buffer = Vec::new();
    bundled_image.read_to_end(&mut buffer).await?;
    fs::write(&image_path, &buffer).await?;

    println!("Successfully extracted desktop.tar to: {:?}", image_path);
    Ok(image_path)
} 