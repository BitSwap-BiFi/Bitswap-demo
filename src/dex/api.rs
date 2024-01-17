pub(crate) use crate::non_empty::NonEmpty;
use crate::error::Error;
use crate::Result;
use crate::sources::Source;
use crate::data::{Response, Ticker, BuildRequest, PreparedRequest, Pair};

use serde_json::Value;

pub struct Bitfinex {
    endpoint: String
}



impl Bitfinex {
    pub fn new_with_endpoint(endpoint: &str) -> Bitfinex {
        Bitfinex {
            endpoint: endpoint.into()
        }
    }

    pub fn new() -> Bitfinex {
        Bitfinex::new_with_endpoint("https://api.bitfinex.com/v1/pubticker/btcust")
    }
}

fn parse_pair(s: &str) -> Result<Pair> {
    let ch = s.chars().next().ok_or(Error::InvalidResponse("failed to parse pair".into()))?;
    if ch != 't' {
        return Err(Error::InvalidResponse("pair didn't start with t".into()))
    };

    let c1 = str::parse(&s[1..4])?;
    let c2 = str::parse(&s[4..7])?;

    Ok(Pair::new(c1, c2))
}
/// Return the price in msats/asset
async fn fetch_price(asset_id: &ContractId) -> Result<u64, Box<dyn std::error::Error>> {
	#[derive(Debug, serde::Deserialize)]
	struct BitfinexPrice {
		last_price: String,
	}

	// Map the asset to the right ticker. here we assume it's always USDt
	let body = reqwest::get("https://api.bitfinex.com/v1/pubticker/btcusdt")
		.await?
		.json::<BitfinexPrice>()
		.await?;

	let last_price = body.last_price.parse::<f64>()?;
	let price = (1.0 / last_price * 1e11) as u64;

	println!("Using price from Bitfinex: {} mSAT = 1 {}", price, asset_id);

	Ok(price)
}

// on trading pairs (ex. tBTCUSD)
//[
//    SYMBOL,                  0
//    BID,                     1
//    BID_SIZE,                2
//    ASK,                     3
//    ASK_SIZE,                4
//    DAILY_CHANGE,            5
//    DAILY_CHANGE_RELATIVE,   6
//    LAST_PRICE,              7
//    VOLUME,                  8
//    HIGH,                    9
//    LOW                      10
//],

fn parse_bitfinex_ticker(val: &Value) -> Result<Ticker> {
    let arr = val.as_array()
                 .ok_or(Error::InvalidResponse("ticker value is not an array".into()))?;

    if arr.len() <= 2 {
        return Err(Error::InvalidResponse("ticker response too short".into()));
    }

    let pair_str = arr[0].as_str()
                         .ok_or_else(|| Error::InvalidResponse("could not parse currency pair".into()))?;

    let pair = parse_pair(pair_str)?;

    let rate = arr[1].as_f64()
                     .ok_or_else(|| Error::InvalidResponse("could not parse rate as f64".into()))?;

    Ok(Ticker { pair, rate })
}

fn parse_bitfinex_response(arr: &Value) -> Result<Vec<Ticker>> {
    let tvalues: Vec<Ticker> =
        match arr {
            Value::Array(xs) => xs.iter().map(parse_bitfinex_ticker).collect::<Result<Vec<Ticker>>>()?,
            _ => Err(Error::InvalidResponse("invalid bitfinex response".into()))?
        };
    Ok(tvalues)
}


impl Source for Bitfinex {
    fn name(&self) -> &str {
        "bitfinex"
    }

    fn build_request(&self, req: BuildRequest) -> Result<PreparedRequest> {
        let str_pairs = req
              .pairs
              .iter()
              .map(|p| format!("t{}", p))
              .collect::<Vec<String>>()
              .join(",");

        let request = http::Request::builder()
            .method("GET")
            .uri(String::from(&self.endpoint) + "/v2/tickers?symbols=" + &str_pairs)
            .body(vec![])?;

        Ok(PreparedRequest {
            http_request: request,
            pairs: req.pairs,
            source: self
        })
    }

    fn parse_response(&self, res: &[u8]) -> Result<Response> {
        let values : Value = serde_json::from_slice(res)?;
        let tvalues = parse_bitfinex_response(&values)?;
        let rates = NonEmpty::new_or_err(tvalues, Error::InvalidResponse("empty response".into()))?;
        let source_name = self.name().into();

        Ok(Response { rates, source_name })
    }
}

#[cfg(test)]
mod test_bitfinex {
    use super::*;

    #[test]
    fn test_bitfinex_parsed_response() {
        let bitfinex = Bitfinex::new();
        let resp = r#"[["tBTCUSD",7999.5,26.285500810000006,8000.2,36.156609280000005,93.80135554,0.0119,7999.5,10193.54898341,8180,7776.6]]"#;

        let parsed = bitfinex.parse_response(&resp.as_bytes());
        // println!("{:?}", parsed);
        assert!(parsed.is_ok());
    }
