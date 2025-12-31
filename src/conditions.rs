pub fn get_park_url(park_code: &str) -> String {
    format!("https://www.sepaq.com/pq/{}/", park_code.to_lowercase())
}

pub fn format_condition_url(url: &str, link_icon: &str) -> String {
    format!("{} Check conditions: {}", link_icon, url)
}
