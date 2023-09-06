use reqwest::Client;
use serde_json::json;

pub struct GQL {
    pub quote_by_symbol_query: String,
    pub quote_by_symbol_payload: serde_json::Value,
    pub get_company_news_events_query: String,
    pub get_company_news_events_payload: serde_json::Value,
    pub get_company_filings_query: String,
    pub get_company_filings_payload: serde_json::Value,
}

impl GQL {
    pub fn new() -> GQL {
        GQL {
            quote_by_symbol_query: String::from("query getQuoteBySymbol($symbol: String, $locale: String) { getQuoteBySymbol(symbol: $symbol, locale: $locale) { symbol name price priceChange percentChange exchangeName exShortName exchangeCode marketPlace sector industry volume openPrice dayHigh dayLow MarketCap MarketCapAllClasses peRatio prevClose dividendFrequency dividendYield dividendAmount dividendCurrency beta eps exDividendDate shortDescription longDescription website email phoneNumber fullAddress employees shareOutStanding totalDebtToEquity totalSharesOutStanding sharesESCROW vwap dividendPayDate weeks52high weeks52low alpha averageVolume10D averageVolume30D averageVolume50D priceToBook priceToCashFlow returnOnEquity returnOnAssets day21MovingAvg day50MovingAvg day200MovingAvg dividend3Years dividend5Years datatype __typename } }"),
            quote_by_symbol_payload: json!({
                "operationName": "getQuoteBySymbol",
                "variables": {"locale": "en"},
                "query": quote_by_symbol_query,
            }),
            get_company_news_events_query: String::from("query getNewsAndEvents($symbol: String!, $page: Int!, $limit: Int!, $locale: String!) { news: getNewsForSymbol(symbol: $symbol, page: $page, limit: $limit, locale: $locale) { headline datetime source newsid summary __typename } events: getUpComingEventsForSymbol(symbol: $symbol, locale: $locale) { title date status type __typename } }"),
            get_company_news_events_payload: json!({
                "operationName": "getNewsAndEvents",
                "variables": {
                    "symbol": "ART",
                    "page": 1,
                    "limit": 100,
                    "locale": "en"
                },
                "query": get_company_news_events_query,
            }),
            get_company_filings_query: String::from("query getCompanyFilings($symbol: String! $fromDate: String $toDate: String $limit: Int) { filings: getCompanyFilings(symbol: $symbol fromDate: $fromDate toDate: $toDate limit: $limit) { size filingDate description name urlToPdf __typename } }"),
            get_company_filings_payload: json!({
                "operationName": "getCompanyFilings",
                "variables": {
                    "symbol": "ART",
                    "fromDate": "2020-09-01",
                    "toDate": "2020-09-30",
                    "limit": 100,
                },
                "query": get_company_filings_query,
            }),
        }
    }
}

pub async fn send_request(payload: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.post("https://money.tmx.com/en/quote/{symbol.upper()}")
        .json(&payload)
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}