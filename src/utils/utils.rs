use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptions};
use serde::{Deserialize, Serialize};

pub fn convert_to_pdf(base_64: String, pdf_options: Option<PdfOptions>) -> Result<Vec<u8>, String> {
    // Get or download Chrome binary
    let chrome_path = get_or_download_chrome()?;

    // Launch a new browser instance
    let browser = Browser::new(LaunchOptions {
        headless: true,
        sandbox: false,
        devtools: false,
        enable_gpu: false,
        enable_logging: false,
        path: Some(chrome_path),
        ignore_certificate_errors: true,
        ..Default::default()
    })
    .map_err(|err| format!("Failed to launch browser: {}", err))?;

    // Open a new tab
    let tab = browser
        .new_tab()
        .map_err(|err| format!("Failed to create new tab: {}", err))?;

    let start = current_timestamp_millis();

    // Navigate to the HTML page
    tab.navigate_to(&base_64)
        .map_err(|err| format!("Failed to navigate: {}", err))?;

    // Wait for navigation to complete
    tab.wait_until_navigated()
        .map_err(|err| format!("Navigation timeout: {}", err))?;

    // Build print options from user input or use defaults
    let print_options = pdf_options.unwrap_or_default().to_print_options();

    // Generate PDF
    let pdf_data = tab
        .print_to_pdf(Some(print_options))
        .map_err(|err| format!("Failed to generate PDF: {}", err))?;

    let end = current_timestamp_millis();

    println!("PDF conversion completed in {} ms", end - start);

    tab.close_with_unload().ok();

    Ok(pdf_data)
}

fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// Convert millimeters to inches for Chrome PDF API
fn mm_to_inches(mm: f64) -> f64 {
    mm / 25.4
}

/// Get Chrome binary path or download if not found
fn get_or_download_chrome() -> Result<PathBuf, String> {
    // Check environment variable first
    if let Ok(chrome_path) = env::var("CHROME_PATH") {
        let path = PathBuf::from(&chrome_path);
        if path.exists() {
            println!("Using Chrome from CHROME_PATH: {}", chrome_path);
            return Ok(path);
        }
    }

    // Check default location
    let default_path = PathBuf::from("/home/rs_pdf/chrome/chrome");
    if default_path.exists() {
        println!("Using Chrome from default location");
        return Ok(default_path);
    }

    // Download Chrome if not found
    println!("Chrome not found. Downloading latest stable version...");
    download_chrome()
}

/// Download latest stable Chrome for Linux
fn download_chrome() -> Result<PathBuf, String> {
    use std::process::Command;

    let chrome_dir = PathBuf::from("/home/rs_pdf/chrome");
    fs::create_dir_all(&chrome_dir)
        .map_err(|e| format!("Failed to create chrome directory: {}", e))?;

    println!("Downloading Chromium...");

    // Use headless_chrome's fetch feature to download Chrome
    // Or download manually using a known stable URL
    let chrome_url = "https://storage.googleapis.com/chrome-for-testing-public/131.0.6778.87/linux64/chrome-linux64.zip";

    let zip_path = chrome_dir.join("chrome.zip");

    // Download using reqwest
    let response = reqwest::blocking::get(chrome_url)
        .map_err(|e| format!("Failed to download Chrome: {}", e))?;

    let mut file =
        File::create(&zip_path).map_err(|e| format!("Failed to create zip file: {}", e))?;

    let bytes = response
        .bytes()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    file.write_all(&bytes)
        .map_err(|e| format!("Failed to write zip file: {}", e))?;

    println!("Downloaded Chrome. Extracting...");

    // Extract zip
    let file = File::open(&zip_path).map_err(|e| format!("Failed to open zip: {}", e))?;

    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Failed to read zip: {}", e))?;

    archive
        .extract(&chrome_dir)
        .map_err(|e| format!("Failed to extract zip: {}", e))?;

    // Clean up zip file
    fs::remove_file(&zip_path).ok();

    println!("Chrome extracted successfully");

    // Find the chrome binary
    let chrome_binary = chrome_dir.join("chrome-linux64/chrome");

    if !chrome_binary.exists() {
        return Err("Chrome binary not found after extraction".to_string());
    }

    // Make it executable
    #[cfg(unix)]
    {
        Command::new("chmod")
            .arg("+x")
            .arg(&chrome_binary)
            .output()
            .map_err(|e| format!("Failed to make chrome executable: {}", e))?;
    }

    println!("Chrome ready at: {}", chrome_binary.display());

    Ok(chrome_binary)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PdfOptions {
    #[serde(default = "default_false")]
    pub landscape: bool,

    #[serde(default = "default_true")]
    pub display_header_footer: bool,

    #[serde(default = "default_true")]
    pub print_background: bool,

    #[serde(default = "default_scale")]
    pub scale: f64,

    /// Paper width in millimeters
    #[serde(default = "default_paper_width")]
    pub paper_width: f64,

    /// Paper height in millimeters
    #[serde(default = "default_paper_height")]
    pub paper_height: f64,

    /// Top margin in millimeters
    #[serde(default = "default_margin")]
    pub margin_top: f64,

    /// Bottom margin in millimeters
    #[serde(default = "default_margin")]
    pub margin_bottom: f64,

    /// Left margin in millimeters
    #[serde(default = "default_margin")]
    pub margin_left: f64,

    /// Right margin in millimeters
    #[serde(default = "default_margin")]
    pub margin_right: f64,

    #[serde(default)]
    pub page_ranges: String,

    #[serde(default)]
    pub header_template: String,

    #[serde(default = "default_footer_template")]
    pub footer_template: String,

    #[serde(default = "default_false")]
    pub prefer_css_page_size: bool,

    /// Whether to show page numbers in the footer
    #[serde(default = "default_true")]
    pub show_page_numbers: bool,
}

// Default value functions for serde
fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_scale() -> f64 {
    1.0
}

fn default_paper_width() -> f64 {
    210.0 // A4 width in mm
}

fn default_paper_height() -> f64 {
    297.0 // A4 height in mm
}

fn default_margin() -> f64 {
    10.0 // 10mm default margin
}

fn default_footer_template() -> String {
    r#"<div style="width:100%;font-size:10px;color:#555;text-align:center;">Page <span class="pageNumber"></span> of <span class="totalPages"></span></div>"#.to_string()
}

impl Default for PdfOptions {
    fn default() -> Self {
        Self {
            landscape: false,
            display_header_footer: true,
            print_background: true,
            scale: 1.0,
            paper_width: 210.0,  // A4 width in mm
            paper_height: 297.0, // A4 height in mm
            margin_top: 10.0,    // 10mm
            margin_bottom: 10.0, // 10mm
            margin_left: 10.0,   // 10mm
            margin_right: 10.0,  // 10mm
            page_ranges: String::new(),
            header_template: String::new(),
            footer_template: default_footer_template(),
            prefer_css_page_size: false,
            show_page_numbers: true,
        }
    }
}

impl PdfOptions {
    pub fn to_print_options(&self) -> PrintToPdfOptions {
        // Determine footer template based on show_page_numbers
        let footer = if self.footer_template.is_empty() {
            if self.show_page_numbers {
                Some(default_footer_template())
            } else {
                Some(String::from("<div></div>"))
            }
        } else {
            Some(self.footer_template.clone())
        };

        PrintToPdfOptions {
            landscape: Some(self.landscape),
            display_header_footer: Some(self.display_header_footer),
            print_background: Some(self.print_background),
            scale: Some(self.scale),
            // Convert mm to inches for Chrome API
            paper_width: Some(mm_to_inches(self.paper_width)),
            paper_height: Some(mm_to_inches(self.paper_height)),
            margin_top: Some(mm_to_inches(self.margin_top)),
            margin_bottom: Some(mm_to_inches(self.margin_bottom)),
            margin_left: Some(mm_to_inches(self.margin_left)),
            margin_right: Some(mm_to_inches(self.margin_right)),
            page_ranges: if self.page_ranges.is_empty() {
                None
            } else {
                Some(self.page_ranges.clone())
            },
            ignore_invalid_page_ranges: Some(false),
            header_template: if self.header_template.is_empty() {
                None
            } else {
                Some(self.header_template.clone())
            },
            footer_template: footer,
            prefer_css_page_size: Some(self.prefer_css_page_size),
            transfer_mode: None,
            generate_document_outline: Some(false),
            generate_tagged_pdf: Some(false),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReqType {
    pub html: String,

    #[serde(default)]
    pub pdf_options: Option<PdfOptions>,
}
