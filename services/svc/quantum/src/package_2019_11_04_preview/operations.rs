#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = "https://quantum.azure.com";
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn jobs(&self) -> jobs::Client {
        jobs::Client(self.clone())
    }
    pub fn providers(&self) -> providers::Client {
        providers::Client(self.clone())
    }
    pub fn quotas(&self) -> quotas::Client {
        quotas::Client(self.clone())
    }
    pub fn storage(&self) -> storage::Client {
        storage::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    Jobs_List(#[from] jobs::list::Error),
    #[error(transparent)]
    Jobs_Get(#[from] jobs::get::Error),
    #[error(transparent)]
    Jobs_Create(#[from] jobs::create::Error),
    #[error(transparent)]
    Jobs_Cancel(#[from] jobs::cancel::Error),
    #[error(transparent)]
    Providers_GetStatus(#[from] providers::get_status::Error),
    #[error(transparent)]
    Storage_SasUri(#[from] storage::sas_uri::Error),
    #[error(transparent)]
    Quotas_List(#[from] quotas::list::Error),
}
pub mod jobs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
            }
        }
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            job_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                job_id: job_id.into(),
            }
        }
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            job_id: impl Into<String>,
            job: impl Into<models::JobDetails>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                job_id: job_id.into(),
                job: job.into(),
            }
        }
        pub fn cancel(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            job_id: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                job_id: job_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::JobDetailsList, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::JobDetailsList =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) job_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::JobDetails, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name,
                        &self.job_id
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::JobDetails =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RestError =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod create {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::JobDetails),
            Created201(models::JobDetails),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) job_id: String,
            pub(crate) job: models::JobDetails,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name,
                        &self.job_id
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::PUT);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.job).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::JobDetails =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(Response::Ok200(rsp_value))
                        }
                        http::StatusCode::CREATED => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::JobDetails =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(Response::Created201(rsp_value))
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RestError =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod cancel {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) job_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<(), Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/jobs/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name,
                        &self.job_id
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::DELETE);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::NO_CONTENT => Ok(()),
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RestError =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod providers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> get_status::Builder {
            get_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
            }
        }
    }
    pub mod get_status {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::ProviderStatusList, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/providerStatus",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ProviderStatusList =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RestError =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod storage {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn sas_uri(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
            blob_details: impl Into<models::BlobDetails>,
        ) -> sas_uri::Builder {
            sas_uri::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
                blob_details: blob_details.into(),
            }
        }
    }
    pub mod sas_uri {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
            pub(crate) blob_details: models::BlobDetails,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::SasUriResponse, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/storage/sasUri",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::POST);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.blob_details).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::SasUriResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RestError =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod quotas {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            workspace_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                workspace_name: workspace_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::RestError,
            },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::error::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::error::Error),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) workspace_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::QuotaList, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/v1.0/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Quantum/workspaces/{}/quotas",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.workspace_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::QuotaList =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RestError =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}
