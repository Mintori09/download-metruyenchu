use crate::{cookies::create_cookies, model::Chapter};
use headless_chrome::Browser;
use regex::Regex;
use std::{
    fs::{self},
    thread,
    time::Duration,
};

pub fn download_me_truyen_chu(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let mut chapters: Vec<Chapter> = serde_json::from_str(&data)?;

    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    for i in 0..chapters.len() {
        let total = chapters.len();
        let chap = &mut chapters[i];

        if chap.is_download {
            println!("> Skip {}/{}: {}", i + 1, total, chap.name);
            continue;
        }

        println!("> Crawl {}/{}: {}", i + 1, total, chap.name);

        tab.set_cookies(create_cookies(&chap.link))?;

        if let Err(e) = tab.navigate_to(&chap.link) {
            eprintln!("> Lỗi khi vào {}: {:?}", chap.link, e);
            continue;
        }

        tab.wait_until_navigated()?;

        if tab.wait_for_element("#chapter-content").is_err() {
            eprintln!("⚠️  Không tìm thấy nội dung cho {}", chap.name);
            continue;
        }

        let js = r#"
            let el = document.querySelector('#chapter-content');
            el ? el.innerText : 'Không tìm thấy nội dung chương.';
        "#;
        let result = tab.evaluate(js, false)?;

        if let Some(val) = result.value {
            let raw_text = val.to_string();
            let text = format_content(raw_text);
            let filename = format!("{}.md", sanitize_filename(&chap.name));

            fs::write(&filename, &text)?;
            println!("Lưu {}", filename);

            chap.is_download = true;

            fs::write(file_path, serde_json::to_string_pretty(&chapters)?)?;
        }

        thread::sleep(Duration::from_millis(800));
    }

    println!("Hoàn tất tải {} chương!", chapters.len());
    Ok(())
}

// fn read_file(file_path: &str) -> Vec<Chapter> {
//     let data = fs::read_to_string(file_path).expect("Không đọc được file!");
//     let chapters: Vec<Chapter> = serde_json::from_str(&data).expect("Không parse được JSON!");
//     chapters
// }
//
// fn save_file(file_path: &str, content: Vec<Chapter>) -> Result<(), Box<dyn Error>> {
//     let file = File::create(file_path)?;
//     let writer = BufWriter::new(file);
//     serde_json::to_writer_pretty(writer, &content)?;
//     Ok(())
// }

fn format_content(content: String) -> String {
    let re = Regex::new(r"\n+").unwrap();
    let mut text = content.replace("\\n", "\n").replace("\\", "");
    text = re.replace_all(&text, "\n").to_string();
    if !text.is_empty() {
        text.remove(0);
        text.pop();
    }
    text
}

use std::path::Path;

fn sanitize_filename(name: &str) -> String {
    // Tạo thư mục download nếu chưa có
    let download_dir = Path::new("download");
    if !download_dir.exists() && fs::create_dir_all(download_dir).is_err() {
        eprintln!("Không thể tạo thư mục download/: lỗi khi tạo.");
    }

    // Lọc ký tự đặc biệt
    let file: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect();

    format!("download/{}", file)
}
