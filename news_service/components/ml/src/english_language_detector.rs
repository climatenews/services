use lingua::Language::{English, French, German, Spanish, Swedish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

pub struct EnglishLanguageDetector {
    detector: LanguageDetector,
}

impl EnglishLanguageDetector {
    pub fn new() -> EnglishLanguageDetector {
        let languages = vec![English, French, German, Spanish, Swedish];
        let detector: LanguageDetector =
            LanguageDetectorBuilder::from_languages(&languages).build();
        EnglishLanguageDetector { detector }
    }

    pub fn is_english_language(&self, text: String) -> bool {
        let detected_language: Option<Language> = self.detector.detect_language_of(text.clone());
        if detected_language == Some(English) {
            return true;
        } else {
            println!("not english: {}", text);
            return false;
        }
    }
}
