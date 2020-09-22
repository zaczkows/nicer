mod platform;
#[cfg(unix)]
mod raw;

const HELP_INFO: &str = r"
Usage: nice [OPTION] [COMMAND [ARG]...]
Run COMMAND with an adjusted niceness, which affects process scheduling.
With no COMMAND, print the current niceness.  Niceness values range from
-20 (most favorable to the process) to 19 (least favorable to the process).

Mandatory arguments to long options are mandatory for short options too.
  -n, --adjustment=N   add integer N to the niceness (default 10)
      --help     display this help and exit
      --version  output version information and exit

NOTE: your shell may have its own version of nice, which usually supersedes
the version described here.  Please refer to your shell's documentation
for details about the options it supports.
";

const VERSION_INFO: &str = r"
Rust replacement for `nice` command.
";

const NICE_ERROR: &str = r"
nice: option requires an argument -- 'n'
Try 'nice --help' for more information.
";

const NICE_SHORT_PARAM: &str = "-n";
const NICE_LONG_PARAM: &str = "--adjustment";

pub struct ParsedParams {
    pub command: Vec<String>,
    pub priority: i8,
}

pub struct ParseError {
    pub message: &'static str,
}

impl std::convert::From<std::num::ParseIntError> for ParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self {
            message: NICE_ERROR,
        }
    }
}

impl std::convert::From<&'static str> for ParseError {
    fn from(message: &'static str) -> Self {
        Self { message }
    }
}

impl ParseError {
    fn new(message: &'static str) -> Self {
        Self { message }
    }
}

pub fn process_args_internal<T>(args: T) -> Result<ParsedParams, ParseError>
where
    T: std::iter::IntoIterator,
    <T as std::iter::IntoIterator>::Item: ToString,
{
    // First parameter is program name
    let mut args = args.into_iter();
    let name: String = args.next().unwrap().to_string();
    let mut command: Vec<String> = vec![];
    let mut priority: i8 = 10;
    loop {
        let param = args.next();
        if param.is_none() {
            return Ok(ParsedParams { command, priority });
        }
        let param: String = param.unwrap().to_string();
        if param == "--" {
            break;
        } else if param == "--help" {
            return Err(std::convert::From::from(HELP_INFO));
        } else if param == "--version" {
            return Err(ParseError::new(VERSION_INFO));
        } else if param.starts_with(NICE_SHORT_PARAM) || param.starts_with(NICE_LONG_PARAM) {
            if param == NICE_SHORT_PARAM || param == NICE_LONG_PARAM {
                let param = args.next();
                if param.is_none() {
                    return Err(ParseError::new(NICE_ERROR));
                }
                let param = param.unwrap().to_string();
                priority = param.parse::<i8>()?;
            } else if param.starts_with(NICE_SHORT_PARAM) {
                priority = param[NICE_SHORT_PARAM.len()..].parse::<i8>()?;
            } else {
                priority = param[NICE_LONG_PARAM.len()..].parse::<i8>()?;
            }
        } else {
            command.push(param);
            break;
        }
    }

    for param in args {
        command.push(param.to_string());
    }
    println!("self name: {}", &name);
    println!("command params: {:?}", &command);
    Ok(ParsedParams { command, priority })
}

pub fn process_args<T>(args: T) -> Result<ParsedParams, ParseError>
where
    T: std::iter::IntoIterator,
    <T as std::iter::IntoIterator>::Item: ToString,
{
    let result = process_args_internal(args);
    if result.is_ok() && result.as_ref().ok().unwrap().command.is_empty() {
        return Err(ParseError::new(HELP_INFO));
    }
    result
}

pub fn run(args: &ParsedParams) {
    // Prints each argument on a separate line
    platform::set_priority(args.priority);
    if !platform::exec_cmd(&args.command) {
        println!("something went wrong");
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_empty_params() {
        let args = vec![""];
        let params = process_args(args);
        assert!(params.is_err());
    }

    #[test]
    fn test_no_command() {
        {
            let args = vec!["nice", "--help"];
            let params = process_args(args);
            assert!(params.is_err());
        }
        {
            let args = vec!["nice", "-n 10"];
            let params = process_args(args);
            assert!(params.is_err());
        }
        {
            let args = vec!["nice", "--"];
            let params = process_args(args);
            assert!(params.is_err());
        }
    }

    #[test]
    fn test_command() {
        {
            let args = vec!["nice", "cmd", "--help"];
            let params = process_args(args);
            assert!(params.is_ok());
            assert_eq!(params.as_ref().ok().unwrap().command.len(), 2);
            assert_eq!(params.as_ref().ok().unwrap().command, vec!["cmd", "--help"]);
        }
        {
            let args = vec!["nice", "-n", "4", "cmd", "--help"];
            let params = process_args(args);
            assert!(params.is_ok());
            assert_eq!(params.as_ref().ok().unwrap().command.len(), 2);
            assert_eq!(params.as_ref().ok().unwrap().command, vec!["cmd", "--help"]);
            assert_eq!(params.as_ref().ok().unwrap().priority, 4);
        }
    }
}
