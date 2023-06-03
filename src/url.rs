use std::{string::ParseError, str::FromStr, collections::HashMap };

#[derive(Debug, Clone)]
pub struct Url {
    value: String,
    pub protocol: Protocol,
    pub base_url: String,
    pub host: String,
    pub routes: Vec<String>,
    params: String,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    HTTPS,
    HTTP
}

impl Url {
    pub fn params(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        let params_as_string = self.params.split("?").last().unwrap_or("");
        params_as_string
            .split("&")
            .for_each(|param| -> (){
                let splits = param.split("=")
                    .take(2)
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                hash.insert(splits[0].to_string(), splits[1].to_string());
            });

        return hash;
    }
}

impl FromStr for Protocol {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "https" => Ok(Protocol::HTTPS),
            "http" => Ok(Protocol::HTTP),
            _ => panic!("Could not parse Protocol")
            
        }
    }
}



// https://ailo.atlassian.net/jira/software/c/projects/L3S/boards/150?modal=detail&selectedIssue=L3S-3430
impl FromStr for Url {
    type Err = ParseError;

    fn from_str(url_string: &str) -> Result<Self, Self::Err> {

        let val = url_string.split("://").take(2).collect::<Vec<&str>>();
        let protocol: Protocol = val[0].parse::<Protocol>()?;
        let split =  val[1]
            .split("/")
            .collect::<Vec<&str>>();

        let host = split
            .first()
            .expect("No elements in the list")
            .to_string();

        let mut routes = split
            .iter()
            .skip(1)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let params = routes
            .pop()
            .expect("No host to pop off list")
            .to_string();

        return Ok(Url { 
            base_url: url_string.to_owned(),
            protocol,
            host,
            value: url_string.to_owned(),
            routes,
            params
        });
    }
}

