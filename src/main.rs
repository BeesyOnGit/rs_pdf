use std::{
    fs::read_to_string,
    path::PathBuf,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use headless_chrome::{Browser, LaunchOptions, types::PrintToPdfOptions};

fn main() {
    // importing the path to the chromium headless_shell binary
    let path = match PathBuf::from_str("/home/pdf/build/headless-shell/headless_shell") {
        Ok(path) => path,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // setting up the browser config
    let browser_config = match LaunchOptions::default_builder().path(Some(path)).build() {
        Ok(conf) => conf,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // launching a new browser
    let browser = match Browser::new(browser_config) {
        Ok(browser) => browser,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // importing an html file to test
    let html = match read_from_file_ut("./deep_seek.html") {
        Ok(html) => html,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // convert html to base64 dataUrl
    let data_url = format!("data:text/html;base64,{}", STANDARD.encode(html));

    // open a new tab
    let tab = match browser.new_tab() {
        Ok(tab) => tab,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // timer start
    let start = current_timestamp_millis();

    // navigate to our html page
    let _ = match tab.navigate_to(&data_url) {
        Ok(_b) => _b,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // wait for navigation
    match tab.wait_until_navigated() {
        Ok(_b) => _b,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // setting the print options
    let print_options = PrintToPdfOptions {
        print_background: Some(true),
        display_header_footer: Some(true),
        margin_bottom: Some(convert_mm_to_in(15)),
        margin_top: Some(convert_mm_to_in(15)),
        margin_left: Some(convert_mm_to_in(10)),
        margin_right: Some(convert_mm_to_in(10)),
        header_template: Some(
            r#"<div style="width:100%;
                font-size:10px;
                color:#555;
                text-align:center;">
                
                </div>"#
                .into(),
        ),
        footer_template: Some(
            r#"
                <div style="width:100%;
                font-size:10px;
                color:#555;
                text-align:center;">Page 
                <span class="pageNumber"></span> sur <span class="totalPages"></span>
                </div>
                "#
            .into(),
        ),
        ..Default::default()
    };

    // do whatever you please with the result pdf array buffer ex: save to file, send via http...
    let _pdf_data = match tab.print_to_pdf(Some(print_options)) {
        Ok(pdf) => pdf,
        Err(err) => {
            println!("error occured : {err}");
            return;
        }
    };

    // timer end
    let end = current_timestamp_millis();

    // display some informations
    println!("operation took : {} ms", end - start);

    tab.close_with_unload().unwrap();
}

pub fn read_from_file_ut(file_path: &str) -> Result<String, String> {
    match read_to_string(file_path) {
        Ok(f) => return Ok(f),
        Err(err) => {
            return Err(err.to_string());
        }
    };
}
fn convert_mm_to_in(mm: i32) -> f64 {
    return mm as f64 / 25.4;
}

fn current_timestamp_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
