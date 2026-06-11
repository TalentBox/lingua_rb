use std::collections::HashSet;
use std::str::FromStr;

use lingua::Language as LinguaLanguage;
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use magnus::{function, method, prelude::*, wrap, Error, Ruby, Symbol};

fn detect(ruby: &Ruby, arguments: magnus::RArray) -> Option<String> {
    match arguments.len() {
        1 => {
            let subject = arguments.shift::<String>().unwrap();
            let detector: LanguageDetector = LanguageDetectorBuilder::from_all_languages().build();
            let detected_language: Option<LinguaLanguage> = detector.detect_language_of(subject);

            detected_language.map(|language| language.to_string())
        }
        2 => {
            let subject = arguments.shift::<String>().unwrap();
            let options = arguments.shift::<magnus::RHash>().unwrap();
            let mut builder =
                match options.fetch::<Symbol, Vec<String>>(ruby.to_symbol("languages")) {
                    Ok(languages) => {
                        let languages: Vec<LinguaLanguage> = languages
                            .into_iter()
                            .filter_map(|l| LinguaLanguage::from_str(&l).ok())
                            .collect();
                        Some(LanguageDetectorBuilder::from_languages(&languages))
                    }
                    Err(_) => None,
                };
            if builder.is_none() {
                builder = match options.fetch::<&str, Vec<String>>("languages") {
                    Ok(languages) => {
                        let languages: Vec<LinguaLanguage> = languages
                            .into_iter()
                            .filter_map(|l| LinguaLanguage::from_str(&l).ok())
                            .collect();
                        Some(LanguageDetectorBuilder::from_languages(&languages))
                    }
                    Err(_) => None,
                };
            }
            let mut builder = builder.unwrap_or_else(LanguageDetectorBuilder::from_all_languages);
            if let Ok(minimum_relative_distance) =
                options.fetch::<&str, f64>("minimum_relative_distance")
            {
                builder.with_minimum_relative_distance(minimum_relative_distance);
            };
            if let Ok(minimum_relative_distance) =
                options.fetch::<Symbol, f64>(ruby.to_symbol("minimum_relative_distance"))
            {
                builder.with_minimum_relative_distance(minimum_relative_distance);
            };
            if options
                .fetch::<&str, bool>("is_every_language_model_preloaded")
                .unwrap_or(false)
                || options
                    .fetch::<Symbol, bool>(ruby.to_symbol("is_every_language_model_preloaded"))
                    .unwrap_or(false)
            {
                builder.with_preloaded_language_models();
            };
            if options
                .fetch::<&str, bool>("is_low_accuracy_mode_enabled")
                .unwrap_or(false)
                || options
                    .fetch::<Symbol, bool>(ruby.to_symbol("is_low_accuracy_mode_enabled"))
                    .unwrap_or(false)
            {
                builder.with_low_accuracy_mode();
            };
            let detector = builder.build();
            let detected_language: Option<LinguaLanguage> = detector.detect_language_of(subject);

            detected_language.map(|language| language.to_string())
        }
        _ => None,
    }
}

#[wrap(class = "Lingua::Language")]
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Language {
    pub lang: String,
    pub iso_code_639_1: String,
    pub iso_code_639_3: String,
}

impl Language {
    fn lang(&self) -> String {
        self.lang.clone()
    }
    fn iso_code_639_1(&self) -> String {
        self.iso_code_639_1.clone()
    }
    fn iso_code_639_3(&self) -> String {
        self.iso_code_639_3.clone()
    }
}

fn language_all(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all())
}

fn language_all_spoken_ones(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all_spoken_ones())
}

fn language_all_with_arabic_script(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all_with_arabic_script())
}

fn language_all_with_cyrillic_script(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all_with_cyrillic_script())
}

fn language_all_with_devanagari_script(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all_with_devanagari_script())
}

fn language_all_with_latin_script(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all_with_latin_script())
}

fn language_all_with_single_unique_script(ruby: &Ruby) -> magnus::RArray {
    language_hash_set_to_ruby_array(ruby, LinguaLanguage::all_with_single_unique_script())
}

fn language_from_lang(lang: magnus::RString) -> Option<Language> {
    if let Ok(str) = lang.to_string() {
        if let Ok(language) = lingua::Language::from_str(&str) {
            return Some(Language {
                lang: language.to_string(),
                iso_code_639_1: language.iso_code_639_1().to_string(),
                iso_code_639_3: language.iso_code_639_3().to_string(),
            });
        }
    }
    None
}

fn language_from_iso_code_639_1(iso_code: magnus::RString) -> Option<Language> {
    let Ok(iso_str) = iso_code.to_string() else {
        return None;
    };
    let Ok(iso) = lingua::IsoCode639_1::from_str(&iso_str) else {
        return None;
    };
    let language = LinguaLanguage::from_iso_code_639_1(&iso);
    Some(Language {
        lang: language.to_string(),
        iso_code_639_1: language.iso_code_639_1().to_string(),
        iso_code_639_3: language.iso_code_639_3().to_string(),
    })
}

fn language_from_iso_code_639_3(iso_code: magnus::RString) -> Option<Language> {
    let Ok(iso_str) = iso_code.to_string() else {
        return None;
    };
    let Ok(iso) = lingua::IsoCode639_3::from_str(&iso_str) else {
        return None;
    };
    let language = LinguaLanguage::from_iso_code_639_3(&iso);
    Some(Language {
        lang: language.to_string(),
        iso_code_639_1: language.iso_code_639_1().to_string(),
        iso_code_639_3: language.iso_code_639_3().to_string(),
    })
}

fn language_hash_set_to_ruby_array(
    ruby: &Ruby,
    languages: HashSet<LinguaLanguage>,
) -> magnus::RArray {
    let result = ruby.ary_new_capa(languages.len());
    for language in languages {
        let _ = result.push(Language {
            lang: language.to_string(),
            iso_code_639_1: language.iso_code_639_1().to_string(),
            iso_code_639_3: language.iso_code_639_3().to_string(),
        });
    }
    result
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Lingua")?;
    module.define_singleton_method("detect", function!(detect, -2))?;

    let language_class = module.define_class("Language", ruby.class_object())?;
    language_class.define_method("lang", method!(Language::lang, 0))?;
    language_class.define_method("iso_code_639_1", method!(Language::iso_code_639_1, 0))?;
    language_class.define_method("iso_code_639_3", method!(Language::iso_code_639_3, 0))?;
    language_class.define_singleton_method("from_lang", function!(language_from_lang, 1))?;
    language_class.define_singleton_method(
        "from_iso_code_639_1",
        function!(language_from_iso_code_639_1, 1),
    )?;
    language_class.define_singleton_method(
        "from_iso_code_639_3",
        function!(language_from_iso_code_639_3, 1),
    )?;
    language_class.define_singleton_method("all", function!(language_all, 0))?;
    language_class
        .define_singleton_method("all_spoken_ones", function!(language_all_spoken_ones, 0))?;
    language_class.define_singleton_method(
        "all_with_arabic_script",
        function!(language_all_with_arabic_script, 0),
    )?;
    language_class.define_singleton_method(
        "all_with_cyrillic_script",
        function!(language_all_with_cyrillic_script, 0),
    )?;
    language_class.define_singleton_method(
        "all_with_devanagari_script",
        function!(language_all_with_devanagari_script, 0),
    )?;
    language_class.define_singleton_method(
        "all_with_latin_script",
        function!(language_all_with_latin_script, 0),
    )?;
    language_class.define_singleton_method(
        "all_with_single_unique_script",
        function!(language_all_with_single_unique_script, 0),
    )?;
    Ok(())
}
