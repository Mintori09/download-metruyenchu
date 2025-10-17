use clap::{Parser, Subcommand};
use download_mtc::chapter::get_chapters;
use download_mtc::download_me_truyen_chu::download_me_truyen_chu;
use download_mtc::epub::{EpubMetadata, epub_build};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Parser)]
#[command(name = "metruyencv")]
#[command(about = "Công cụ tải và xử lý truyện từ metruyencv.com", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        json_path: String,
    },

    EpubBuild {},

    GetChapter {
        url: String,
        #[arg(short, long, default_value = "chapters.json")]
        output: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Download { json_path } => {
            handle_json_download(json_path)?;
        }

        Commands::EpubBuild {} => {
            let metadata = EpubMetadata::read_from_console();
            if let Err(e) = epub_build(&metadata) {
                eprintln!("Lỗi: {}", e);
            }
        }

        Commands::GetChapter { url, output } => {
            println!("Đang lấy danh sách chương từ: {}", url);
            if let Ok(chapters) = get_chapters(url) {
                let file = File::create(output)?;
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &chapters)?;
                println!("Đã lưu vào {}", output);
            } else {
                eprintln!("Không thể lấy chương!");
            }
        }
    }

    Ok(())
}

fn handle_json_download(json_path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(json_path);
    if !path.exists() {
        return Err(format!("Không tìm thấy file: {}", json_path).into());
    }

    println!("Đang tải truyện từ: {}", json_path);
    if let Err(error) = download_me_truyen_chu(json_path) {
        eprintln!("Lỗi: {}", error);
    }

    Ok(())
}
