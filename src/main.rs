use chrono::{DateTime, Local, TimeZone};
use dotenvy::dotenv;
use reqwest::Client;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::stdout;
use std::io::Write;

use termion::clear;

use serde_json::{json, Value};

use viuer::{print_from_file, Config};

mod api;
use api::OpenWeaterToTsv;
/* use api::Clouds;
use api::Coord;
use api::Main;
use api::OpenWeatherResponse;
use api::Sys;
use api::Weather;
use api::Wind; */

use std::time::Instant;
use std::{thread, time};

use url::Url;
use url::ParseError as UrlParseError;

// クライアント定義
struct ApiClient {
    server: String,
    client: Client,
}

// クライアント実装
impl ApiClient {
    async fn get_weather(&self) -> Result<String, Box<dyn std::error::Error>> {
        // APIキーをenvファイルを取得する。
        let api_key = env::var("API_KEY").expect("API KEY env error...");
        let location_name = env::var("LOCATION_NAME").expect("LOCATION NAME env error...");
        // HashMapにQueryParamを設定。
        let mut params = HashMap::new();
        params.insert("q", location_name);
        params.insert("units", "metric".to_string());
        params.insert("lang", "ja".to_string());
        params.insert("appid", api_key);

        match Url::parse(&self.server) {
            Ok(url) => { println!("{}", url); },
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                println!("Err(RelativeUrlWithoutBase)");
            },
            Err(e) => { println!("{}", e); }
        }


        // テスト ここで止める
        panic!("########### END ###########");

        // 非同期でJSONデータを取得。
        let resp = self
            .client
            .get(format!("{}", self.server))
            .query(&params)
            .send()
            .await?;
        // レスポンスから文字列で受け取る。
        let body = resp.text().await?;

        Ok(body)
    }
}

async fn do_get_weather(api_client: ApiClient) -> Result<(), Box<dyn std::error::Error>> {
    let body = api_client.get_weather().await?;
    // JSON文字列を構造体にデシリアライズする。
    /* let deserialize: OpenWeatherResponse =
        serde_json::from_str(&body).expect("deserialize error...");

    println!("response to struct: {:?}", deserialize); */

    // JSON文字列を取り出せるようにvalueにする
    let deserialize: Value = serde_json::from_str(&body)?;
    //println!("test struct: {:?}", deserialize);

    let mut openweather_to_tsv: OpenWeaterToTsv = OpenWeaterToTsv::new();

    // 内部パラメータ ステータスコード200じゃない場合は終了
    let cod = deserialize.get("cod");
    if let Some(v) = cod {
        openweather_to_tsv.cod = v.as_i64().unwrap();
        println!("cod: {}", v.as_i64().unwrap());
        if v.as_i64().unwrap() != 200 {
            println!("not 200 status\ncheck config");
            panic!();
        }
    }

    // 都市の地理的位置、緯度
    let lat = deserialize.get("coord").and_then(|v| v.get("lat"));
    if let Some(v) = lat {
        openweather_to_tsv.lat = v.as_f64().unwrap();
        println!("lat: {:?}", v.as_f64().unwrap());
    }

    // 都市の地理的位置、緯度
    let lon = deserialize.get("coord").and_then(|v| v.get("lon"));
    if let Some(v) = lon {
        openweather_to_tsv.lon = v.as_f64().unwrap();
        println!("lon: {:?}", v.as_f64().unwrap());
    }

    let weather = deserialize.get("weather");
    let mut weather_val = &json!({});
    if let Some(v) = weather {
        let weather_vec = v.as_array().unwrap();
        for v in weather_vec {
            weather_val = v
        }
        println!("weather info count: {}", weather_vec.len());
    }

    // 気象条件ID
    let weather_id = weather_val.get("id");
    if let Some(v) = weather_id {
        openweather_to_tsv.weather_to_id = v.as_i64().unwrap();
        println!("id: {}", v.as_i64().unwrap());
    }

    // 気象パラメータのグループ（雨、雪、極端など）
    let weather_main = weather_val.get("main");
    if let Some(v) = weather_main {
        openweather_to_tsv.weather_to_main = v.as_str().unwrap().to_string();
        println!("main: {}", v.as_str().unwrap());
    }

    // グループ内の気象条件。あなたの言語で出力を得ることができます。
    let weather_description = weather_val.get("description");
    if let Some(v) = weather_description {
        openweather_to_tsv.description = v.as_str().unwrap().to_string();
        println!("description: {}", v.as_str().unwrap());
    }

    // Weather icon id
    let weather_icon = weather_val.get("icon");
    let mut icon_name: &str = "";
    if let Some(v) = weather_icon {
        openweather_to_tsv.icon = v.as_str().unwrap().to_string();
        println!("icon: {}", v.as_str().unwrap());
        icon_name = v.as_str().unwrap();
    }

    // 内部パラメータ
    let base = deserialize.get("base");
    if let Some(v) = base {
        openweather_to_tsv.base = v.as_str().unwrap().to_string();
        println!("base: {}", v.as_str().unwrap());
    }

    // 温度。単位デフォルト：ケルビン、メートル法：摂氏、インペリアル：華氏。
    let temp = deserialize.get("main").and_then(|v| v.get("temp"));
    if let Some(v) = temp {
        openweather_to_tsv.temp = v.as_f64().unwrap();
        println!("temp: {}", v.as_f64().unwrap());
    }

    // 温度。この温度パラメータは、人間の天気の知覚を説明します。単位デフォルト：ケルビン、メートル法：摂氏、インペリアル：華氏。
    let feels_like = deserialize.get("main").and_then(|v| v.get("feels_like"));
    if let Some(v) = feels_like {
        openweather_to_tsv.feels_like = v.as_f64().unwrap();
        println!("feels_like: {}", v.as_f64().unwrap());
    }

    // 最低気温
    let temp_min = deserialize.get("main").and_then(|v| v.get("temp_min"));
    if let Some(v) = temp_min {
        openweather_to_tsv.temp_min = v.as_f64().unwrap();
        println!("temp_min: {}", v.as_f64().unwrap());
    }

    // 最高気温
    let temp_max = deserialize.get("main").and_then(|v| v.get("temp_max"));
    if let Some(v) = temp_max {
        openweather_to_tsv.temp_max = v.as_f64().unwrap();
        println!("temp_max: {}", v.as_f64().unwrap());
    }

    // 大気圧（sea_levelまたはgrnd_levelデータがない場合は、海面上）、hPa
    let pressure = deserialize.get("main").and_then(|v| v.get("pressure"));
    if let Some(v) = pressure {
        openweather_to_tsv.pressure = v.as_i64().unwrap();
        println!("pressure: {}", v.as_i64().unwrap());
    }

    // 海面の大気圧、hPa
    let sea_level = deserialize.get("main").and_then(|v| v.get("sea_level"));
    if let Some(v) = sea_level {
        openweather_to_tsv.sea_level = v.as_i64().unwrap();
        println!("sea_level: {}", v.as_i64().unwrap());
    }

    // 地表面の大気圧、hPa
    let grnd_level = deserialize.get("main").and_then(|v| v.get("grnd_level"));
    if let Some(v) = grnd_level {
        openweather_to_tsv.grnd_level = v.as_i64().unwrap();
        println!("grnd_level: {}", v.as_f64().unwrap());
    }

    // 湿度、％
    let humidity = deserialize.get("main").and_then(|v| v.get("humidity"));
    if let Some(v) = humidity {
        openweather_to_tsv.humidity = v.as_i64().unwrap();
        println!("humidity: {}", v.as_f64().unwrap());
    }

    // 視程、メーター。視程の最大値は10kmです
    let visibility = deserialize.get("visibility");
    if let Some(v) = visibility {
        openweather_to_tsv.visibility = v.as_i64().unwrap();
        println!("visibility: {}", v.as_i64().unwrap());
    }

    // 風速。単位推：メートル/秒、メートル法：メートル/秒、インペリアル：マイル/時。
    let speed = deserialize.get("wind").and_then(|v| v.get("speed"));
    if let Some(v) = speed {
        openweather_to_tsv.speed = v.as_f64().unwrap();
        println!("speed: {}", v.as_f64().unwrap());
    }

    // 風向、度（気象）
    let deg = deserialize.get("wind").and_then(|v| v.get("deg"));
    if let Some(v) = deg {
        openweather_to_tsv.deg = v.as_i64().unwrap();
        println!("deg: {}", v.as_i64().unwrap());
    }

    // 突風。単位デフォルト：メートル/秒、メートル法：メートル/秒、インペリアル：マイル/時
    let gust = deserialize.get("wind").and_then(|v| v.get("gust"));
    if let Some(v) = gust {
        openweather_to_tsv.gust = v.as_f64().unwrap();
        println!("gust: {}", v.as_f64().unwrap());
    }

    // 曇り、％
    let clouds_all = deserialize.get("clouds").and_then(|v| v.get("all"));
    if let Some(v) = clouds_all {
        openweather_to_tsv.all = v.as_i64().unwrap();
        println!("clouds all: {}", v.as_i64().unwrap());
    }

    // 過去1時間の雨量、mm
    let rain_h1 = deserialize.get("rain").and_then(|v| v.get("h1"));
    if let Some(v) = rain_h1 {
        openweather_to_tsv.rain_1h = v.as_f64().unwrap();
        println!("rain h1: {}", v.as_i64().unwrap());
    }

    // 過去3時間の雨量、mm
    let rain_h3 = deserialize.get("rain").and_then(|v| v.get("h3"));
    if let Some(v) = rain_h3 {
        openweather_to_tsv.rain_3h = v.as_f64().unwrap();
        println!("rain h3: {}", v.as_f64().unwrap());
    }

    // 過去1時間の積雪量、mm
    let snow_h1 = deserialize.get("snow").and_then(|v| v.get("h1"));
    if let Some(v) = snow_h1 {
        openweather_to_tsv.snow_h1 = v.as_f64().unwrap();
        println!("sbow h1: {}", v.as_f64().unwrap());
    }

    // 過去3時間の積雪量、mm
    let snow_h3 = deserialize.get("snow").and_then(|v| v.get("h3"));
    if let Some(v) = snow_h3 {
        openweather_to_tsv.snow_h3 = v.as_f64().unwrap();
        println!("snow h3: {}", v.as_f64().unwrap());
    }

    // データ計算の時間、UNIX、UTC
    let dt = deserialize.get("dt");
    if let Some(v) = dt {
        openweather_to_tsv.dt = v.as_i64().unwrap();
        println!("dt: {}", v.as_i64().unwrap());
    }

    // 内部パラメータ
    let sys_type = deserialize.get("sys").and_then(|v| v.get("type"));
    if let Some(v) = sys_type {
        openweather_to_tsv.r#type = v.as_i64().unwrap();
        println!("sys type: {}", v.as_i64().unwrap());
    }

    // 内部パラメータ
    let sys_id = deserialize.get("sys").and_then(|v| v.get("id"));
    if let Some(v) = sys_id {
        openweather_to_tsv.sys_to_id = v.as_i64().unwrap();
        println!("sys id: {}", v.as_i64().unwrap());
    }

    // 内部パラメータ
    let sys_message = deserialize.get("sys").and_then(|v| v.get("message"));
    if let Some(v) = sys_message {
        openweather_to_tsv.message = v.as_f64().unwrap();
        println!("sys message: {:?}", v.as_f64());
    }

    // Country code (GB, JP etc.)
    let sys_country = deserialize.get("sys").and_then(|v| v.get("country"));
    if let Some(v) = sys_country {
        openweather_to_tsv.country = v.as_str().unwrap().to_string();
        println!("sys country: {}", v.as_str().unwrap());
    }

    // 日の出時刻、UNIX、UTC
    let sys_sunrise = deserialize.get("sys").and_then(|v| v.get("sunrise"));
    if let Some(v) = sys_sunrise {
        let dt1: DateTime<Local> = Local.timestamp(v.as_i64().unwrap(), 0);
        openweather_to_tsv.sunrise = dt1.format("%H:%M:%S").to_string();
        println!("sunrise: {}", dt1.format("%H:%M:%S"));
    }

    // 日没時間、UNIX、UTC
    let sys_sunset = deserialize.get("sys").and_then(|v| v.get("sunset"));
    if let Some(v) = sys_sunset {
        let dt1: DateTime<Local> = Local.timestamp(v.as_i64().unwrap(), 0);
        openweather_to_tsv.sunset = dt1.format("%H:%M:%S").to_string();
        println!("sunset: {}", dt1.format("%H:%M:%S"));
    }

    // UTCから秒単位でシフト
    let timezone = deserialize.get("timezone");
    if let Some(v) = timezone {
        openweather_to_tsv.timezone = v.as_i64().unwrap();
        println!("timezone: {}", v.as_i64().unwrap());
    }

    // City ID
    let id = deserialize.get("id");
    if let Some(v) = id {
        openweather_to_tsv.id = v.as_i64().unwrap();
        println!("id: {}", v.as_i64().unwrap());
    }

    // City name
    let name = deserialize.get("name");
    if let Some(v) = name {
        openweather_to_tsv.name = v.as_str().unwrap().to_string();
        println!("name: {}", v.as_str().unwrap());
    }

    // 天気アイコンを表示
    if icon_name != "" {
        let icon_path = format!("./assets/{}.png", icon_name);
        let conf = Config {
            width: Some(20),
            height: Some(10),
            x: 30,
            y: 1,
            ..Default::default()
        };
        print_from_file(icon_path, &conf).expect("icon error");
    } else {
        println!("No Icon");
    }

    // 構造体をやめたjson valueで取る(OpenWeatherのAPIのJson定義で1hという名前で構造体を作成できず。。。)
    // Coord Info
    /* let coord = deserialize.coord;
    let coord = match coord {
        None => Coord { lon: 0.0, lat: 0.0 },
        Some(val) => val,
    };
    let lon = coord.lon;
    let lat = coord.lat;
    println!("lo: {}, lat: {}", lon, lat);

    // Weater Info
    let westher_vec = deserialize.weather;
    let westher_vec = match westher_vec {
        None => {
            let weather_none = Weather {
                id: 0,
                main: String::from(""),
                description: String::from(""),
                icon: String::from(""),
            };
            let mut weather_none_vec = vec![];
            weather_none_vec.push(weather_none);
            weather_none_vec
        }
        Some(val) => val,
    };
    let weather = &westher_vec[0];
    let weather_to_id = weather.id;
    let weather_to_main = &weather.main;
    let description = &weather.description;
    let icon = &weather.icon;
    println!(
        "id: {}, main: {}, description: {}, icon: {}",
        weather_to_id, weather_to_main, description, icon
    );

    // Base Info
    let base = &deserialize.base;
    let base = match base {
        None => "",
        Some(val) => val,
    };
    println!("base: {}", base);

    // Main Info
    let main = deserialize.main;
    let main = match main {
        None => Main {
            temp: 0.0,
            feels_like: 0.0,
            temp_min: 0.0,
            temp_max: 0.0,
            pressure: 0,
            humidity: 0,
        },
        Some(val) => val,
    };
    let temp = main.temp;
    let feels_like = main.feels_like;
    let temp_min = main.temp_min;
    let temp_max = main.temp_max;
    let pressure = main.pressure;
    let humidity = main.humidity;
    println!(
        "temp: {}, feels_like: {}, temp_min: {}, temp_max: {}, pressure: {}, humidity: {}",
        temp, feels_like, temp_min, temp_max, pressure, humidity
    );

    // Visibility Info
    let visibility = deserialize.visibility;
    let visibility = match visibility {
        None => 0,
        Some(val) => val,
    };
    println!("visibility: {}", visibility);

    // Wind Info
    let wind = deserialize.wind;
    let wind = match wind {
        None => Wind { speed: 0.0, deg: 0 },
        Some(val) => val,
    };
    let speed = wind.speed;
    let deg = wind.deg;
    println!("speed: {}, deg: {}", speed, deg);

    // Clouds Info
    let clouds = deserialize.clouds;
    let clouds = match clouds {
        None => Clouds { all: 0 },
        Some(val) => val,
    };
    let all = clouds.all;
    println!("all {}", all);

    // Dt Info
    let dt = deserialize.dt;
    let dt = match dt {
        None => 0,
        Some(val) => val,
    };
    println!("dt: {}", dt);

    // Sys Info
    let sys = deserialize.sys;
    let sys = match sys {
        None => Sys {
            r#type: 0,
            id: 0,
            message: Some(0.0),
            country: String::from(""),
            sunrise: 0,
            sunset: 0,
        },
        Some(val) => val,
    };
    let r#type = sys.r#type;
    let sys_to_id = sys.id;
    let message = sys.message;
    let message = match message {
        None => 0.0,
        Some(val) => val,
    };
    let country = &sys.country;
    let sunrise = sys.sunrise;
    let sunset = sys.sunset;
    println!(
        "type: {}, sys_to_id: {}, message: {}, country: {}, sunrise: {}, sunset: {}",
        r#type, sys_to_id, message, country, sunrise, sunset
    );

    // Timezone Info
    let timezone = deserialize.timezone;
    let timezone = match timezone {
        None => 0,
        Some(val) => val,
    };
    println!("timezone: {}", timezone);

    // Id Info
    let id = deserialize.id;
    let id = match id {
        None => 0,
        Some(val) => val,
    };
    println!("id: {}", id);

    // Name Info
    let name = &deserialize.name;
    let name = match name {
        None => "",
        Some(val) => val,
    };
    println!("name: {}", name);

    // Cod Info
    let cod = deserialize.cod;
    let cod = match cod {
        None => 0,
        Some(val) => val,
    };
    println!("cod: {}", cod);

    let openweather_to_tsv = OpenWeaterToTsv::new(
        lon,
        lat,
        weather_to_id,
        weather_to_main,
        description,
        icon,
        base,
        temp,
        feels_like,
        temp_min,
        temp_max,
        pressure,
        humidity,
        visibility,
        speed,
        deg,
        all,
        dt,
        r#type,
        sys_to_id,
        message,
        country,
        sunrise,
        sunset,
        timezone,
        id,
        name,
        cod,
    );

    */

    // 環境設定ファイルで出力するかを判定
    let tsv_out_flg = env::var("TSV_OUT").expect("env error...");
    if PartialEq::eq(&tsv_out_flg, "1") {
        // tsvファイルの作成
        weather_write_to_tsv(openweather_to_tsv).expect("tsv to write error...");
    }

    Ok(())
}

// 環境設定し直す
fn re_setting() -> std::io::Result<()> {
    println!("\n");
    println!("Enter the API KEY.");
    let mut input_api_key = String::new();
    std::io::stdin().read_line(&mut input_api_key).ok();
    let result_apikey = input_api_key.trim().to_string();
    println!("\n");
    println!("Enter the API URL.");
    let mut input_url = String::new();
    std::io::stdin().read_line(&mut input_url).ok();
    let result_url = input_url.trim().to_string();
    println!("\n");
    println!("Enter the API LOCATION.");
    let mut input_location = String::new();
    std::io::stdin().read_line(&mut input_location).ok();
    let result_location = input_location.trim().to_string();

    let env_file = format!(
        "OPEN_WEATHER_URL={}\nAPI_KEY={}\nLOCATION_NAME={}",
        result_url, result_apikey, result_location
    );

    let mut input_ans = String::new();
    println!("\n");
    println!("{}", env_file);
    println!("Is it okay to reflect it in the settings? y or n");
    std::io::stdin().read_line(&mut input_ans).ok();
    let result_ans = input_ans.trim();

    match result_ans {
        "Y" | "y" => {
            let file = File::create("./.env");
            let mut file = match file {
                Err(_) => panic!("not found filepath..."),
                Ok(val) => val,
            };
            write!(file, "{}", env_file)?;
            file.flush()?;
            println!("\n");
            println!("The settings have been reflected.");
            println!("\n");
        }
        _ => {
            println!("\n");
            println!("Does not reflect the settings.");
            println!("\n");
        }
    }

    Ok(())
}

fn weather_write_to_tsv(openweather_to_tsv: OpenWeaterToTsv) -> Result<(), std::io::Error> {
    let local: DateTime<Local> = Local::now();
    let local_datetime = local.format("%Y-%m-%d%H:%M:%S").to_string();

    let mut wtr = csv::WriterBuilder::new()
        // 区切りにする
        .delimiter(b'\t')
        .from_path(format!("./weatherlog/{}.tsv", &local_datetime))
        .expect("Path not found...");
    // 天気情報の構造体をシリアライズ化して追加する
    wtr.serialize(openweather_to_tsv)
        .expect("weather info serialize error...");
    wtr.flush().expect("Write error...");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("./weatherlog").expect("dir create error");
    dotenv().ok();

    let api_key = env::var("API_KEY");
    let _api_key = match api_key {
        Err(_) => String::from("4378163cb4675f5aeff249a30842c89e"),
        Ok(val) => val,
    };
    let url = env::var("OPEN_WEATHER_URL");
    let url = match url {
        Err(_) => String::from("https://api.openweathermap.org/data/2.5/weather"),
        Ok(val) => val,
    };
    let location = env::var("LOCATION_NAME");
    let location = match location {
        Err(_) => String::from("osaka"),
        Ok(val) => val,
    };
    println!("Do you want to set it up?");
    println!("Currently set of: ");
    println!("API KEY:         {}", "###########");
    println!("OpenWeather URL: {}", url);
    println!("Location name:   {}", location);
    println!("Please enter Y or N : ");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let result = input.trim();
        match result {
            "Y" | "y" => {
                let _ = re_setting();
                break;
            }
            "N" | "n" => {
                break;
            }
            _ => {
                println!("Please try again.");
                println!("enter Y(y) or N(n) : ");
            }
        }
    }

    // 30分毎に取得する
    let thirty_minutes = time::Duration::from_secs(1800);
    // 7日間で自動停止
    let end_time = time::Duration::from_secs(604800);
    let start = Instant::now();
    loop {
        let client = Client::new();
        let server = env::var("OPEN_WEATHER_URL");
        let server = match server {
            Err(_) => String::from("https://api.openweathermap.org/data/2.5/weather"),
            Ok(val) => val,
        };

        let api_client = ApiClient {
            server: server,
            client: client,
        };

        // 標準出力
        let mut stdout = stdout();

        // 表示を一旦クリア
        write!(stdout, "{}", clear::All)?;

        // 非同期でデータを受け取る
        do_get_weather(api_client).await?;

        // 経過時間を取得
        let end = start.elapsed();

        // 経過時間が閾値を超えていた場合終了
        if end > end_time {
            break;
        }
        // スレッドを指定した時間sleepする
        thread::sleep(thirty_minutes);
    }

    Ok(())
}
