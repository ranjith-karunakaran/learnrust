#[derive(serde::Deserialize)]
struct EData {
    year: String, 
    industry_code_ANZSIC: String,
    industry_name_ANZSIC: String, 
    variable: String,
}
fn main() {
    if let Ok(csv) = std::fs::read_to_string("test.csv") {
        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        for row in rdr.deserialize() {
            let edata: EData = row.unwrap();
            println!("year: {}, industry_code: {:>4}, industry_name: {:>20}, variable: {:>20}", 
                edata.year, edata.industry_code_ANZSIC, 
                edata.industry_name_ANZSIC, edata.variable);
        }
    }
}