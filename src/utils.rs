use serde_derive::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
pub struct Page {
    pub item_count: i32,
    pub page_size: i32,
    pub page_count: i32,
    pub offset: i32,
    pub limit: i32,
    pub page_index: i32,
    pub has_next: bool,
    pub has_pre: bool,
}

impl Page {
    pub fn new(item_count: i32, page_index: i32, page_size: i32) -> Self {
        let page_count = item_count / page_size
            + if item_count % page_size > 0 { 1 } else { 0 };

        let (offset, limit, page_index) = if item_count == 0 || page_index > page_count {
            (0, 0, 1)
        } else {
            (page_size * (page_index - 1), page_size, page_index)
        };

        let has_next = page_index < page_count;
        let has_pre = page_index > 1;

        Page {
            item_count,
            page_size,
            page_count,
            offset,
            limit,
            page_index,
            has_next,
            has_pre,
        }
    }

    pub fn new_size_10(item_count: i32, page_index: i32) -> Self {
        Self::new(item_count, page_index, 10)
    }
}

pub fn get_page_index(page_str: String) -> i32{
    
    let mut p = if let Ok(parsed_page) = page_str.parse::<i32>() {parsed_page} else { 1 };

    if p < 1 {
        p=1
    }
    p
}

pub fn list_files_in_directory(path: &str) -> Vec<String> {
    let entries = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut file_names = Vec::new();
    for entry in entries {
        if let Some(name) = entry.file_name().and_then(|n| n.to_str()) {
            file_names.push(name.to_string());
        }
    }

    file_names
}

pub fn list_files(path:&str) -> Vec<String> {
    let path_replace = path.replace("-", "/");

    let entries = fs::read_dir(path_replace)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut file_names = Vec::new();

    for entry in entries {
        if let Some(name) = entry.file_name().and_then(|n| n.to_str()) {
            file_names.push(name.to_string());
        }
    }
    file_names
}