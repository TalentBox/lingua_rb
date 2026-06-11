# frozen_string_literal: true

require "test_helper"

class TestLingua < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::Lingua::VERSION
  end

  def test_it_detect_english
    assert_equal "English", Lingua.detect("this is definitely English")
  end

  def test_it_detect_chinese
    assert_equal "Chinese", Lingua.detect("你好，我来自中国")
  end

  def test_it_detect_hebrew
    assert_equal "Hebrew", Lingua.detect("וזה בעברית")
  end

  def test_it_detect_polish_with_options
    assert_equal "Polish", Lingua.detect("państwowych", languages: %w[english russian polish])
  end

  def test_it_detect_polish_with_string_options
    assert_equal "Polish", Lingua.detect("państwowych", { "languages": %w[english russian polish] })
  end

  def test_it_detect_polish_with_symble_options
    assert_equal "Polish", Lingua.detect("państwowych", { languages: %w[english russian polish] })
  end

  def test_it_detect_nil_with_options
    assert_nil Lingua.detect("כלב", languages: %w[english russian polish])
  end

  def test_it_detect_nil_with_string_options
    assert_nil Lingua.detect("כלב", { "languages" => %w[english russian polish] })
  end

  def test_it_detect_nil_with_symble_options
    assert_nil Lingua.detect("כלב", { languages: %w[english russian polish] })
  end

  def test_it_support_minimum_relative_distance
    assert_nil Lingua.detect(
      "languages are awesome",
      languages: %w[english french german spanish],
      minimum_relative_distance: 0.9
    )
  end

  def test_it_support_is_every_language_model_preloaded
    assert "English", Lingua.detect(
      "languages are awesome",
      languages: %w[english french german spanish],
      is_every_language_model_preloaded: true
    )
  end

  def test_it_support_is_low_accuracy_mode_enabled
    assert "English", Lingua.detect(
      "languages are awesome",
      languages: %w[english french german spanish],
      is_low_accuracy_mode_enabled: true
    )
  end

  def test_language_from_lang_with_match
    language = Lingua::Language.from_lang("English")
    assert language.is_a?(Lingua::Language)
    assert_equal "English", language.lang
    assert_equal "en", language.iso_code_639_1
    assert_equal "eng", language.iso_code_639_3
  end

  def test_language_from_lang_without_match
    assert_nil Lingua::Language.from_lang("Unknown")
  end

  def test_language_from_iso_code_639_1_with_match
    language = Lingua::Language.from_iso_code_639_1("fr")
    assert language.is_a?(Lingua::Language)
    assert_equal "French", language.lang
    assert_equal "fr", language.iso_code_639_1
    assert_equal "fra", language.iso_code_639_3
  end

  def test_language_from_iso_code_639_1_without_match
    assert_nil Lingua::Language.from_iso_code_639_1("zz")
  end

  def test_language_from_iso_code_639_3_with_match
    language = Lingua::Language.from_iso_code_639_3("nld")
    assert language.is_a?(Lingua::Language)
    assert_equal "Dutch", language.lang
    assert_equal "nl", language.iso_code_639_1
    assert_equal "nld", language.iso_code_639_3
  end

  def test_language_from_iso_code_639_3_without_match
    assert_nil Lingua::Language.from_iso_code_639_3("zzz")
  end

  def test_language_all
    assert_languages_array Lingua::Language.all, 75
  end

  def test_language_all_spoken_ones
    assert_languages_array Lingua::Language.all_spoken_ones, 74
  end

  def test_language_all_with_arabic_script
    assert_languages_array Lingua::Language.all_with_arabic_script, 3
  end

  def test_language_all_with_cyrillic_script
    assert_languages_array Lingua::Language.all_with_cyrillic_script, 8
  end

  def test_language_all_with_devanagari_script
    assert_languages_array Lingua::Language.all_with_devanagari_script, 2
  end

  def test_language_all_with_latin_script
    assert_languages_array Lingua::Language.all_with_latin_script, 49
  end

  def test_language_all_with_single_unique_script
    assert_languages_array Lingua::Language.all_with_single_unique_script, 11
  end

  private

  def assert_languages_array(values, expected_size)
    assert_equal expected_size, values.size
    assert values.all? { |l| l.is_a? Lingua::Language }, "returns an array of Lingua::Language"
    assert values[0].lang.is_a?(String)
    assert values[0].iso_code_639_1.is_a?(String)
    assert values[0].iso_code_639_3.is_a?(String)
  end
end
