use std::borrow::Cow;

use plist::Dictionary;

#[inline]
fn read_key<'a>(dict: &'a Dictionary, key: &str) -> Option<&'a str> {
    dict.get(key).and_then(|v| v.as_string())
}

pub struct App {
    dict: Dictionary,
}

impl App {
    pub(crate) fn new(dict: Dictionary) -> Self {
        App { dict }
    }
}
impl App {
    pub fn name(&self) -> Cow<str> {
        self.display_name()
            .or_else(|| self.short_name())
            .unwrap_or_default()
            .into()
    }
    fn display_name(&self) -> Option<&str> {
        read_key(&self.dict, "CFBundleDisplayName")
    }
    /// A user-visible short name for the bundle.
    fn short_name(&self) -> Option<&str> {
        read_key(&self.dict, "CFBundleName")
    }
    pub fn dump(&self) -> String {
        self.dict
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}: {}\n",
                    k,
                    match v {
                        plist::Value::String(s) => format!("{:#?}", s),
                        plist::Value::Boolean(v) => format!("{:#?}", v),
                        plist::Value::Data(v) => format!("{:#?}", v),
                        plist::Value::Date(v) => format!("{:#?}", v),
                        plist::Value::Integer(v) => format!("{:#?}", v),
                        plist::Value::Real(v) => format!("{:#?}", v),
                        plist::Value::Uid(v) => format!("{:#?}", v),
                        // very long, so just print the type
                        plist::Value::Array(_) => "(array)".to_string(),
                        plist::Value::Dictionary(_) => "(dictionary)".to_string(),
                        _ => "(other)".to_string(),
                    }
                )
            })
            .collect()
    }
    pub fn id(&self) -> Cow<str> {
        self.id_raw().into()
    }
    // returns an id in the format tld.publisher.appname
    fn id_raw(&self) -> &str {
        read_key(&self.dict, "CFBundleIdentifier").unwrap_or_default()
    }
    pub fn publisher(&self) -> Cow<str> {
        let id = self.id_raw();
        let mut iter = id.split('.');
        let count = iter.clone().count();

        let by = match count {
            1 | 2 => iter.next(),
            3 | 4 => iter.nth(1),
            _ => iter.next(),
        };

        by.unwrap_or_default().into()
    }
    pub fn version(&self) -> Cow<str> {
        read_key(&self.dict, "CFBundleVersion")
            .unwrap_or_default()
            .into()
    }
    pub fn list() -> Result<impl Iterator<Item = Self>, Box<dyn std::error::Error>> {
        let sys_apps = std::fs::read_dir("/Applications")?;
        let user_apps = std::fs::read_dir(std::env::var("HOME")? + "/Applications")?;
        let apps = sys_apps.chain(user_apps);
        Ok(apps.filter_map(|f| {
            let plist_path = f.ok()?.path().join("Contents").join("info.plist");
            let book = plist::Value::from_file(plist_path).ok()?;
            App::new(book.into_dictionary()?).into()
        }))
    }
}
