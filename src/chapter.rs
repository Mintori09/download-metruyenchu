use crate::{cookies::create_cookies, model::Chapter};
use headless_chrome::Browser;
use scraper::{Html, Selector};

/// Trích xuất danh sách chương từ nội dung HTML
fn extract_chapters_from_html(html: &str) -> Vec<Chapter> {
    let document = Html::parse_document(html);
    let chapter_selector =
        Selector::parse("a[data-x-bind^=\"ChapterItem\"]").expect("Selector chapter không hợp lệ");
    let title_selector = Selector::parse("div[data-x-text=\"chapter.name\"]")
        .expect("Selector tiêu đề không hợp lệ");

    document
        .select(&chapter_selector)
        .filter_map(|element| {
            let link = element.value().attr("href")?;
            let name = element
                .select(&title_selector)
                .next()
                .map(|n| n.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "Không rõ tên".to_string());

            Some(Chapter {
                name,
                link: link.to_string(),
                is_download: false,
            })
        })
        .collect()
}

/// Lấy danh sách chương từ một URL (truy cập qua headless Chrome)
pub fn get_chapters(url: &str) -> Result<Vec<Chapter>, Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    // Thiết lập cookie và điều hướng
    tab.set_cookies(create_cookies(url))?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    // Mở danh sách chương
    if let Ok(button) = tab.wait_for_element("button[data-x-bind^='ChapterOpenToc']") {
        button.click()?;
        tab.wait_for_element("div.flex-auto.overflow-y-auto")?;
    }

    // Lấy HTML sau khi danh sách chương xuất hiện
    let html_content = tab.get_content()?;
    tab.close(true)?;

    Ok(extract_chapters_from_html(&html_content))
}
