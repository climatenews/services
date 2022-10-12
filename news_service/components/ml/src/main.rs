use db::init_env;
use openai_api::Client;

#[tokio::main]
async fn main() {
    init_env();
    let api_token = std::env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(&api_token);
    let prompt = String::from("The following is a list of companies and the categories they fall into:\n\nApple, Facebook, Fedex\n\nApple\nCategory:",);
    println!(
        "{}{}",
        prompt,
        client.complete_prompt(prompt.as_str()).await.unwrap()
    );
}


// response = openai.Completion.create(
//   model="text-davinci-002",
//   prompt="The following is a list of companies and the categories they fall into:\n\nApple, Facebook, Fedex\n\nApple\nCategory:",
//   temperature=0,
//   max_tokens=6,
//   top_p=1,
//   frequency_penalty=0,
//   presence_penalty=0,
//   stop=["\n"]
// )