extern crate text_to_polly_ssml;

#[test]
fn test_simple_parsing() {
  let result = text_to_polly_ssml::parse_string(r#"${amazon:effect|name=whisper}test${/amazon:effect}"#.to_owned());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), r#"<?xml version="1.0"?><speak xml:lang="en-US" onlangfailure="processorchoice" xmlns="http://www.w3.org/2001/10/synthesis" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><amazon:effect name="whispered">test</amazon:effect></speak>"#);
}

#[test]
fn test_only_text_parsing() {
  let result = text_to_polly_ssml::parse_string(r#"hey world"#.to_owned());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), r#"<?xml version="1.0"?><speak xml:lang="en-US" onlangfailure="processorchoice" xmlns="http://www.w3.org/2001/10/synthesis" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">hey world</speak>"#);
}

#[test]
fn test_complex_parsing() {
  let result = text_to_polly_ssml::parse_string(r#"Hello, My name is justin.
I'm going to stop talking for a bit. ${break} now even longer... ${break|strength=strong|time=4s}
I'm going to switch my language. ${lang|lang=fr_FR} hey ${/lang}, now with an optional fallback: ${lang|lang=fr_FR|onlangfailure=changevoice} ${/lang}
How about a mark? ${mark|name=markName} a name ${/mark}.
How about my own paragraph? ${p} test ${/p}
How about a phoneme? ${phoneme|alphabet=ipa|ph=pɪˈkɑːn} pecan ${/phoneme}
Now lets go to Prosody. ${prosody|volume=+6dB} loud ${/prosody} Now even more ${prosody|volume=+6db|rate=x-fast|pitch=+4%} coffee ${/prosody}
Now lets go to a sentence. ${s} some words. ${/s}
Now lets go to say-as: ${say-as|interpret-as=spell-out} abc ${/say-as}.
What about a Sub? ${sub|alias=mercury} hg ${/sub}
What aboue a word role? ${w|role=amazon:VB} test ${/w}
What about whisper? ${amazon:effect|name=whisper} this is a secret to everyone ${/amazon:effect}
What about some DRC? ${amazon:effect|name=drc}This text has a higher pitch than normal.${/amazon:effect}
What about some Vocal Tract Length? ${amazon:effect|vocal-tract-length=+10%}Yo.${/amazon:effect}"#.to_owned());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), r#"<?xml version="1.0"?><speak xml:lang="en-US" onlangfailure="processorchoice" xmlns="http://www.w3.org/2001/10/synthesis" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">Hello, My name is justin.
I&apos;m going to stop talking for a bit. <break/> now even longer... <break strength="strong" time="4s"/>
I&apos;m going to switch my language. <lang xml:lang="fr_FR" onlangfailure="processorchoice"> hey </lang>, now with an optional fallback: <lang xml:lang="fr_FR" onlangfailure="changevoice"> </lang>
How about a mark? <mark name="markName"> a name </mark>.
How about my own paragraph? <p> test </p>
How about a phoneme? <phoneme alphabet="ipa" ph="pɪˈkɑːn"> pecan </phoneme>
Now lets go to Prosody. <prosody volume="+6dB"> loud </prosody> Now even more <prosody volume="+6db" rate="x-fast" pitch="+4%"> coffee </prosody>
Now lets go to a sentence. <s> some words. </s>
Now lets go to say-as: <say-as interpret-as="spell-out"> abc </say-as>.
What about a Sub? <sub alias="mercury"> hg </sub>
What aboue a word role? <w role="amazon:VB"> test </w>
What about whisper? <amazon:effect name="whispered"> this is a secret to everyone </amazon:effect>
What about some DRC? <amazon:effect name="drc">This text has a higher pitch than normal.</amazon:effect>
What about some Vocal Tract Length? <amazon:effect vocal-tract-length="+10%">Yo.</amazon:effect></speak>"#);
}
