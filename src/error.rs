use std::process::exit;
use colored::Colorize;

pub struct VeltError<T>
where 
    T: Fn() 
{
    pub c: T,
    pub line: u32,
    pub err: String
}

impl<T> VeltError<T>
where
    T: Fn()
{

    pub fn new (line: u32, err: String, on_render: T) -> Self
    {
        Self
        {
           line,
           err,
           c: on_render
        }
    }

    pub fn exec (&self)
    {
        let e = format!("{}: {}", "ERROR".red().bold(),
                       format!("{} {}", self.err, format!("{} {}", "at line".red(), self.line)).red());
        println!("{}", e);

        (self.c)();

        exit(1);
    }

    // pub fn create ( &self, line: u32, error: String )
    // {
    //     let e = format!("{}: {}", "ERROR".red().bold(),
    //                     format!("{} {}", error, format!("{} {}", "at line".red(), line)).red());
    //     println!("{}", e);
    //
    //     (self.c)();
    //     exit(1);
    // }

    pub fn inst(idx: u32, line: &str, s: &str) -> String
    {
        let mut f = "";
        let mut e = "";

        match line.find(s)
        {
            Some(idx) =>
                {
                    f = &line[0..idx];
                    e = &line[(idx + s.len())..line.len()-1]
                }
            None => {}
        }

        let s = format!("{}", format!("{}",
              format!("{}{}{}{}{}", idx.to_string().blue().bold(), "|".blue(), f, s.bold().red(), e)));
        return s.clone();
    }
}