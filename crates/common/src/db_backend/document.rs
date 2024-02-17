use serde::{de::DeserializeOwned, Serialize};

pub trait DbDocument: Serialize + DeserializeOwned {
    const NAME: &'static str;

    fn id(&self) -> impl Into<String>;
}
