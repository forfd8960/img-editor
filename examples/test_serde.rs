use img_editor::types::operations::{FilterType, OperationType};

fn main() {
    let op = OperationType::Filter(FilterType::Grayscale);
    let json = serde_json::to_string_pretty(&op).unwrap();
    println!("Grayscale operation:\n{}\n", json);
    
    let op = OperationType::Filter(FilterType::Blur { radius: 2.5 });
    let json = serde_json::to_string_pretty(&op).unwrap();
    println!("Blur operation:\n{}", json);
}
