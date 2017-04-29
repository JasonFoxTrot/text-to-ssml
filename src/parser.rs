use ::errors::*;
use nom::{IResult, rest};
use ::ssml_constants::*;
use ::xml_writer::XmlWriter;
use std::str;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct StartTag {
  pub tag_key: String,
  pub params: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct EndTag {
  pub tag_key: String,
}

#[derive(Clone, Debug)]
pub struct OneItem {
  pub start_tag: Option<StartTag>,
  pub end_tag: Option<EndTag>,
  pub data: Option<String>,
}

named!(string<&str>,
  alt!(map_res!(
    take_until!("${"), str::from_utf8
  ) | map_res!(
    rest, str::from_utf8
  ))
);

named!(start_tag_info<StartTag>,
  map!(
    do_parse!(
      tag!("${") >>
      not!(char!('/')) >>
      key: map_res!(take_until!("}"), str::from_utf8) >>
      tag!("}") >>
      (key)
    ),
    |key: (&str)| {
      if key.contains("|") {
        let mut as_split = key.split("|");
        let tag_key = as_split.next().unwrap().to_owned();
        let mut parsed_out_values = BTreeMap::new();
        loop {
          match as_split.next() {
            Some(x) => {
              let mut as_split_new = x.split("=");
              let btree_key = as_split_new.next();
              let btree_value = as_split_new.next();
              if btree_key.is_none() || btree_value.is_none() {
                break
              }
              parsed_out_values.insert(btree_key.unwrap().to_owned(),
                btree_value.unwrap().to_owned());
            },
            None => { break }
          };
        }
        StartTag {
          tag_key: tag_key,
          params: parsed_out_values,
        }
      } else {
        StartTag {
          tag_key: key.to_owned(),
          params: BTreeMap::new(),
        }
      }
    }
  )
);

named!(end_tag_info<EndTag>,
  map!(
    do_parse!(
      tag!("${/") >>
      key: map_res!(take_until!("}"), str::from_utf8) >>
      tag!("}") >>
      (key)
    ),
    |key_name: &str| {
      EndTag {
        tag_key: key_name.to_owned(),
      }
    }
  )
);

named!(text_to_ssml_parser<Vec<OneItem>>,
  complete!(
    many1!(
      alt!(
        complete!(map!(start_tag_info, |start_tag| {
          OneItem {
            start_tag: Some(start_tag),
            end_tag: None,
            data: None,
          }
        })) | complete!(map!(end_tag_info, |end_tag| {
          OneItem {
            start_tag: None,
            end_tag: Some(end_tag),
            data: None,
          }
        })) | complete!(map!(string, |data: &str| {
          OneItem {
            start_tag: None,
            end_tag: None,
            data: Some(data.to_owned()),
          }
        }))
      )
    )
  )
);

/// Parses some text as SSML. It should note the error here allows for a lot of wiggle room.
/// It's still totally possible to generate invalid SSML with this. This simply does what the
/// user tells it too. If a user doesn't close a tag, we won't close a tag. If they close a
/// tag without opening one we won't close it. If they include a paragraph tag inside a paragraph
/// tag we'll still render it. All of these are invalid SSML, but don't trigger an error.
/// This is meant to be that way as you can try anything with SSML, since polly doesn't fully
/// follow the SSML v1.1 spec, now you can play around as much as you want.
pub fn parse_as_ssml(data: String) -> Result<String> {
  let initial_parse: IResult<&[u8], Vec<OneItem>> = text_to_ssml_parser(data.as_bytes());

  if initial_parse.is_err() {
    return Err(ErrorKind::NomResultError.into())
  } else if initial_parse.is_incomplete() {
    return Err(ErrorKind::NomIncompleteError.into())
  }

  let (_, parsed) = initial_parse.unwrap();

  let mut xml_writer = try!(XmlWriter::new());
  try!(xml_writer.start_ssml_speak(None, None));

  let _ = parsed.into_iter().inspect(|item| {
    if let Some(ref start_tag) = item.start_tag {
      let as_tag = start_tag.tag_key.clone().parse::<PossibleOpenTags>();
      if as_tag.is_err() {
        return;
      }
      let tag_frd = as_tag.unwrap();

      match tag_frd {
        PossibleOpenTags::Break => {
          let mut strength: Option<BreakStrength> = None;
          let mut time: Option<BreakTime> = None;

          if start_tag.params.contains_key("strength") {
            let attempted_parse = start_tag.params.get("strength").unwrap()
              .parse::<BreakStrength>();
            if attempted_parse.is_ok() {
              strength = Some(attempted_parse.unwrap());
            }
          }
          if start_tag.params.contains_key("time") {
            let attempted_parse = start_tag.params.get("time").unwrap()
              .parse::<BreakTime>();
            if attempted_parse.is_ok() {
              time = Some(attempted_parse.unwrap());
            }
          }
          let _ = xml_writer.ssml_break(strength, time);
        },
        PossibleOpenTags::LangTag => {
          if !start_tag.params.contains_key("lang") {
            return;
          }
          let lang = start_tag.params.get("lang").unwrap().to_owned();
          let mut onlangfailure: Option<String> = None;
          if start_tag.params.contains_key("onlangfailure") {
            onlangfailure = Some(start_tag.params.get("onlangfailure").unwrap().to_owned());
          }
          let _ = xml_writer.start_ssml_lang(lang, onlangfailure);
        },
        PossibleOpenTags::Mark => {
          if !start_tag.params.contains_key("name") {
            return;
          }
          let name = start_tag.params.get("name").unwrap().to_owned();
          let _ = xml_writer.start_ssml_mark(name);
        }
        PossibleOpenTags::Paragraph => {
          let _ = xml_writer.start_ssml_paragraph();
        },
        PossibleOpenTags::Phoneme => {
          if !start_tag.params.contains_key("alphabet") ||
            !start_tag.params.contains_key("ph") {
            return;
          }
          let potential_alphabet = start_tag.params.get("alphabet").unwrap()
            .parse::<PhonemeAlphabet>();
          if potential_alphabet.is_err() {
            return;
          }
          let alphabet = potential_alphabet.unwrap();
          let ph = start_tag.params.get("ph").unwrap().to_owned();
          let _ = xml_writer.start_ssml_phoneme(alphabet, ph);
        },
        PossibleOpenTags::Prosody => {
          let mut volume: Option<String> = None;
          let mut rate: Option<ProsodyRate> = None;
          let mut pitch: Option<String> = None;

          if start_tag.params.contains_key("volume") {
            volume = Some(start_tag.params.get("volume").unwrap().to_owned());
          }
          if start_tag.params.contains_key("rate") {
            let potentially_parsed = start_tag.params.get("rate").unwrap()
              .parse::<ProsodyRate>();
            if potentially_parsed.is_ok() {
              rate = Some(potentially_parsed.unwrap());
            }
          }
          if start_tag.params.contains_key("pitch") {
            pitch = Some(start_tag.params.get("pitch").unwrap().to_owned());
          }

          let _ = xml_writer.start_ssml_prosody(volume, rate, pitch);
        },
        PossibleOpenTags::Sentence => {
          let _ = xml_writer.start_ssml_sentence();
        },
        PossibleOpenTags::SayAs => {
          if !start_tag.params.contains_key("interpret-as") {
            return;
          }
          let interpret_as = start_tag.params.get("interpret-as").unwrap().to_owned();
          let _ = xml_writer.start_ssml_say_as(interpret_as);
        },
        PossibleOpenTags::Sub => {
          if !start_tag.params.contains_key("alias") {
            return;
          }
          let alias = start_tag.params.get("alias").unwrap().to_owned();
          let _ = xml_writer.start_ssml_sub(alias);
        },
        PossibleOpenTags::Word => {
          if !start_tag.params.contains_key("role") {
            return;
          }
          let potentially_parsed = start_tag.params.get("role").unwrap()
            .parse::<WordRole>();
          if potentially_parsed.is_ok() {
            let _ = xml_writer.start_ssml_w(potentially_parsed.unwrap());
          }
        },
        PossibleOpenTags::AmazonEffect => {
          if !start_tag.params.contains_key("name") {
            return;
          }
          let potentially_parsed = start_tag.params.get("name").unwrap()
            .parse::<AmazonEffect>();
          if potentially_parsed.is_ok() {
            let _ = xml_writer.start_ssml_amazon_effect(potentially_parsed.unwrap());
          }
        }
      };
    };

    if let Some(ref end_tag) = item.end_tag {
      let as_tag = end_tag.tag_key.clone().parse::<PossibleClosingTags>();
      if as_tag.is_err() {
        return
      }
      let tag_frd = as_tag.unwrap();

      let _ = match tag_frd {
        PossibleClosingTags::LangTag => xml_writer.end_ssml_lang(),
        PossibleClosingTags::Mark => xml_writer.end_ssml_mark(),
        PossibleClosingTags::Paragraph => xml_writer.end_ssml_paragraph(),
        PossibleClosingTags::Phoneme => xml_writer.end_ssml_phoneme(),
        PossibleClosingTags::Prosody => xml_writer.end_ssml_prosody(),
        PossibleClosingTags::Sentence => xml_writer.end_ssml_sentence(),
        PossibleClosingTags::SayAs => xml_writer.end_ssml_say_as(),
        PossibleClosingTags::Sub => xml_writer.end_ssml_sub(),
        PossibleClosingTags::Word => xml_writer.end_ssml_w(),
        PossibleClosingTags::AmazonEffect => xml_writer.end_ssml_amazon_effect()
      };
    };

    if let Some(ref data) = item.data {
      let _ = xml_writer.write_text(data.replace("$\\{", "${").as_str());
    }
  }).count();

  try!(xml_writer.end_ssml_speak());

  Ok(xml_writer.render())
}