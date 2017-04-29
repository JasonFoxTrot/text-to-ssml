use quick_xml;
use std::str;

error_chain! {

  links {
    QuickXml(quick_xml::errors::Error, quick_xml::errors::ErrorKind);
  }

  foreign_links {
    Utf8Error(str::Utf8Error);
  }

  errors {
    NomResultError {
      description("There was a nom result error")
      display("Nom Result Error")
    }

    NomIncompleteError {
      description("Nom Incomplete Error")
      display("Nom did not finish parsing. usually this means the parser broke")
    }
  }

}