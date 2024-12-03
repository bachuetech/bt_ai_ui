pub fn get_first_of_split(s: &str, separator: &str) -> (String, String){
    if let Some(position) = s.find(separator){
        let str1 = s[..position].to_owned();
        let str2 = s[position + 1..].to_owned();
        (str1, str2)
    }else{
        (s.to_owned(),"".to_owned())
    }
}

pub fn get_first_ocurrance(s: &str, separator: &str) -> String{
    if let Some(position) = s.find(separator){
        s[..position].to_owned()
    }else{
        "".to_owned()
    }
}