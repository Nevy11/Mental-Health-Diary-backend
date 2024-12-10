use crate::models::APIResult;

pub async fn narrowed_mhd_q(question_asked: String) -> Option<String> {
    let api_key = "AIzaSyAKCbO2C11vC_IP84UPlcALm2d4fFdWaRQ";
    let cse_id = "56fda9c0fcf864ce0";
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
        api_key, cse_id, question_asked
    );
    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(response) => {
            match response.json::<APIResult>().await {
                Ok(api_result) => {
                    if let Some(first_item) = api_result.items.first() {
                        Some(first_item.snippet.clone())
                    } else {
                        println!("No answers found.");
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Error in converting the response json to a APIResult struct.\nError: {e:?}");
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("Error in googling question: {e:?}");
            None
        }
    }
}
