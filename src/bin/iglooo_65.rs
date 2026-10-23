use std::fs;

pub trait Source {
    // The third rule is that, if there are multiple input lifetime parameters,
    // but one of them is &self or &mut self because this is a method,
    // the lifetime of self is assigned to all output lifetime parameters

    fn one_method(&self, key: &str) -> Option<&str>;
}

struct Config {
    text: String,
}

impl Config {
    fn new(path: &str) -> Self {
        let content = fs::read_to_string(path);
        Self {
            text: content.unwrap(),
        }
    }

    // method needs to return a lifetime but it says it will return of self but the thing is what
    // if the method returns the dafault then that won't be good so we need to define the lifetimes
    // for both and key doesn't require any lifetime because it is being used for comaprison only
    // like the case was in the trait it was returning the slice of self so we didn't need the
    // lifetime but here the situtation is a bit diffrent either it can be slice of self or default
    fn fixed_callback<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        let result = self.one_method(key);
        result.unwrap_or(default)
    }

    // method might do |s| s returns the same &str it receives,i
    // so the returned reference borrows from key and must not outlive it.

    // We need FnOnce because unwrap_or_else runs it's closure atmost once so we can use FnOnce
    fn lazy_fallback<'a, T>(&'a self, key: &'a str, f: T) -> &'a str
    where
        T: FnOnce(&str) -> &str,
    {
        let some_result = self.one_method(key);

        some_result.unwrap_or_else(|| f(key))
        // match some_result {
        // Some(value) => value,
        // None => f(key),
        // }
    }
}

impl Source for Config {
    // the returned values is coming from self, It's the slice of self
    fn one_method(&self, key: &str) -> Option<&str> {
        self.text
            .lines()
            .filter_map(|s| s.split_once('='))
            .find(|(k, _)| *k == key)
            .map(|(_, value)| value)
    }
}

fn one_free_function<T>(path: &T) -> u16
where
    T: Source,
{
    path.one_method("port")
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}

fn main() {
    let config_struct = Config::new("app.conf");
    println!("{}", config_struct.text);

    println!("{}", config_struct.one_method("port").unwrap());

    println!("{}", config_struct.fixed_callback("boiled", "potatos"));
    println!("{}", config_struct.fixed_callback("host", "potatos"));

    // It borrows the config_struct instead of owning that's why we are able to use it later in the
    // function as well
    println!("{}", one_free_function(&config_struct));

    println!("{}", one_free_function(&Config::new("bad_port.conf")));
    println!("{}", one_free_function(&Config::new("no_port.conf")));

    println!("{}", config_struct.text);

    // let result;
    let result_1;

    // this is going to fail because default_value does not live long enough in this the case the
    // thing is result might point at something which isn't there.

    // {
    // let default_value = String::from("hello");
    // result = config_struct.fixed_callback("boiled", default_value.as_str());
    // }
    // println!("{}", result);

    // this is going to work because we got no beef with key. so the result points at the deafult
    // value we ain't doing with key it is there for moral support (matching, not used for return a
    // slice of it)
    {
        let key = String::from("Hello");
        result_1 = config_struct.fixed_callback(key.as_str(), "potatos");
    }
    println!("{}", result_1);

    println!("{}", config_struct.lazy_fallback("boiled", |_s| "potatos"));
    println!("{}", config_struct.lazy_fallback("port", |_s| "potatos"));
    println!("{}", config_struct.lazy_fallback("boiled", |s| s));
}
