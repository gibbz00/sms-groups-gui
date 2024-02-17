use serde::{de::DeserializeOwned, Serialize};

pub trait DbDocument: Serialize + DeserializeOwned {
    const NAME: &'static str;
    type Id;

    fn id(&self) -> Self::Id;
}
