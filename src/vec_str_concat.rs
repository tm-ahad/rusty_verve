
pub fn vec_str_concat(vec: Vec<&str>) -> String
{
    let mut r = "".to_string();

    for s in vec.iter()
    {
        r = format!("{}{}", r, s);
    }
    r
}