use crate::error::VeltError;

pub fn unknown_key(keys: Vec<&str>, v: String, _line: u32, line: &str)
{

    for key in keys
    {

        match v.find(key)
        {
            Some(_) =>
            {
                let e = VeltError::new(_line, format!("{} {}", "unknown keyword", key), || {
                    println!("{}", VeltError::<fn()>::inst(_line, line, "let"));
                });

                e.exec();
            },
            _ => {}
        }
    }
}