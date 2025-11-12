use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use dioxus::prelude::*;

use crate::layouts::docs::Document;
use crate::layouts::pages::MarkdownPage;
use crate::pages::home::home_page;

#[derive(PartialEq, Eq, Clone)]
pub struct Summary {
    pub source_folder: &'static str,
    pub categories: Vec<Category>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Category {
    pub name: String,
    pub pages: Vec<Page>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Page {
    pub date: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub folder: &'static str,
    pub markdown: &'static str,
    pub image: Option<&'static str>,
    pub author: Option<&'static str>,
    pub author_image: Option<&'static str>,
}

impl Page {
    pub fn permalink(&self) -> String {
        format!("https://stack-cli.com/{}", self.folder)
    }
}

pub async fn generate_home_page() {
    let html = home_page();

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub fn generate(summary: Summary) {
    let src = format!("content/{}", summary.source_folder);
    let src = Path::new(&src);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).unwrap();
}

pub fn generate_docs(summary: Summary) {
    let src = format!("content/{}", summary.source_folder);
    let src = Path::new(&src);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).unwrap();

    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                Document {
                    summary: summary.clone(),
                    category: category.clone(),
                    doc: *page,
                }
            };

            let html = crate::render(page_ele);
            let dir: PathBuf = ["dist", page.folder].iter().collect();
            fs::create_dir_all(&dir).expect("Unable to create docs directory");
            let file = dir.join("index.html");

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub async fn generate_pages(summary: Summary) {
    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                MarkdownPage {
                    post: *page
                }
            };
            let html = crate::render(page_ele);

            let file = format!("dist/{}", page.folder);

            fs::create_dir_all(&file).expect("Could not create directory");

            let file = format!("dist/{}/index.html", page.folder);

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_folder(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
