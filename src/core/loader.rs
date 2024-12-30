use std::path::Path;

use bytes::Bytes;
use reqwest::Client;

use crate::{core::zip::extract_file_from_zip_bytes, models::chrome_driver::ChromeDriverRoot};


// The latest versions for which all CfT assets are available for download, for each Chrome release channel (Stable/Beta/Dev/Canary) with an extra downloads property for each channel, listing the full download URLs per asset.
const LATEST_CHROME_DRIVER_API: &str = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
const LATEST_GECKO_DRIVER_RELEASES_PAGE: &str = "https://github.com/mozilla/geckodriver/releases/latest";
const LATEST_OPERA_DRIVER_RELEASES_PAGE: &str = "https://github.com/operasoftware/operachromiumdriver/releases/latest";
const PLATFORM: &str = "win64";

pub async fn load_drivers(output_dir: &Path) {
    log::info!("loading drivers");
    let client = reqwest::Client::new();
    load_newest_chrome_driver(&client, output_dir).await;
    load_newest_gecko_driver(&client, output_dir).await;
    load_newest_opera_driver(&client, output_dir).await;
}

pub async fn load_newest_chrome_driver(client: &Client, output_dir: &Path) {
    log::info!("loading newest chrome driver");
    let res = reqwest_for_chrome_driver_root(&client).await;

    let online_version = res.channels.stable.version.clone();
    log::debug!("Available stable version of chrome driver is {}", online_version);

    let url = res.channels.stable.downloads.chromedriver
    .iter()
    .find( |e| e.platform == PLATFORM )
    .expect("No such platform")
    .url.clone();

    log::trace!("Url to download {}", url);
    let bytes = load_bytes_from_url(&client, &url).await.expect("Error while loading from url");
    let location_extracted_file = "chromedriver-win64/chromedriver.exe";
    let extracted_file_name = "chromedriver.exe";
    extract_file_from_zip_bytes(&bytes, location_extracted_file, extracted_file_name, &output_dir).expect("Error while extracting from zip");
    log::info!("Chrome driver was successfully loaded");
}

async fn reqwest_for_chrome_driver_root(client: &Client) -> ChromeDriverRoot {
    log::trace!("Making reqwest for chrome driver root");
    let response = client
    .get(LATEST_CHROME_DRIVER_API)
    .send()
    .await
    .inspect(|ok| { log::debug!("Received a response from API with code {}", ok.status().as_str()); } )
    .inspect_err(|_err| { log::error!("Failed to get response from API"); })
    .unwrap()
    ;

    let obj = response
    .json::<ChromeDriverRoot>()
    .await
    .inspect(|_ok| { log::debug!("Successful serialize of API json"); } )
    .inspect_err(|_err| { log::error!("Failed to serialize json from API"); })
    .unwrap()
    ;
    obj
}

async fn load_bytes_from_url(client: &Client, url: &str) -> Result<Bytes, reqwest::Error> {
    log::debug!("loading from url {}", url);
    let response = client
    .get(url)
    .send()
    .await?;
    log::debug!("load from url returned code {}", response.status().as_str());
    let bytes = response
    .bytes()
    .await?;
    Ok(bytes)
}



pub async fn load_newest_gecko_driver(client: &Client, output_dir: &Path) {
    log::info!("loading newest gecko driver");
    let response = client.get(LATEST_GECKO_DRIVER_RELEASES_PAGE) // Replace with your initial URL
    .send()
    .await
    .inspect(|ok| { log::debug!("Received a response from page with code {}", ok.status().as_str()); } )
    .inspect_err(|_err| { log::error!("Failed to get response from page"); })
    .unwrap()
    ;

    // Get the final URL after redirection
    let actual_url = response.url();
    log::debug!("Gecko latest release actual url: {}", actual_url);

    // https://github.com/mozilla/geckodriver/releases/tag/v0.35.0
    let actual_url_string = actual_url.to_string();
    let list: Vec<&str> = actual_url_string.split('v').collect();
    let latest_version = list.last().unwrap();
    log::trace!("latest verion of gecko driver is {}", latest_version);
    // https://github.com/mozilla/geckodriver/releases/download/v0.35.0/geckodriver-v0.35.0-win64.zip
    let link_to_download = format!("https://github.com/mozilla/geckodriver/releases/download/v{}/geckodriver-v{}-win64.zip", latest_version, latest_version);
    let bytes = load_bytes_from_url(&client, &link_to_download).await.unwrap();

    let location_extracted_file = "geckodriver.exe";
    let extracted_file_name = "geckodriver.exe";
    extract_file_from_zip_bytes(&bytes, location_extracted_file, extracted_file_name, &output_dir).expect("Error while extracting from zip");
    log::info!("Gecko driver was successfully loaded");
}



pub async fn load_newest_opera_driver(client: &Client, output_dir: &Path) {
    log::info!("loading newest opera driver");
    let response = client.get(LATEST_OPERA_DRIVER_RELEASES_PAGE) // Replace with your initial URL
    .send()
    .await
    .inspect(|ok| { log::debug!("Received a response from page with code {}", ok.status().as_str()); } )
    .inspect_err(|_err| { log::error!("Failed to get response from page"); })
    .unwrap()
    ;

    // Get the final URL after redirection
    let actual_url = response.url();
    log::debug!("Opera latest release actual url: {}", actual_url);

    // https://github.com/operasoftware/operachromiumdriver/releases/tag/v.130.0.6723.137
    let actual_url_string = actual_url.to_string();
    let list: Vec<&str> = actual_url_string.split("v.").collect();
    let latest_version = list.last().unwrap();
    log::trace!("latest verion of opera driver is {}", latest_version);
    // https://github.com/operasoftware/operachromiumdriver/releases/download/v.130.0.6723.137/operadriver_win64.zip
    let link_to_download = format!("https://github.com/operasoftware/operachromiumdriver/releases/download/v.{}/operadriver_win64.zip", latest_version);
    let bytes = load_bytes_from_url(&client, &link_to_download).await.unwrap();

    let location_extracted_file = "operadriver_win64/operadriver.exe";
    let extracted_file_name = "operadriver.exe";
    extract_file_from_zip_bytes(&bytes, location_extracted_file, extracted_file_name, &output_dir).expect("Error while extracting from zip");
    log::info!("Opera driver was successfully loaded");
}


