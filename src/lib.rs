use std::ffi::CString;

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

pub fn run() {
    // Prints each argument on a separate line
    let mut args = std::env::args();
    // First parameter is program name
    let name = args.next().unwrap();
    let mut command: Vec<CString> = vec![];
    let mut priority: i8 = 10;
    loop {
        let param = args.next();
        if param.is_none() {
            break;
        }
        let param = param.unwrap();
        if param == "--help" {
            println!("{}", HELP_INFO);
            std::process::exit(0);
        } else if param == "--version" {
            println!("{}", VERSION_INFO);
            std::process::exit(0);
        } else if param.starts_with(NICE_SHORT_PARAM) || param.starts_with(NICE_LONG_PARAM) {
            if param == NICE_SHORT_PARAM || param == NICE_LONG_PARAM {
                let param = args.next();
                if param.is_none() {
                    println!("{}", NICE_ERROR);
                    std::process::exit(1);
                }
                let param = param.unwrap();
                priority = param.parse::<i8>().unwrap_or_else(|_| {
                    println!("{}", NICE_ERROR);
                    std::process::exit(1);
                });
            } else {
                if param.starts_with(NICE_SHORT_PARAM) {
                    priority = param[NICE_SHORT_PARAM.len()..]
                        .parse::<i8>()
                        .unwrap_or_else(|_| {
                            println!("{}", NICE_ERROR);
                            std::process::exit(1);
                        });
                } else {
                    priority = param[NICE_LONG_PARAM.len()..]
                        .parse::<i8>()
                        .unwrap_or_else(|_| {
                            println!("{}", NICE_ERROR);
                            std::process::exit(1);
                        });
                }
            }
        } else {
            command.push(CString::new(param).unwrap());
        }
    }

    println!("self name: {}", &name);
    println!("command params: {:?}", &command);
    let mut params: Vec<*const libc::c_char> =
        command[..].iter().map(|item| item.as_ptr()).collect();
    params.push(std::ptr::null());
    let error = unsafe {
        raw::nice(priority as i32);
        libc::execvp(command[0].as_ptr(), params.as_ptr())
    };
    unsafe {
        libc::perror(std::ptr::null());
    }
    println!("something went wrong: {}", error);
}
