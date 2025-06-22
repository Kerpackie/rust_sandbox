
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
    use reqwest::Error;
    use serde_json::Value;
    use super::*;
    
    #[tokio::test]
    async fn tests_calls_async_function() {
        let api_url: &str = "https://pokeapi.co/api/v2/pokemon/ditto"; 
        let my_result = my_async_call(api_url).await;
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