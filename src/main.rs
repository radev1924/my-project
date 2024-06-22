use std::fmt::format;

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNER: f64 = 0.20;
const PROFIT: f64 = 0.05;
const OPEX: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();

            let tax: f64 = num * TAXPER;
            let owner: f64 = num * OWNER;
            let profit: f64 = num * PROFIT;
            let opex: f64 = num * OPEX;

            let result: String = format!("Taxes:{:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpex: {:.2}", tax, owner, profit, opex);


            ui.set_results( result.into());
        }
    });

    ui.run()
}
