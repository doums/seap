pub mod cli_parser {

    #[derive(Debug)]
    pub struct Parser {
        args: std::env::Args,
        flags: Vec<Flag>,
    }

    #[derive(Debug)]
    pub struct Flag(&'static str, char, &'static str, bool);

    #[derive(Debug)]
    pub struct Token(&'static str, Option<String>);

    fn find_option<'a>(
        options: &'a [Flag],
        short: Option<char>,
        long: Option<&'a str>,
    ) -> Result<&'a Flag, &'static str> {
        for option in options {
            if let Some(i) = short {
                if i == option.1 {
                    return Ok(option);
                }
            }
            if let Some(i) = long {
                if i == option.2 {
                    return Ok(option);
                }
            }
        }
        Err("unknown option")
    }

    fn parse_opt(
        args: &mut std::env::Args,
        options: &[Flag],
        arg: &str,
        tokens: &mut Vec<Token>,
    ) -> Result<(), String> {
        let current_arg = &arg[1..];
        for (i, c) in current_arg.char_indices() {
            let option = find_option(options, Some(c), None)
                .map_err(|_| format!("flag unknown \"{}\"", c))?;
            if option.3 == true {
                let arg_opt = &current_arg[i + 1..];
                if arg_opt.len() > 0 {
                    tokens.push(Token(option.0, Some(String::from(arg_opt))));
                    break;
                } else {
                    match args.next() {
                        Some(str) => {
                            tokens.push(Token(option.0, Some(String::from(str))));
                            return Ok(());
                        }
                        None => return Err(format!("missing value for flag \"{}\"", option.0)),
                    }
                }
            } else {
                tokens.push(Token(option.0, None));
            }
        }
        Ok(())
    }

    fn parse_long_opt(
        args: &mut std::env::Args,
        options: &[Flag],
        arg: &str,
        tokens: &mut Vec<Token>,
    ) -> Result<(), String> {
        let current_arg = &arg[2..];
        match current_arg.find("=") {
            None => {
                let option = find_option(options, None, Some(&current_arg))
                    .map_err(|_| format!("flag unknown \"{}\"", current_arg))?;
                if option.3 == true {
                    match args.next() {
                        Some(str) => {
                            tokens.push(Token(option.0, Some(String::from(str))));
                            return Ok(());
                        }
                        None => return Err(format!("missing value for flag \"{}\"", option.0)),
                    }
                } else {
                    tokens.push(Token(option.0, None));
                }
            }
            Some(i) => {
                let first = &current_arg[..i];
                let last = &current_arg[i + 1..];
                if first.len() == 0 || last.len() == 0 {
                    return Err(format!("bad syntax for flag \"{}\"", current_arg));
                }
                let option = find_option(options, None, Some(first))
                    .map_err(|_| format!("flag unknown \"{}\"", first))?;
                if option.3 == true {
                    tokens.push(Token(option.0, Some(String::from(last))));
                } else {
                    return Err(format!("bad syntax for flag \"{}\"", current_arg));
                }
            }
        }
        Ok(())
    }

    impl Parser {
        pub fn new(args: std::env::Args) -> Parser {
            Parser {
                args: args,
                flags: Vec::new(),
            }
        }

        pub fn flag(
            &mut self,
            name: &'static str,
            short: char,
            long: &'static str,
            takes_value: bool,
        ) -> &mut Parser {
            self.flags.push(Flag(name, short, long, takes_value));
            self
        }

        pub fn help(&mut self) -> &mut Parser {
            self.flags.push(Flag("help", 'h', "help", false));
            self
        }

        pub fn verbose(&mut self) -> &mut Parser {
            self.flags.push(Flag("verbose", 'V', "verbose", false));
            self
        }

        pub fn version(&mut self) -> &mut Parser {
            self.flags.push(Flag("version", 'v', "version", false));
            self
        }

        pub fn debug(&mut self) -> &mut Parser {
            self.flags.push(Flag("debug", 'd', "debug", false));
            self
        }

        pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
            let mut tokens: Vec<Token> = vec![];
            let mut accept_opt = true;
            self.args.next(); //skip the binary
            while let Some(arg) = self.args.next() {
                if arg.len() == 2 && &arg[..2] == "--" {
                    accept_opt = false;
                } else if arg.len() > 2 && &arg[..2] == "--" && accept_opt == true {
                    parse_long_opt(&mut self.args, &self.flags, &arg, &mut tokens)?;
                } else if arg.len() > 1 && &arg[..1] == "-" && accept_opt == true {
                    parse_opt(&mut self.args, &self.flags, &arg, &mut tokens)?;
                } else {
                    tokens.push(Token("arg", Some(String::from(arg))));
                }
            }
            Ok(tokens)
        }
    }
}
