use serde_derive::Serialize;

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
            offset      :1,
            limit       :10,
            page_index  :1,
            has_next,
            has_pre,
        }
    }

    pub fn new_size_10(item_count: i32, page_index: i32) -> Self {
        Self::new(item_count, page_index, 10)
    }
}

// fn main() {
//     let p1 = Page::new(100, 1, 10);
//     println!("{:?}", p1);

//     let p2 = Page::new(90, 9, 10);
//     println!("{:?}", p2);
// }

pub fn get_page_index(page_str: String) -> i32{
    let mut p = 1;
    if let Ok(parsed_page) = page_str.parse::<i32>() {
        p = parsed_page;
    }
    if p < 1 {
        p=1
    }
    p
}