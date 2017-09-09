//! Contains SSML Constants. Things like all possible Strength values for the Break Tag.
//! This is meant to be internal, so you should probably never interact with this directly.

use std::fmt;
use std::str::FromStr;

/// Denotes the potential values for the Strength of a Break tag.
/// These values are straight out of the SSML 1.1 W3C Standard which can be found
/// [HERE](https://www.w3.org/TR/2010/REC-speech-synthesis11-20100907/#edef_break),
/// and what they actually do in polly is documented:
/// [HERE](http://docs.aws.amazon.com/polly/latest/dg/supported-ssml.html#break-tag).
pub enum BreakStrength {
  NoStrength,
  XWeak,
  Weak,
  Medium,
  Strong,
  XStrong,
}

impl fmt::Display for BreakStrength {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &BreakStrength::NoStrength => write!(f, "none"),
      &BreakStrength::XWeak => write!(f, "x-weak"),
      &BreakStrength::Weak => write!(f, "weak"),
      &BreakStrength::Medium => write!(f, "medium"),
      &BreakStrength::Strong => write!(f, "strong"),
      &BreakStrength::XStrong => write!(f, "x-strong"),
    }
  }
}

impl FromStr for BreakStrength {
    type Err = ();

    fn from_str(s: &str) -> Result<BreakStrength, ()> {
      match &*s.to_lowercase() {
        "break" => Ok(BreakStrength::NoStrength),
        "x-weak" => Ok(BreakStrength::XWeak),
        "weak" => Ok(BreakStrength::Weak),
        "medium" => Ok(BreakStrength::Medium),
        "strong" => Ok(BreakStrength::Strong),
        "x-strong" => Ok(BreakStrength::XStrong),
        _ => Err(()),
      }
    }
}

/// Denotes the potential amount of time to Break inside the Break Tag.
/// These values are straight out of the SSML 1.1 W3C Standard which can be found
/// [HERE](https://www.w3.org/TR/2010/REC-speech-synthesis11-20100907/#edef_break)
/// and what they actually do in polly is documented:
/// [HERE](http://docs.aws.amazon.com/polly/latest/dg/supported-ssml.html#break-tag).
pub struct BreakTime {
  /// The actual value of time to break for.
  pub time: u32,
  /// If the break time is in seconds. If this is set to false it is believed to be in
  /// milliseconds.
  pub is_seconds: bool,
}

impl BreakTime {

  /// Constructs a new Break Time.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use text_to_polly_ssml::ssml_constants::BreakTime;
  /// let break_time = BreakTime::new(10, true);
  /// let other_break_time = BreakTime::new(5, false);
  /// ```
  pub fn new(value: u32, is_seconds: bool) -> BreakTime {
    BreakTime {
      time: value,
      is_seconds: is_seconds,
    }
  }

}

impl fmt::Display for BreakTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.time, if self.is_seconds { "s" } else { "ms" })
  }
}

impl FromStr for BreakTime {
    type Err = ();

    fn from_str(s: &str) -> Result<BreakTime, ()> {
      if s.ends_with("ms") && s != "ms" {
        let mut as_split = s.split("ms");
        let potential_number = as_split.next().unwrap();
        let as_num = potential_number.parse::<u32>();
        if as_num.is_ok() {
          return Ok(BreakTime::new(as_num.unwrap(), false))
        }
      } else if s.ends_with("s") && s != "s" {
        let mut as_split = s.split("s");
        let potential_number = as_split.next().unwrap();
        let as_num = potential_number.parse::<u32>();
        if as_num.is_ok() {
          return Ok(BreakTime::new(as_num.unwrap(), true))
        }
      }
      return Err(())
    }
}

/// Represents all phoneme alphabets that AWS Polly Supports.
/// Documentation on supported alphabets can be found under description of the phoneme
/// tags on AWS Polly. Those are located:
/// [HERE](http://docs.aws.amazon.com/polly/latest/dg/supported-ssml.html#phoneme-tag)
pub enum PhonemeAlphabet {
  Ipa,
  XSampa,
}

impl fmt::Display for PhonemeAlphabet {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &PhonemeAlphabet::Ipa => write!(f, "ipa"),
      &PhonemeAlphabet::XSampa => write!(f, "x-sampa"),
    }
  }
}

impl FromStr for PhonemeAlphabet {
    type Err = ();

    fn from_str(s: &str) -> Result<PhonemeAlphabet, ()> {
      match &*s.to_lowercase() {
        "ipa" => Ok(PhonemeAlphabet::Ipa),
        "x-sampa" => Ok(PhonemeAlphabet::XSampa),
        _ => Err(()),
      }
    }
}

/// Represents all possible ProsodyRate rates that AWS Polly Supports.
/// The full documentation on all possible rates are found in AWS Documentation:
/// [HERE](http://docs.aws.amazon.com/polly/latest/dg/supported-ssml.html#prosody-tag)
pub enum ProsodyRate {
  XSlow,
  Slow,
  Medium,
  Fast,
  XFast,
}

impl fmt::Display for ProsodyRate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &ProsodyRate::XSlow => write!(f, "x-slow"),
      &ProsodyRate::Slow => write!(f, "slow"),
      &ProsodyRate::Medium => write!(f, "medium"),
      &ProsodyRate::Fast => write!(f, "fast"),
      &ProsodyRate::XFast => write!(f, "x-fast"),
    }
  }
}

impl FromStr for ProsodyRate {
    type Err = ();

    fn from_str(s: &str) -> Result<ProsodyRate, ()> {
      match &*s.to_lowercase() {
        "x-slow" => Ok(ProsodyRate::XSlow),
        "slow" => Ok(ProsodyRate::Slow),
        "medium" => Ok(ProsodyRate::Medium),
        "fast" => Ok(ProsodyRate::Fast),
        "x-fast" => Ok(ProsodyRate::XFast),
        _ => Err(()),
      }
    }
}

/// Represents all possible WorldRoles that AWS Polly Supports.
/// The full documentation on all possible world roles are found in AWS docs:
/// [HERE](http://docs.aws.amazon.com/polly/latest/dg/supported-ssml.html#w-tag)
pub enum WordRole {
  Verb,
  PastTense,
  PresentTense,
}

impl fmt::Display for WordRole {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &WordRole::Verb => write!(f, "amazon:VB"),
      &WordRole::PastTense => write!(f, "amazon:VBD"),
      &WordRole::PresentTense => write!(f, "amazon:SENSE_1"),
    }
  }
}

impl FromStr for WordRole {
    type Err = ();

    fn from_str(s: &str) -> Result<WordRole, ()> {
      match &*s.to_lowercase() {
        "amazon:vb" => Ok(WordRole::Verb),
        "amazon:vbd" => Ok(WordRole::PastTense),
        "amazon:sense_1" => Ok(WordRole::PresentTense),
        _ => Err(()),
      }
    }
}

/// Represents all possible AWS Effects that AWS Polly Supports.
/// The full documentation on all possible amazon effects are in the AWS docs:
/// [HERE](http://docs.aws.amazon.com/polly/latest/dg/supported-ssml.html).
pub enum AmazonEffect {
  Whispered,
  Drc,
}

impl fmt::Display for AmazonEffect {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &AmazonEffect::Whispered => write!(f, "whispered"),
      &AmazonEffect::Drc => write!(f, "drc"),
    }
  }
}

impl FromStr for AmazonEffect {
    type Err = ();

    fn from_str(s: &str) -> Result<AmazonEffect, ()> {
      match &*s.to_lowercase() {
        "whispered" | "whisper" => Ok(AmazonEffect::Whispered),
        "drc" => Ok(AmazonEffect::Drc),
        _ => Err(()),
      }
    }
}

pub enum PossibleClosingTags {
  LangTag,
  Mark,
  Paragraph,
  Phoneme,
  Prosody,
  Sentence,
  SayAs,
  Sub,
  Word,
  AmazonEffect
}

impl FromStr for PossibleClosingTags {
    type Err = ();

    fn from_str(s: &str) -> Result<PossibleClosingTags, ()> {
      match &*s.to_lowercase() {
        "lang" => Ok(PossibleClosingTags::LangTag),
        "mark" => Ok(PossibleClosingTags::Mark),
        "p" => Ok(PossibleClosingTags::Paragraph),
        "phoneme" => Ok(PossibleClosingTags::Phoneme),
        "prosody" => Ok(PossibleClosingTags::Prosody),
        "s" => Ok(PossibleClosingTags::Sentence),
        "say-as" => Ok(PossibleClosingTags::SayAs),
        "sub" => Ok(PossibleClosingTags::Sub),
        "w" => Ok(PossibleClosingTags::Word),
        "amazon:effect" => Ok(PossibleClosingTags::AmazonEffect),
        _ => Err(()),
      }
    }
}

pub enum PossibleOpenTags {
  Break,
  LangTag,
  Mark,
  Paragraph,
  Phoneme,
  Prosody,
  Sentence,
  SayAs,
  Sub,
  Word,
  AmazonEffect
}

impl FromStr for PossibleOpenTags {
    type Err = ();

    fn from_str(s: &str) -> Result<PossibleOpenTags, ()> {
      match &*s.to_lowercase() {
        "break" => Ok(PossibleOpenTags::Break),
        "lang" => Ok(PossibleOpenTags::LangTag),
        "mark" => Ok(PossibleOpenTags::Mark),
        "p" => Ok(PossibleOpenTags::Paragraph),
        "phoneme" => Ok(PossibleOpenTags::Phoneme),
        "prosody" => Ok(PossibleOpenTags::Prosody),
        "s" => Ok(PossibleOpenTags::Sentence),
        "say-as" => Ok(PossibleOpenTags::SayAs),
        "sub" => Ok(PossibleOpenTags::Sub),
        "w" => Ok(PossibleOpenTags::Word),
        "amazon:effect" => Ok(PossibleOpenTags::AmazonEffect),
        _ => Err(()),
      }
    }
}
