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
