use pattern::core::PatternRegistry;
use pattern::core::PersonAnalyzer;
use std::time::{Instant};
use std::io;

fn main() {

    println!("Please input your text to analyze:");

    let mut text_to_analyze = String::new();
    io::stdin()
    .read_line(&mut text_to_analyze)
    .expect("Failed to read line");

    let start = Instant::now();
    println!("{}\n", "-".repeat(50));

    let text_to_analyze: &str = text_to_analyze.trim();

    let registry = PatternRegistry::new();
    let mut pattern = registry.detect_pattern(text_to_analyze);

    // si no encuentra un patron, entonces verifica si puede ser una LOCATION
    if pattern == "" {
        let text_without_space = &text_to_analyze.replace(" ","");
        pattern = registry.detect_person_or_location("LOCATION", &text_without_space);
    }

    // si aun no encuentra un patron, entonces verifica si puede ser un PERSON
    if pattern == "" {
        pattern = registry.detect_person_or_location("PERSON", text_to_analyze);
        
        if pattern == "PERSON" {
            let person_analyzer = PersonAnalyzer;
            let analysis_result = person_analyzer.analysis(text_to_analyze);
            if !analysis_result.is_empty() {
                pattern = analysis_result;
            } else {
                pattern = String::new();
            }
        }
    }
    
    println!("PII Type: {} \nText analyzed: {}", pattern, text_to_analyze);
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());
}
