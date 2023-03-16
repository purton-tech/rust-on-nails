+++
title = "Integration Testing"
description = "Integration Testing"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 30
sort_by = "weight"


[extra]
lead = ''
toc = true
top = false
+++

Integration tests are used to test application from top to bottom. That means simulating the browser across important workflows as if it was a real user. We will use [Selenium](https://www.selenium.dev/) as our headless browser.

Add the Selenium docker container to `.devcontainer/docker-compose.yml` and restart your devcontainer. Note the *No VNC* and *VNC* comments, the selenium container allows us to connect via [VNC](https://en.wikipedia.org/wiki/Virtual_Network_Computing) to the container so we can actually see the browser as it performs the tests. The *No VNC* port means we don't even have to install VNC. You can connect with a browser to this port and use the [No VNC](https://novnc.com/info.html) browser client.

```yaml
  # Integration testing using a headless chrome browser
  selenium:
    image: selenium/standalone-chrome:4.1.1-20220121
    shm_size: 2gb
    environment:
      VNC_NO_PASSWORD: 1
    ports:
      # VNC
      - 5900:5900
      # No VNC
      - 7900:7900
```

We can write our tests in Rust using [ThirtyFour](https://github.com/stevepryde/thirtyfour) which is a Selenium / WebDriver library for Rust, for automated website UI testing.

Add the following to bottom of `app/Cargo.toml`.

```
[dev-dependencies]
# WebDriver Library for UI testing.
thirtyfour = { version = "0", default-features = false, features = [ "reqwest-rustls-tls", "tokio-runtime" ] }
```

We need a helper class to configure our selenium driver. Add the following to `app/tests/config.rs`.

```rust
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
        WebDriver::new(&self.webdriver_url, &caps).await
    }
}
```

Create the following example test in `app/tests/example_test.rs`.

```rust
pub mod config;

use thirtyfour::prelude::*;

// let's set up the sequence of steps we want the browser to take
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn run_test() -> WebDriverResult<()> {
    let config = config::Config::new().await;

    let driver = config.get_driver().await?;

    let result = test(&driver, &config).await;

    driver.quit().await?;

    result?;

    Ok(())
}

async fn test(driver: &WebDriver, config: &config::Config) -> WebDriverResult<()> {
    let delay = std::time::Duration::new(11, 0);
    driver.set_implicit_wait_timeout(delay).await?;

    driver.get(&config.host).await?;

    driver
        .find_element(By::Id("email"))
        .await?
        .send_keys("test@test.com")
        .await?;

    Ok(())
}
```

Point your browser at `http://localhost:7900` to view the tests. Run the test from the `app` folder with `cargo test`

> Production Example 
> [Cloak Integration Tests](https://github.com/purton-tech/cloak/tree/main/app/tests) and
> [Cloak docker-compose.yml](https://github.com/purton-tech/cloak/blob/main/.devcontainer/docker-compose.yml)