use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct DailyBlotterData {
    pub orderdate: i64,
    pub ordertime: i64,
    pub accountnumber: String,
    pub accountname: String,
    pub traderid: String,
    pub symbol: String,
    pub ordercc: String,
    pub orderit: String,
    pub orderid: String,
    pub orderidseq: String,
    pub porderid: String,
    pub action: String,
    pub side: String,
    pub qty: i64,
    pub maxfloor: i32,
    pub price: f64,
    pub type_: String,
    pub dest: String,
    pub qtyexec: i64,
    pub priceexec: f64,
    pub execmkt: String,
    pub cumqty: i32,
    pub qtyleaves: i32,
    pub clorderid: String,
    pub clorderidorig: String,
    pub root: String,
    pub exp: String,
    pub strike: String,
    pub ordercp: String,
    pub clientid: String,
    pub firmid: String,
    pub poseff: String,
    pub tradeid: String,
    pub execid: String,
    pub datasource: String,
    pub datasubsource: String,
    pub ext: String,
    pub smp: String,
    pub moi: String,
    pub stopprice: f64,
    pub ordertext: String,
    pub ordervo: String,
    pub route: String,
    pub ordertf: String,
    pub issued: String,
    pub imidrpt: String,
    pub imidrcv: String,
    pub dir: bool,
    pub held: bool,
    pub opid: String,
    pub filename: String,
    pub id: i64,
    pub tif: String,
    pub isblotter: bool,
    pub extclorderid: String,
    pub trader_name: String,
    pub created_date: i64,
}

impl DailyBlotterData {
    /// Load data from a file into a vector of DailyBlotterData structs
    pub fn load_from_file(file_path: &str) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut data_list = Vec::new(); // Vector to store all DailyBlotterData structs

        for line in reader.lines().skip(1) {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect(); // Assuming CSV format

            // Create a new DailyBlotterData struct for each line
            let data = DailyBlotterData {
                orderdate: i64::from_str(parts[0])?,
                ordertime: i64::from_str(parts[1])?,
                accountnumber: parts[2].to_string(),
                accountname: parts[3].to_string(),
                traderid: parts[4].to_string(),
                symbol: parts[5].to_string(),
                ordercc: parts[6].to_string(),
                orderit: parts[7].to_string(),
                orderid: parts[8].to_string(),
                orderidseq: parts[9].to_string(),
                porderid: parts[10].to_string(),
                action: parts[11].to_string(),
                side: parts[12].to_string(),
                qty: i64::from_str(parts[13])?,
                maxfloor: i32::from_str(parts[14])?,
                price: f64::from_str(parts[15])?,
                type_: parts[16].to_string(),
                dest: parts[17].to_string(),
                qtyexec: i64::from_str(parts[18])?,
                priceexec: f64::from_str(parts[19])?,
                execmkt: parts[20].to_string(),
                cumqty: i32::from_str(parts[21])?,
                qtyleaves: i32::from_str(parts[22])?,
                clorderid: parts[23].to_string(),
                clorderidorig: parts[24].to_string(),
                root: parts[25].to_string(),
                exp: parts[26].to_string(),
                strike: parts[27].to_string(),
                ordercp: parts[28].to_string(),
                clientid: parts[29].to_string(),
                firmid: parts[30].to_string(),
                poseff: parts[31].to_string(),
                tradeid: parts[32].to_string(),
                execid: parts[33].to_string(),
                datasource: parts[34].to_string(),
                datasubsource: parts[35].to_string(),
                ext: parts[36].to_string(),
                smp: parts[37].to_string(),
                moi: parts[38].to_string(),
                stopprice: f64::from_str(parts[39])?,
                ordertext: parts[40].to_string(),
                ordervo: parts[41].to_string(),
                route: parts[42].to_string(),
                ordertf: parts[43].to_string(),
                issued: parts[44].to_string(),
                imidrpt: parts[45].to_string(),
                imidrcv: parts[46].to_string(),
                dir: bool::from_str(parts[47])?,
                held: bool::from_str(parts[48])?,
                opid: parts[49].to_string(),
                filename: parts[50].to_string(),
                id: i64::from_str(parts[51])?,
                tif: parts[52].to_string(),
                isblotter: bool::from_str(parts[53])?,
                extclorderid: parts[54].to_string(),
                trader_name: parts[55].to_string(),
                created_date: i64::from_str(parts[56])?,
            };

            // Push the struct into the vector
            data_list.push(data);
        }

        Ok(data_list)
    }
}
