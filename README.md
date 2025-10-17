# Download MTC

[![Crates.io](https://img.shields.io/crates/v/download_mtc.svg)](https://crates.io/crates/download_mtc)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Mintori09/download_mtc/rust.yml?branch=main)](https://github.com/Mintori09/download_mtc/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A fast CLI tool for downloading and packaging content from MeTruyenChu into a clean EPUB format.

This tool uses a headless Chrome browser to reliably scrape chapter content, saves it locally as Markdown files, and then compiles them into a properly formatted EPUB book.

## Features

*   **Reliable Scraping**: Uses a headless browser (`headless_chrome`) to execute JavaScript and fetch chapter content, just like a real user.
*   **EPUB Generation**: Packages all downloaded content into a single, well-formatted `.epub` file using `epub-builder`.
*   **Resume Support**: Keeps track of downloaded chapters in your JSON file, so you can safely stop and resume the process without re-downloading content.
*   **Clean Formatting**: Converts raw chapter text into clean Markdown files, which are then converted to simple HTML for the EPUB.
*   **Customizable**: Allows you to specify the book title, author, and cover image for the final EPUB.

## Prerequisites

1.  **Rust**: You need the Rust toolchain installed. You can get it from rustup.rs.
2.  **Google Chrome/Chromium**: The tool requires an installed version of Google Chrome or Chromium to function.

## How It Works

The tool operates in two main stages:

1.  **Downloading**: It reads a JSON file containing a list of chapter URLs. It then launches a headless browser, navigates to each URL, extracts the chapter text, and saves it as a `.md` file in the `download/` directory.
2.  **Building**: After downloading, it reads all the `.md` files from the `download/` directory, sorts them naturally (e.g., "Chương 1" before "Chương 10"), and compiles them into a single `.epub` file with your specified metadata.

## Installation

You can install the tool directly from the source.

1.  Clone the repository:
    ```sh
    git clone https://github.com/Mintori09/download_mtc.git
    cd download_mtc
    ```

2.  Build and run the project:
    ```sh
    cargo run --release -- [COMMAND] [OPTIONS]
    ```

## Usage

### 1. Prepare your Chapter List

Create a JSON file (e.g., `chapters.json`) with a list of chapters you want to download. The structure for each chapter should be:

```json
[
  {
    "name": "Chương 1: Mở Đầu",
    "link": "https://metruyenchu.com/truyen/your-story/chuong-1",
    "is_download": false
  },
  {
    "name": "Chương 2: Diễn Biến",
    "link": "https://metruyenchu.com/truyen/your-story/chuong-2",
    "is_download": false
  }
]
```

## Prerequisites

You need to have Google Chrome or Chromium installed on your system. The library will try to find it automatically. If it's in a non-standard location, you may need to specify the path to the executable.

## Usage

Here's a simple example of how to navigate to a website and take a screenshot:

```rust
// Note: This is a hypothetical API based on the project name.
// You should adapt it to your library's actual API.

use rust_chrome_headless::{Browser, LaunchOptions};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Launch the browser
    let browser = Browser::launch(LaunchOptions::default())?;

    // Create a new tab
    let tab = browser.new_tab()?;

    // Navigate to a URL and wait for the page to load
    tab.navigate_to("https://www.rust-lang.org/")?;

    // Wait for the "Learn More" button to be visible
    tab.wait_for_element("a.button-download")?;

    // Take a screenshot
    let png_data = tab.capture_screenshot(Default::default())?;

    // Save the screenshot to a file
    std::fs::write("rust-lang-org.png", png_data)?;

    println!("Screenshot saved to rust-lang-org.png");

    Ok(())
}
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

1.  Fork the repository.
2.  Create your feature branch (`git checkout -b my-new-feature`).
3.  Commit your changes (`git commit -am 'Add some feature'`).
4.  Push to the branch (`git push origin my-new-feature`).
5.  Create a new Pull Request.

## License

This project is licensed under either of
*   Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
*   MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.
