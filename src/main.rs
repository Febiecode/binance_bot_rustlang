use dotenv::dotenv;
use std::{env, error::Error};


use teloxide::{payloads::SendMessageSetters, prelude::*};
use teloxide::utils::command::BotCommand;
use teloxide::{utils::markdown::link};
use teloxide::types::ParseMode::MarkdownV2;

use binance::api::*;
use binance::market::*;
// use binance::account::*;

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

// /help
// /register
// /price BTC
// /price BTC USDT
// /price btc usdt
// /price BNB BTC
// /price bnb btc
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "display this text.")]
  Help,
  #[command(description = "show a Binance sign up page.")]
  Register,
  #[command(description = "show a cryptcurrency price in USDT by default.")]
  Price(String),
//   #[command(description = "handle a username and an age.", parse_with = "split")]
//   UsernameAndAge { username: String, age: u8 },
}

async fn responses_to_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let binance_api = dotenv!("BINANCE_API");
    // let binance_secret = dotenv!("BINANCE_SECRET");

    let market: Market = Binance::new(None, None);
    // let account: Account = Binance::new(Some(binance_api.into()), Some(binance_secret.into()));

    // match market.get_price("BTCUSDT") {
    //     Ok(answer) => println!("{:#?}", answer),
    //     Err(e) => println!("Error: {:#?}", e),
    // }

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        // Command::Register => cx.answer(format!("Don't have a Binance account yet, you can make it here. (https://accounts.binance.com/en/register?ref=SQ86TYC5)")).await?,
        Command::Register => {
            // ERROR teloxide::error_handlers > Error: ApiError { kind: Unknown("Bad Request: can't parse entities: Character '!' is reserved and must be escaped with the preceding '\\'"), status_code: 400 }
            let register_link = link("https://accounts.binance.com/en/register?ref=SQ86TYC5", "Don't have a Binance account yet? You can register here\\.");
            cx.answer(register_link).parse_mode(MarkdownV2).send().await?
            // cx.answer(reigster_link).await?
        },
        Command::Price(crpytocurrency) => {
            let mut iter = crpytocurrency.split_whitespace();

            if let Some(first_crypto_symbol) = iter.next() {

                let second_crypto_symbol = if let Some(second_crypto_symbol) = iter.next() {
                    println!("There was a second_crypto_symbol.");
                    second_crypto_symbol
                } else {
                    println!("There was no second_crypto_symbol. Use default.");
                    "USDT"
                };
                // We won't iter anymore so we don't need to handle the third symbol(characters)

                let target = to_uppercase(
                    &format!("{}{}", &first_crypto_symbol, &second_crypto_symbol)
                );

                match market.get_price(target) {
                    Ok(symbol_price) => {
                        println!("{:#?}", &symbol_price); // Use log instead? log::info!(&symbol_price);
                        // cx.answer(format!("The price you want is {:#?}.", &symbol_price.price)).await?
                        cx.answer(format!("The price you want is {:#?}. ", &symbol_price.price)).await?
                    },
                    Err(e) => {
                        eprint!("{:#?}", e); // Use log instead? log::error!(e);

                        cx.answer(format!("Something went wrong. Did you use the correct cryptocurrency pair?")).await?
                    },
                }
            } else {
                cx.answer("Cryptocurrency symbols were not specified. To start with, you can use /price ETH or /price ETH USDT.").await?
                // Without the code above, Teloxide shows this error internally.
                // ERROR teloxide::error_handlers > Error: ApiError { kind: MessageTextIsEmpty, status_code: 400 }
            }
        }

        // Command::UsernameAndAge { username, age } => {
        //     cx.answer(format!("Your username is @{} and age is {}.", username, age)).await?
        // }
    };

    Ok(())
}

// cargo wuse dotenv::dotenv;
use std::{env, error::Error};


use teloxide::{payloads::SendMessageSetters, prelude::*};
use teloxide::utils::command::BotCommand;
use teloxide::{utils::markdown::link};
use teloxide::types::ParseMode::MarkdownV2;

use binance::api::*;
use binance::market::*;
// use binance::account::*;

fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

// /help
// /register
// /price BTC
// /price BTC USDT
// /price btc usdt
// /price BNB BTC
// /price bnb btc
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
  #[command(description = "display this text.")]
  Help,
  #[command(description = "show a Binance sign up page.")]
  Register,
  #[command(description = "show a cryptcurrency price in USDT by default.")]
  Price(String),
//   #[command(description = "handle a username and an age.", parse_with = "split")]
//   UsernameAndAge { username: String, age: u8 },
}

async fn responses_to_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let binance_api = dotenv!("BINANCE_API");
    // let binance_secret = dotenv!("BINANCE_SECRET");

    let market: Market = Binance::new(None, None);
    // let account: Account = Binance::new(Some(binance_api.into()), Some(binance_secret.into()));

    // match market.get_price("BTCUSDT") {
    //     Ok(answer) => println!("{:#?}", answer),
    //     Err(e) => println!("Error: {:#?}", e),
    // }

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        // Command::Register => cx.answer(format!("Don't have a Binance account yet, you can make it here. (https://accounts.binance.com/en/register?ref=SQ86TYC5)")).await?,
        Command::Register => {
            // ERROR teloxide::error_handlers > Error: ApiError { kind: Unknown("Bad Request: can't parse entities: Character '!' is reserved and must be escaped with the preceding '\\'"), status_code: 400 }
            let register_link = link("https://accounts.binance.com/en/register?ref=SQ86TYC5", "Don't have a Binance account yet? You can register here\\.");
            cx.answer(register_link).parse_mode(MarkdownV2).send().await?
            // cx.answer(reigster_link).await?
        },
        Command::Price(crpytocurrency) => {
            let mut iter = crpytocurrency.split_whitespace();

            if let Some(first_crypto_symbol) = iter.next() {

                let second_crypto_symbol = if let Some(second_crypto_symbol) = iter.next() {
                    println!("There was a second_crypto_symbol.");
                    second_crypto_symbol
                } else {
                    println!("There was no second_crypto_symbol. Use default.");
                    "USDT"
                };
                // We won't iter anymore so we don't need to handle the third symbol(characters)

                let target = to_uppercase(
                    &format!("{}{}", &first_crypto_symbol, &second_crypto_symbol)
                );

                match market.get_price(target) {
                    Ok(symbol_price) => {
                        println!("{:#?}", &symbol_price); // Use log instead? log::info!(&symbol_price);
                        // cx.answer(format!("The price you want is {:#?}.", &symbol_price.price)).await?
                        cx.answer(format!("The price you want is {:#?}. ", &symbol_price.price)).await?
                    },
                    Err(e) => {
                        eprint!("{:#?}", e); // Use log instead? log::error!(e);

                        cx.answer(format!("Something went wrong. Did you use the correct cryptocurrency pair?")).await?
                    },
                }
            } else {
                cx.answer("Cryptocurrency symbols were not specified. To start with, you can use /price ETH or /price ETH USDT.").await?
                // Without the code above, Teloxide shows this error internally.
                // ERROR teloxide::error_handlers > Error: ApiError { kind: MessageTextIsEmpty, status_code: 400 }
            }
        }

        // Command::UsernameAndAge { username, age } => {
        //     cx.answer(format!("Your username is @{} and age is {}.", username, age)).await?
        // }
    };

    Ok(())
}

// cargo watch -x "run -- --release"
// cargo watch -x check
#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok(); // Read .env(TELOXIDE_TOKEN) and set env variables with this

    teloxide::enable_logging!();
    log::info!("Starting the_bot...");

    let bot = Bot::from_env().auto_send();
    let bot_name: String = "bianance_bot".into();

    teloxide::commands_repl(bot, bot_name, responses_to_command).await;
}atch -x "run -- --release"
// cargo watch -x check
#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok(); // Read .env(TELOXIDE_TOKEN) and set env variables with this

    teloxide::enable_logging!();
    log::info!("Starting the_bot...");

    let bot = Bot::from_env().auto_send();
    let bot_name: String = "bianance_bot".into();

    teloxide::commands_repl(bot, bot_name, responses_to_command).await;
}
