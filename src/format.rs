pub fn format_memory(bytes: u64) -> String {
    let kb = 1024_f64;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;

    let bytes_f64 = bytes as f64;

    if bytes_f64 >= gb {
        format!("{:.5} GB", bytes_f64 / gb)
    } else if bytes_f64 >= mb {
        format!("{:.5} MB", bytes_f64 / mb) 
    } else if bytes_f64 >= kb {
        format!("{:.5} KB", bytes_f64 / kb)
    } else {
        format!("{:.5} B", bytes_f64)
    }
}