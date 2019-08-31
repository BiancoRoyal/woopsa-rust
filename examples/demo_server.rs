#[macro_use]
extern crate json;
extern crate woopsa;

fn main() {
    let weatherStation = json::parse(
        r#"
        {
            Temperature: 24.2,
            IsRaining: false,
            Sensitivity: 0.5,
            Altitude: 430,
            City: "Geneva",
            Time: new Date(),
            Thermostat: {
                SetPoint: 24.0
            }
        }
    "#,
    )
    .unwrap();

    woopsa
        .Server
        .start_with_model(weatherStation, "127.0.0.1:1880");
}
