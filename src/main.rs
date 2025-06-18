use pattern::core::PatternRegistry;
use pattern::core::PersonAnalyzer;
use std::time::{Instant};
use std::io;

fn detect_pii(text_to_analyze: &str) -> String {
    let text_to_analyze = &text_to_analyze.trim();

    // Check if the text has any patterns like IP, credit card, US-SSN 
    let registry = PatternRegistry::new();
    let mut pattern = registry.detect_pattern(&text_to_analyze);
    // If no PII was found, check if it could be PERSON
    if pattern.is_empty() {
        pattern = registry.detect_person_or_location("PERSON", &text_to_analyze);
        
        if pattern == "PERSON" {
            let person_analyzer = PersonAnalyzer;
            let analysis_result = person_analyzer.analysis(&text_to_analyze);
            if !analysis_result.is_empty() {
                pattern = analysis_result;
            } else {
                pattern = String::new();
            }
        }
    }
    // If no PII is detected yet, proceed to check if it is LOCATION
    if pattern.is_empty() {
        let text_without_space = &text_to_analyze.replace(" ", "");
        pattern = registry.detect_person_or_location("LOCATION", &text_without_space);
    }
    pattern
}

fn main() {
    // Request a text to analyze
    println!("\nPlease input your text to analyze:");
    let mut text_to_analyze = String::new();
    io::stdin()
    .read_line(&mut text_to_analyze)
    .expect("Failed to read line");

    let start = Instant::now();
    println!("{}\n", "-".repeat(50));

    // Start PII analysis
    let pii = detect_pii(&text_to_analyze);

    // Show result
    println!("PII Type: {} \nText analyzed: {}", pii, text_to_analyze);
    let duration = start.elapsed();
    println!("Time taken: {} ms", duration.as_millis());
}