use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use natord::compare;
use std::{
    fs,
    io::{self, Cursor, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct EpubMetadata {
    pub title: String,
    pub author: String,
    pub image_link: String,
    pub folder_path: String,
}

impl Default for EpubMetadata {
    fn default() -> Self {
        Self {
            title: "".into(),
            author: "".into(),
            image_link: "cover.png".into(),
            folder_path: "./download".into(),
        }
    }
}
impl EpubMetadata {
    pub fn read_from_console() -> Self {
        let mut metadata = Self::default();

        metadata.title = Self::read_input("Enter title: ");
        metadata.author = Self::read_input("Enter author: ");
        metadata.image_link =
            Self::read_input_with_default("Enter image link", &metadata.image_link);
        metadata.folder_path =
            Self::read_input_with_default("Enter folder path", &metadata.folder_path);

        metadata
    }

    fn read_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    fn read_input_with_default(prompt: &str, default: &str) -> String {
        print!("{} [{}]: ", prompt, default);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();
        if input.is_empty() {
            default.to_string()
        } else {
            input.to_string()
        }
    }
}

/// Đọc file ảnh → Vec<u8>
fn read_image(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// Chuyển nội dung Markdown thành HTML cơ bản
fn markdown_to_html(path: &Path) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    let title = extract_chapter_title(path);

    let mut html = format!(
        r#"<?xml version='1.0' encoding='utf-8'?>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops">
  <head>
    <title>{title}</title>
    <link rel="stylesheet" type="text/css" href="style.css"/>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
  </head>
  <body>
"#
    );

    // Chuyển từng dòng thành <p>
    for line in content.lines().filter(|l| !l.trim().is_empty()) {
        html.push_str(&format!("<p>{}</p>\n", line.trim()));
    }

    html.push_str("</body></html>");
    Ok(html)
}

/// Lấy tiêu đề chương từ tên file
fn extract_chapter_title(path: &Path) -> String {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Chương không rõ");

    let mut title = file_stem.replace('_', ":");

    // Nếu bắt đầu bằng "Chương" thì thêm dấu ":"
    if let Some(space_idx) = title.find(' ') {
        let (prefix, rest) = title.split_at(space_idx);
        if prefix.eq_ignore_ascii_case("chương") {
            title = format!("{}:{}", prefix, rest);
        }
    }

    title.trim_matches(',').trim().to_string()
}

/// Sinh EPUB từ các file .md trong thư mục ./download/
pub fn epub_build(epub_metadata: &EpubMetadata) -> Result<(), Box<dyn std::error::Error>> {
    let directory = &epub_metadata.folder_path;
    let title = &epub_metadata.title;
    let author = &epub_metadata.author;

    // Lấy danh sách file .md và sắp xếp tự nhiên (Chương 1 < Chương 10)
    let mut chapters: Vec<PathBuf> = fs::read_dir(directory)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension()? == "md" {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    chapters.sort_by(|a, b| {
        compare(
            &a.file_name().unwrap().to_string_lossy(),
            &b.file_name().unwrap().to_string_lossy(),
        )
    });

    // CSS gọn hơn
    let css = r#"
p {
    text-indent: 1.5em;
    margin-top: 0;
    margin-bottom: 1em;
    line-height: 1.6;
}
p.first-line {
    text-indent: 0;
}
"#;

    // Ảnh bìa
    let image_data = read_image("cover.png")?;
    let cover_cursor = Cursor::new(image_data);

    // Khởi tạo EPUB builder
    let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;
    builder
        .metadata("author", author)?
        .metadata("title", title)?
        .add_resource("style.css", Cursor::new(css), "text/css")?
        .add_cover_image("cover.png", cover_cursor, "image/png")?
        .inline_toc();

    // Thêm từng chương
    for (i, path) in chapters.iter().enumerate() {
        let html = markdown_to_html(path)?;
        let chapter_title = extract_chapter_title(path);
        let xhtml_name = format!("chapter_{i}.xhtml");

        builder.add_content(
            EpubContent::new(xhtml_name, html.as_bytes())
                .title(&chapter_title)
                .reftype(ReferenceType::Text),
        )?;
    }

    let output_name = format!("{title}.epub");
    let mut file = fs::File::create(&output_name)?;
    builder.generate(&mut file)?;

    println!("EPUB đã tạo thành công: {output_name}");
    Ok(())
}
