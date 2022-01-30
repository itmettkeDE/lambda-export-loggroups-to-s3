#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub bucket: Option<String>,
    pub prefix: Option<String>,
    pub invoke_time: Option<chrono::NaiveDateTime>,
}

impl Event {
    pub(crate) fn get_bucket(&self) -> anyhow::Result<std::borrow::Cow<'_, str>> {
        use anyhow::bail;

        if let Some(ref v) = self.bucket {
            return Ok(std::borrow::Cow::Borrowed(v));
        }
        if let Ok(env) = std::env::var(crate::ENV_VAR_BUCKET) {
            return Ok(env.into());
        }
        bail!("Either the lambda event must contain a bucket key or the env variable {}, must be defined.", crate::ENV_VAR_BUCKET)
    }

    pub(crate) fn get_prefix(&self) -> anyhow::Result<std::borrow::Cow<'_, str>> {
        use anyhow::bail;

        if let Some(ref v) = self.prefix {
            return Ok(std::borrow::Cow::Borrowed(v));
        }
        if let Ok(env) = std::env::var(crate::ENV_VAR_PREFIX) {
            return Ok(env.into());
        }
        bail!("Either the lambda event must contain a prefix key or the env variable {}, must be defined.", crate::ENV_VAR_PREFIX)
    }
}
