use lingua::Language::{English, French, German, Spanish, Swedish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use log::info;

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

    pub fn is_english_language(&self, text: &str) -> bool {
        let detected_language: Option<Language> = self.detector.detect_language_of(text);
        if detected_language == Some(English) {
            return true;
        } else {
            info!("not english: {}", text);
            return false;
        }
    }
}