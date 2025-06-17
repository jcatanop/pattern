use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

// Define a pattern function type
type PatternFn = Arc<dyn Fn() -> Regex + Send + Sync>;

// Helper function to create patterns
fn create_pattern(regex: &'static str) -> PatternFn {
    Arc::new(move || Regex::new(regex).unwrap())
}

pub struct PatternRegistry {
    patterns: HashMap<&'static str, PatternFn>,
}

impl PatternRegistry {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Full names of persons ->ok
        patterns.insert(
            "PERSON",
            create_pattern(r"^(?i)\p{L}+(?:\s+\p{L}+)*$"),
        );

        // Locations (cities, countries, etc.) ->ok
        patterns.insert(
            "LOCATION",
            create_pattern(r"(?i)(?:avenue|lane|road|boulevard|drive|street|ave|dr|rd|blvd|ln|st|suite|apt|unit|north|south|east|west|northeast|northwest|southeast|southwest)"),
        );

        // Phone numbers (including fax)  ->ok
        patterns.insert(
            "PHONE_NUMBER",
            create_pattern(r"^(\+\d{1,3}[-. ]?)?\(?\d{3}\)?[-. ]?\d{3}[-. ]?\d{4}$"),
        );
        
        // Email -> ok
        patterns.insert(
            "EMAIL_ADDRESS",
            create_pattern(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"),
        );
        
        // SSN US Social Security Number
        patterns.insert(
            "US_SSN",
            create_pattern(r"^\d{3}-\d{2}-\d{4}$"),
        );
       

/*    "US_ITIN": AnonymizationStrategies.REMOVE,  # US Individual Taxpayer Identification Number
    "US_DRIVER_LICENSE": AnonymizationStrategies.REMOVE,  # US Driver's License numbers
    "US_BANK_ACCOUNT": AnonymizationStrategies.REMOVE,  # US bank account numbers
    "US_PASSPORT": AnonymizationStrategies.REMOVE,  # US passport numbers
    "UK_NHS": AnonymizationStrategies.REMOVE,  # UK National Health Service Number
    "UK_NINO": AnonymizationStrategies.REMOVE,  # UK National Insurance Number
    "IBAN_CODE": AnonymizationStrategies.REMOVE,  # International Bank Account Numbers
    "NRP": AnonymizationStrategies.REMOVE,  # National Registration Number
    "MEDICAL_LICENSE": AnonymizationStrategies.REMOVE,  # Medical license numbers
    "CRYPTO": AnonymizationStrategies.REMOVE,  # Cryptocurrency addresses
 */

        // Credit card numbers
        patterns.insert(
            "CREDIT_CARD",
            create_pattern(r"^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11})$"),
        );

        // IP v4   ->ok
        patterns.insert(
            "IP_ADDRESS_V4",
            create_pattern(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$|^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$"),
        );
        
        // IP v6   ->ok
        patterns.insert(
            "IP_ADDRESS_V6",
            create_pattern(r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:([0-9a-fA-F]{1,4}:)?[0-9a-fA-F]{1,4}|::([0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}::|[0-9a-fA-F]{1,4}::([0-9a-fA-F]{1,4}:){0,5}[0-9a-fA-F]{1,4}|::([0-9a-fA-F]{1,4}:){0,6}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,6}::[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))$"),
        ); 

        Self { patterns }
    }

    pub fn get_pattern(&self, name: &str) -> Option<Regex> {
        self.patterns.get(name).map(|f| f())
    }

    pub fn get_all_patterns(&self) -> HashMap<String, Regex> {
        self.patterns
            .iter()
            .map(|(&k, v)| (k.to_string(), v()))
            .collect()
    }

    // funcion para identicar patrones diferentes a location y person
    pub fn detect_pattern(&self, text_to_analyze: &str) -> String {
        let all_patterns = self.get_all_patterns();
        for (pattern_name, pattern) in all_patterns {
            if pattern_name == "PERSON"{ continue; }
            if pattern_name == "LOCATION"{ continue; }
            if pattern.is_match(&text_to_analyze) {
                return pattern_name;
            }
        }
        "".to_string()
    }

    pub fn detect_person_or_location(&self, pattern_required: &str, text_to_analyze: &str) -> String {
        if let Some(pattern) = self.get_pattern(pattern_required) {
            if pattern.is_match(text_to_analyze) {
                return pattern_required.to_string();
            }
        }
        "".to_string()
    }

}