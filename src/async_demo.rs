use std::io::{Error, ErrorKind};

async fn my_async_call_decode_errors(url: &str) -> Result<serde_json::Value, Error> {
    
    let response: reqwest::Response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;
    
    let json_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;
    
    Ok(json_response)
}

//Return JSON formatted response, or Error
async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {

    let response: serde_json::Value = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_function_decodes_errors() {
        let api_url: &str = "https://pokeapi.co/api/v2/pokemon/ditto";
        let my_result: Result<serde_json::Value, std::io::Error>  = my_async_call_decode_errors(api_url).await;
        match my_result {
            Ok(r) => {
                dbg!(r);
            }
            Err(_) => {
                panic!("Failed to make request");
            }
        }
    }
    
    #[tokio::test]
    async fn tests_calls_async_function() {
        let api_url: &str = "https://pokeapi.co/api/v2/pokemon/ditto";
        let my_result: Result<serde_json::Value, reqwest::Error>  = my_async_call(api_url).await;
        match my_result {
            Ok(r) => {
                dbg!(r);
            }
            Err(_) => {
                panic!("Failed to make request");
            }
        }
    }
}