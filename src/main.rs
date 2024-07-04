use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result as AnyhowResult;
use graphql_client::{GraphQLQuery, Response};
use reqwest::header;
use reqwest::header::COOKIE;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Category {
    Estate,
    Parcel,
    Wearable,
    End,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    category: Category,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema1.graphql",
    query_path = "query1.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct MyQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "traggo-graphql/schema.graphql",
    query_path = "traggo-graphql/login.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "traggo-graphql/schema.graphql",
    query_path = "traggo-graphql/settings.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Settings;

async fn perform_login(
    variables: login::Variables,
    url: &str,
) -> Result<(String, Response<login::ResponseData>), Error> {
    let request_body = Login::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post(url).json(&request_body).send().await?;

    let headers = res.headers().clone();

    let cookie_header = headers.get("Set-Cookie").unwrap();

    let cookie_value = cookie_header.to_str()?;

    let halves: Vec<&str> = cookie_value.split("; ").collect::<Vec<&str>>();

    let response_body: Response<login::ResponseData> = res.json().await?;

    Ok((halves[0].to_string(), response_body))
}

async fn query_settings(
    variables: settings::Variables,
    url: &str,
    cookie: &str,
) -> Result<Response<settings::ResponseData>, Error> {
    let request_body = Settings::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&request_body)
        .header(COOKIE, cookie)
        .send()
        .await?;

    let response_body: Response<settings::ResponseData> = res.json().await?;

    Ok(response_body)
}

#[tokio::main]
async fn main() -> AnyhowResult<(), Error> {
    let url = "";
    let login_variables = login::Variables {
        name: "".to_string(),
        pass: "".to_string(),
        device_type: login::DeviceType::ShortExpiry,
    };

    let (cookie, _) = perform_login(login_variables, &url).await?;

    println!("Using token: {:?}", cookie);

    let settings_variables = settings::Variables {};

    let settings_response = query_settings(settings_variables, &url, &cookie).await?;

    println!("Settings response: {:?}", settings_response);

    Ok(())
}
