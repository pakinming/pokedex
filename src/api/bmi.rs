// use crate::domain::bmi::BmiCalculator;
use serde::{Deserialize, Serialize};
use crate::api::Status;




#[derive(Deserialize)]
struct Request {
    weight: f32,
    height: f32,
    
}

#[derive(Serialize)]
struct Response {
   bmi: f32
}



pub fn serve(req: &rouille::Request) -> rouille::Response {

    //  BmiCalculator::calculate(req.weight, req.height)

    println!("{:?}", &req);

    let req = match rouille::input::json_input::<Request>(req) {
        Ok(req) => Request {
            weight: req.weight,
            height: req.height,            
        },
        _ => return rouille::Response::from(Status::BadRequest),
    };

    rouille::Response::json(&Response {
        bmi: calculate(req.weight, req.height)
    })

}


pub fn calculate(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bmi() {
        let weight = 50.0;
        let height = 1.75;
        let bmi = calculate(weight, height);
        assert_eq!(bmi, 22.857143, "The BMI should be correctly calculated");
    }

    #[test]
    fn test_calculate_bmi_with_zero_height() {
        let weight = 70.0;
        let height = 0.0;
        let bmi = calculate(weight, height);
        assert_eq!(bmi, f32::INFINITY, "BMI should be Infinity when height is zero");
    }
}