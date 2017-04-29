# Text to Polly SSML #


| OS      | Build Status                                                                                                                                                          |
|:--------|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Linux   | [![Linux Build Status](https://circleci.com/gh/SecurityInsanity/text-to-polly-ssml/tree/master.svg?style=svg)](https://circleci.com/gh/SecurityInsanity/text-to-polly-ssml/tree/master) |

A Library to turn Text into Valid "Polly SSML". Note I say Polly SSML HEre, since the goal for this is
to be eventually sent to AWS Polly. AWS Polly does not implement the full SSML v1.1 Spec. It implements
a subset of it, and as such that is the subset we support. E.g. if you can't do it in polly, you can't do it
here.

## Usage ##

Simply import the library as a crate, and call parse_string:

```rust
extern crate text_to_polly_ssml;

fn main() {
  let result = text_to_polly_ssml::parse_string("my string".to_owned());
  assert!(result.is_ok());
  let ssml = result.unwrap();
}
```


## License ##

This library is licensed under MIT.