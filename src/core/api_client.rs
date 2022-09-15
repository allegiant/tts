use serde::Deserialize;

pub fn get_data(url: &str) -> String {
    match get_response(url) {
        Ok(resoonse) => resoonse.data,
        Err(err) => {
            println!("{:?}", err);
            String::from("")
        }
    }
}

fn get_response(url: &str) -> Result<BaseResponse, Box<dyn std::error::Error>> {
    let json: BaseResponse = reqwest::blocking::get(url)?
        .json::<BaseResponse>()?;
    // if json.successed() {
    //     println!("{:?}", json.code);
    //     println!("{:?}", json.message);
    //     println!("{:?}", json.status);
    //     if json.has_data() {
    //         println!("{:?}", json.data);
    //     }
    // }

    Ok(json)
}

#[derive(Deserialize, Debug)]
struct BaseResponse {
    code: i16,
    message: String,
    status: bool,
    data: String,
}

impl BaseResponse {
    fn successed(&self) -> bool {
        self.code == 200
    }

    fn has_data(&self) -> bool {
        self.data.len() > 0
    }
}
