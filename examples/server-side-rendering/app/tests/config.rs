use std::env;
use thirtyfour::prelude::*;

#[derive(Clone, Debug)]
pub struct Config {
    pub webdriver_url: String,
    pub host: String,
}

impl Config {
    pub async fn new() -> Config {
        let webdriver_url: String = if env::var("WEB_DRIVER_URL").is_ok() {
            env::var("WEB_DRIVER_URL").unwrap()
        } else {
            // Default to selenium in our dev container
            "http://selenium:4444".into()
        };

        let host = if env::var("WEB_DRIVER_DESTINATION_HOST").is_ok() {
            env::var("WEB_DRIVER_DESTINATION_HOST").unwrap()
        } else {
            "http://auth:9090".into()
        };

        Config {
            webdriver_url,
            host,
        }
    }

    pub async fn get_driver(&self) -> WebDriverResult<WebDriver> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-gpu")?;
        caps.add_chrome_arg("--start-maximized")?;
        // We need the below otherwise window.crypto.subtle is not defined
        caps.add_chrome_arg("--unsafely-treat-insecure-origin-as-secure=http://envoy:7100")?;
        WebDriver::new(&self.webdriver_url, &caps).await
    }
}
