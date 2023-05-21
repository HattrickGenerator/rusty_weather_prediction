extern crate reqwest;
extern crate forecast;

use reqwest::Client;

use forecast::{ApiClient, ForecastRequestBuilder,
               TimeMachineRequestBuilder, ExcludeBlock, ExtendBy,
               Lang, Units};

const LAT: f64 = 6.66;
const LONG: f64 = 66.6;
const TIME: u64 = 666;

fn main() {
    //code piece taken from:

    let api_key = "my_dark_sky_api_key"; // please don't actually hardcode your API key!

    let reqwest_client = Client::new();
    let _api_client = ApiClient::new(&reqwest_client);

    let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

    let _forecast_request = ForecastRequestBuilder::new(api_key, LAT, LONG)
        .exclude_block(ExcludeBlock::Hourly)
        .exclude_blocks(&mut blocks)
        .extend(ExtendBy::Hourly)
        .lang(Lang::English)
        .units(Units::SI)
        .build();

    let _time_machine_request = TimeMachineRequestBuilder::new(api_key, LAT, LONG, TIME)
        .exclude_block(ExcludeBlock::Hourly)
        .exclude_blocks(&mut blocks)
        .lang(Lang::Arabic)
        .units(Units::Imperial)
        .build();

    // let forecast_response = api_client.get_forecast(forecast_request).unwrap();
    // let time_machine_response = api_client.get_time_machine(time_machine_request).unwrap();
}