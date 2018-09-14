use std::str::Chars;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid header")]
    InvalidHeader,
}

pub enum Part {
    Buf(String),
    Input(String),
}

pub struct Template {
    name: Vec<String>,
    output: String,
    parts: Vec<Part>,
}

impl Template {
    pub fn new(name: Vec<String>, output: String, parts: Vec<Part>) -> Self {
        Template {
            name,
            output,
            parts,
        }
    }
}

pub struct TemplateBuilder {
    name: Option<Vec<String>>,
    output: Option<String>,
    parts: Vec<Part>,
}

impl TemplateBuilder {
    pub fn new() -> Self {
        TemplateBuilder {
            name: None,
            output: None,
            parts: Vec::new(),
        }
    }

    fn read_header(&mut self, buf: &str) -> Result<(), Error> {
        let lines = buf.trim().split('\n').collect::<Vec<&str>>();
        for line in lines {
            let kv = line.splitn(2, ':').collect::<Vec<&str>>();
            let key = kv[0].trim();
            let value = kv[1].trim();
            if key == "name" {
                self.name = Some(value.split(',').map(|s| s.to_string()).collect());
            } else if key == "output" {
                self.output = Some(value.to_owned());
            } else {
                return Err(Error::InvalidHeader);
            }
        }
        return Ok(());
    }

    pub fn parse(&mut self, buf: &str) -> Result<(), Error> {
        let splitted = buf.splitn(3, "---").collect::<Vec<&str>>();
        if splitted.len() != 3 {
            return Err(Error::InvalidHeader);
        }
        self.read_header(splitted[1])?;
        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn header() {
        let mut tb = TemplateBuilder::new();
        tb.parse("---\nname: myname\noutput: output\n---\n")
            .unwrap();
        assert_eq!(tb.name, Some(vec!["myname".to_owned()]));
        assert_eq!(tb.output, Some("output".to_owned()));
    }
}
