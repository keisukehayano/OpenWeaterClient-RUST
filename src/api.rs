use serde::Deserialize;
use serde::Serialize;

// API定義
// JSONを受け取ったあとに構造体にデシリアライズする為のもの

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    // 都市の地理的位置、経度
    pub lon: f32,
    // 都市の地理的位置、経度
    pub lat: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    // 気象条件ID
    pub id: i32,
    // 気象パラメータのグループ（雨、雪、極端など）
    pub main: String,
    // グループ内の気象条件。あなたの言語で出力を得ることができます。
    pub description: String,
    // 天気アイコンID
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    // 温度。単位のデフォルト：ケルビン、メートル法：摂氏、インペリアル：華氏。
    pub temp: f32,
    // 温度。この温度パラメータは、人間の天気の知覚を説明します。
    // 単位のデフォルト：ケルビン、メートル法：摂氏、インペリアル：華氏。
    pub feels_like: f32,
    // 現時点での最低気温。これは、現在観測されている最低気温です（大規模なメガロポリスや都市部内）。
    // 単位のデフォルト：ケルビン、メートル法：摂氏、インペリアル：華氏。
    pub temp_min: f32,
    // 現時点での最高気温。これは、現在観測されている最高気温です（大規模なメガロポリスと都市部内）。
    // 単位のデフォルト：ケルビン、メートル法：摂氏、インペリアル：華氏。
    pub temp_max: f32,
    // 大気圧（sea_levelまたはgrnd_levelデータがない場合は、海面上）、hPa
    pub pressure: i32,
    // 湿度、％
    pub humidity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    // 風速。
    // 単位デフォルト：メートル/秒、メートル法：メートル/秒、インペリアル：マイル/時。
    pub speed: f32,
    // 風向、度（気象）
    pub deg: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clouds {
    // 曇り、％
    pub all: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    // 内部パラメータ
    pub r#type: i32,
    // 内部パラメータ
    pub id: i32,
    // 内部パラメータ
    pub message: Option<f32>,
    // 国コード（GB、JPなど）
    pub country: String,
    // 日の出時刻、UNIX、UTC
    pub sunrise: i32,
    // 日没時間、UNIX、UTC
    pub sunset: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenWeatherResponse {
    pub coord: Option<Coord>,
    pub weather: Option<Vec<Weather>>,
    // 内部パラメータ
    pub base: Option<String>,
    pub main: Option<Main>,
    // 視程、メーター。視程の最大値は10kmです
    pub visibility: Option<i32>,
    pub wind: Option<Wind>,
    pub clouds: Option<Clouds>,
    // データ計算の時間、UNIX、UTC
    pub dt: Option<i32>,
    pub sys: Option<Sys>,
    // UTCから秒単位でシフト
    pub timezone: Option<i32>,
    // City ID
    pub id: Option<i32>,
    // City name
    pub name: Option<String>,
    // 内部パラメータ
    pub cod: Option<i32>,
}

// Coord構造体の初期化
/* impl Coord {
    pub fn new(lon: f32, lat: f32) -> Self {
        Coord {
            lon: lon,
            lat: lat,
        }
    }
}

// Weather構造体の初期化
impl Weather {
    pub fn new(id: i32, main: String, description: String, icon: String) -> Self {
        Weather {
            id: id,
            main: main,
            description: description,
            icon: icon,
        }
    }
}

// Main構造体の初期化
impl Main {
    pub fn new(temp: f32, feels_like: f32, temp_min: f32, temp_max: f32, pressure: i32, humidity: i32) -> Self {
        Main {
            temp: temp,
            feels_like: feels_like,
            temp_min: temp_min,
            temp_max: temp_max,
            pressure: pressure,
            humidity: humidity,
        }
    }
}

// Wind構造体の初期化
impl Wind {
    pub fn new(speed: f32, deg: i32) -> Self {
        Wind {
            speed: speed,
            deg: deg,
        }
    }
}

// Clouds構造体の初期化
impl Clouds {
    pub fn new(all: i32) -> Self {
        Clouds {
            all: all,
        }
    }
}

// Sys構造体の初期化
impl Sys {
    pub fn new(
        r#type: i32,
        id: i32,
        message: f32,
        country: String,
        sunrise: i32,
        sunset: i32,
    ) -> Self {
        Sys {
            r#type: r#type,
            id: id,
            message: Some(message),
            country: country,
            sunrise: sunrise,
            sunset: sunset,
        }
    }
} */

// tsv 変換用の構造体
/* #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OpenWeaterToTsv<'a> {
    pub lon: f32,
    pub lat: f32,
    pub weather_to_id: i32,
    pub weather_to_main: &'a str,
    pub description: &'a str,
    pub icon: &'a str,
    pub base: &'a str,
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
    pub visibility: i32,
    pub speed: f32,
    pub deg: i32,
    pub all: i32,
    pub dt: i32,
    pub r#type: i32,
    pub sys_to_id: i32,
    pub message: f32,
    pub country: &'a str,
    pub sunrise: i32,
    pub sunset: i32,
    pub timezone: i32,
    pub id: i32,
    pub name: &'a str,
    pub cod: i32,
} */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenWeaterToTsv {
    pub lon: f64,
    pub lat: f64,
    pub weather_to_id: i64,
    pub weather_to_main: String,
    pub description: String,
    pub icon: String,
    pub base: String,
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
    pub humidity: i64,
    pub visibility: i64,
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
    pub all: i64,
    pub rain_1h: f64,
    pub rain_3h: f64,
    pub snow_h1: f64,
    pub snow_h3: f64,
    pub dt: i64,
    pub r#type: i64,
    pub sys_to_id: i64,
    pub message: f64,
    pub country: String,
    pub sunrise: String,
    pub sunset: String,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

/* impl<'a> OpenWeaterToTsv<'a> {
    pub fn new(
        lon: f32,
        lat: f32,
        weather_to_id: i32,
        weather_to_main: &'a str,
        description: &'a str,
        icon: &'a str,
        base: &'a str,
        temp: f32,
        feels_like: f32,
        temp_min: f32,
        temp_max: f32,
        pressure: i32,
        humidity: i32,
        visibility: i32,
        speed: f32,
        deg: i32,
        all: i32,
        dt: i32,
        r#type: i32,
        sys_to_id: i32,
        message: f32,
        country: &'a str,
        sunrise: i32,
        sunset: i32,
        timezone: i32,
        id: i32,
        name: &'a str,
        cod: i32,
    ) -> Self {
        OpenWeaterToTsv {
            lon: lon,
            lat: lat,
            weather_to_id: weather_to_id,
            weather_to_main: weather_to_main,
            description: description,
            icon: icon,
            base: base,
            temp: temp,
            feels_like: feels_like,
            temp_min: temp_min,
            temp_max: temp_max,
            pressure: pressure,
            humidity: humidity,
            visibility: visibility,
            speed: speed,
            deg: deg,
            all: all,
            dt: dt,
            r#type: r#type,
            sys_to_id: sys_to_id,
            message: message,
            country: country,
            sunrise: sunrise,
            sunset: sunset,
            timezone: timezone,
            id: id,
            name: name,
            cod: cod,
        }
    }
} */

impl OpenWeaterToTsv {
    pub fn new() -> Self {
        OpenWeaterToTsv {
            lon: 0.0,
            lat: 0.0,
            weather_to_id: 0,
            weather_to_main: String::from(""),
            description: String::from(""),
            icon: String::from(""),
            base: String::from(""),
            temp: 0.0,
            feels_like: 0.0,
            temp_min: 0.0,
            temp_max: 0.0,
            pressure: 0,
            sea_level: 0,
            grnd_level: 0,
            humidity: 0,
            visibility: 0,
            speed: 0.0,
            deg: 0,
            gust: 0.0,
            all: 0,
            rain_1h: 0.0,
            rain_3h: 0.0,
            snow_h1: 0.0,
            snow_h3: 0.0,
            dt: 0,
            r#type: 0,
            sys_to_id: 0,
            message: 0.0,
            country: String::from(""),
            sunrise: String::from(""),
            sunset: String::from(""),
            timezone: 0,
            id: 0,
            name: String::from(""),
            cod: 0,
        }
    }
}
