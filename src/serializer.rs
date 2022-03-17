#[macro_export]
macro_rules! to_serialized {
    ($content_type:expr, $name:ident, ($($key:ident), *)) => {
    
        match $content_type {

            "application/json" => to_json!($name, ($($key), *)),
            "application/cfg" => to_cfg!($name, ($($key), *)),
            "application/yaml" => to_yaml!($name, ($($key), *)),
            _ => "".to_string()

        }

    }
}

#[macro_export]
macro_rules! to_cfg {
    ($name:ident, ($($key:ident), *)) => {
    
        format!(concat!("", $(
            stringify!($key),
            " = ",
            "{}",
            "\n",
        )*), $( $name.$key, )*)

    }
}

#[macro_export]
macro_rules! to_yaml {
    ($name:ident, ($($key:ident), *)) => {
    
        format!(concat!("", $(
            stringify!($key),
            ": ",
            "{}",
            "\n",
        )*), $( $name.$key, )*)

    }
}

#[macro_export]
macro_rules! to_json {
    ($name:ident, ($($key:ident), *)) => {
        
        // This is by far the ugliest solution in the world. But it works, and its fast as hell, so no one is touching it. 
        // Also, only string types are allowed as output

        format!("{}}}", &format!(concat!(concat!("{{", $(
            "\"",
            stringify!($key),
            "\"",
            ": ",
            "{}",
            ", ",
        )*), "}}"), $( format!("\"{}\"", $name.$key ), )*)[..format!(concat!(concat!("{{", $(
            "\"",
            stringify!($key),
            "\"",
            ": ",
            "{}",
            ", ",
        )*), "}}"), $( format!("\"{}\"", $name.$key ), )*).len() - 3])

    }
}
