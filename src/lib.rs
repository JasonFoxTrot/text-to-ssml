extern crate quick_xml;

pub mod ssml_constants;
pub mod xml_writer;

use std::collections::BTreeMap;

/// Parses a String into the Unique Text to SSML Format. Useful for taking a string
/// and making some sweet, sweet SSML.
pub fn parse_string(to_parse: String) -> Result<String, quick_xml::errors::Error> {
  let mut xml_writer = try!(xml_writer::XmlWriter::new());
  try!(xml_writer.start_ssml_speak(None, None));

  for line in to_parse.lines() {
    if line.is_empty() {
      continue;
    }
    try!(xml_writer.start_ssml_paragraph());
    let mut is_first: bool = true;
    for word in line.split_whitespace() {
      if word.is_empty() {
        continue;
      }
      if !is_first {
        try!(xml_writer.write_text(" "));
      } else {
        is_first = false;
      }
      if word.starts_with("${/") && word != "${/" {
        // Should only be two parts, take the second.
        let split_round_one = word.split("${/").last();
        if split_round_one.is_none() {
          return Err(quick_xml::errors::ErrorKind::Msg("Empty End Tag".to_owned()).into())
        }
        let split_round_two = split_round_one.unwrap().split("}").next();
        if split_round_two.is_none() {
          return Err(quick_xml::errors::ErrorKind::Msg("No closing bracket, end".to_owned()).into())
        }
        let split_frd = split_round_two.unwrap();
        let as_tag = split_frd.parse::<ssml_constants::PossibleClosingTags>();
        if as_tag.is_err() {
          continue;
        } else {
          use ssml_constants::PossibleClosingTags::*;
          let _ = match as_tag.unwrap() {
            LangTag => xml_writer.end_ssml_lang(),
            Mark => xml_writer.end_ssml_mark(),
            Paragraph => xml_writer.end_ssml_paragraph(),
            Phoneme => xml_writer.end_ssml_phoneme(),
            Prosody => xml_writer.end_ssml_prosody(),
            Sentence => xml_writer.end_ssml_sentence(),
            SayAs => xml_writer.end_ssml_say_as(),
            Sub => xml_writer.end_ssml_sub(),
            Word => xml_writer.end_ssml_w(),
            AmazonEffect => xml_writer.end_ssml_amazon_effect()
          };
        }
      } else if word.starts_with("${") && word != "${" {
        // Should only be two parts, take the second.
        let split_round_one = word.split("${").last();
        if split_round_one.is_none() {
          return Err(quick_xml::errors::ErrorKind::Msg("Empty start Tag".to_owned()).into())
        }
        let split_round_two = split_round_one.unwrap().split("}").next();
        if split_round_two.is_none() {
          return Err(quick_xml::errors::ErrorKind::Msg("No closing bracket, start".to_owned()).into())
        }
        let split_almost_frd = split_round_two.unwrap();
        let mut split_frd_frd = split_almost_frd.split("|");

        let first_element = split_frd_frd.next().unwrap();
        let as_tag = first_element.parse::<ssml_constants::PossibleOpenTags>();
        if as_tag.is_err() {
          continue;
        }
        let mut parsed_out_values = BTreeMap::new();
        loop {
          match split_frd_frd.next() {
            Some(x) => {
              let mut as_split_new = x.split("=");
              let btree_key = as_split_new.next();
              let btree_value = as_split_new.next();
              if btree_key.is_none() || btree_value.is_none() {
                break
              }
              parsed_out_values.insert(btree_key.unwrap().to_owned(), btree_value.unwrap().to_owned());
            },
            None => { break }
          };
        }

        match as_tag.unwrap() {
          ssml_constants::PossibleOpenTags::Break => {
            let mut strength: Option<ssml_constants::BreakStrength> = None;
            let mut time: Option<ssml_constants::BreakTime> = None;
            if parsed_out_values.contains_key("strength") {
              let attempted_parse = parsed_out_values.get("strength").unwrap()
                .parse::<ssml_constants::BreakStrength>();
              if attempted_parse.is_ok() {
                strength = Some(attempted_parse.unwrap());
              }
            }
            if parsed_out_values.contains_key("time") {
              let attempted_parse = parsed_out_values.get("time").unwrap()
                .parse::<ssml_constants::BreakTime>();
              if attempted_parse.is_ok() {
                time = Some(attempted_parse.unwrap());
              }
            }
            try!(xml_writer.ssml_break(strength, time));
          },
          ssml_constants::PossibleOpenTags::LangTag => {
            if !parsed_out_values.contains_key("lang") {
              continue;
            }
            let lang = parsed_out_values.get("lang").unwrap().to_owned();
            let mut onlangfailure: Option<String> = None;
            if parsed_out_values.contains_key("onlangfailure") {
              onlangfailure = Some(parsed_out_values.get("onlangfailure").unwrap().to_owned());
            }
            try!(xml_writer.start_ssml_lang(lang, onlangfailure));
          },
          ssml_constants::PossibleOpenTags::Mark => {
            if !parsed_out_values.contains_key("name") {
              continue;
            }
            let name = parsed_out_values.get("name").unwrap().to_owned();
            try!(xml_writer.start_ssml_mark(name));
          },
          ssml_constants::PossibleOpenTags::Paragraph => {
            try!(xml_writer.start_ssml_paragraph());
          },
          ssml_constants::PossibleOpenTags::Phoneme => {
            if !parsed_out_values.contains_key("alphabet") ||
              !parsed_out_values.contains_key("ph") {
              continue;
            }
            let potential_alphabet = parsed_out_values.get("alphabet").unwrap()
              .parse::<ssml_constants::PhonemeAlphabet>();
            if potential_alphabet.is_err() {
              continue;
            }
            let alphabet = potential_alphabet.unwrap();
            let ph = parsed_out_values.get("ph").unwrap().to_owned();
            try!(xml_writer.start_ssml_phoneme(alphabet, ph));
          },
          ssml_constants::PossibleOpenTags::Prosody => {
            let mut volume: Option<String> = None;
            let mut rate: Option<ssml_constants::ProsodyRate> = None;
            let mut pitch: Option<String> = None;

            if parsed_out_values.contains_key("volume") {
              volume = Some(parsed_out_values.get("volume").unwrap().to_owned());
            }
            if parsed_out_values.contains_key("rate") {
              let potentially_parsed = parsed_out_values.get("rate").unwrap()
                .parse::<ssml_constants::ProsodyRate>();
              if potentially_parsed.is_ok() {
                rate = Some(potentially_parsed.unwrap());
              }
            }
            if parsed_out_values.contains_key("pitch") {
              pitch = Some(parsed_out_values.get("pitch").unwrap().to_owned());
            }

            try!(xml_writer.start_ssml_prosody(volume, rate, pitch));
          },
          ssml_constants::PossibleOpenTags::Sentence => {
            try!(xml_writer.start_ssml_sentence());
          },
          ssml_constants::PossibleOpenTags::SayAs => {
            if !parsed_out_values.contains_key("interpret-as") {
              continue;
            }
            let interpret_as = parsed_out_values.get("interpret-as").unwrap().to_owned();
            try!(xml_writer.start_ssml_say_as(interpret_as));
          },
          ssml_constants::PossibleOpenTags::Sub => {
            if !parsed_out_values.contains_key("alias") {
              continue;
            }
            let alias = parsed_out_values.get("alias").unwrap().to_owned();
            try!(xml_writer.start_ssml_sub(alias));
          },
          ssml_constants::PossibleOpenTags::Word => {
            if !parsed_out_values.contains_key("role") {
              continue;
            }
            let potentially_parsed = parsed_out_values.get("role").unwrap()
              .parse::<ssml_constants::WordRole>();
            if potentially_parsed.is_ok() {
              try!(xml_writer.start_ssml_w(potentially_parsed.unwrap()));
            }
          },
          ssml_constants::PossibleOpenTags::AmazonEffect => {
            if !parsed_out_values.contains_key("name") {
              continue;
            }
            let potentially_parsed = parsed_out_values.get("name").unwrap()
              .parse::<ssml_constants::AmazonEffect>();
            if potentially_parsed.is_ok() {
              try!(xml_writer.start_ssml_amazon_effect(potentially_parsed.unwrap()));
            }
          }
        };
      } else {
        try!(xml_writer.write_text(word));
      }
    }
    try!(xml_writer.end_ssml_paragraph());
  }

  try!(xml_writer.end_ssml_speak());

  Ok(xml_writer.render())
}