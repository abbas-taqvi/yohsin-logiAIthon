use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
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
    /// Load data from a file into the struct
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        println!("{:?}", reader);

        let mut data = DailyBlotterData {
            orderdate: 0,
            ordertime: 0,
            accountnumber: String::new(),
            accountname: String::new(),
            traderid: String::new(),
            symbol: String::new(),
            ordercc: String::new(),
            orderit: String::new(),
            orderid: String::new(),
            orderidseq: String::new(),
            porderid: String::new(),
            action: String::new(),
            side: String::new(),
            qty: 0,
            maxfloor: 0,
            price: 0.0,
            type_: String::new(),
            dest: String::new(),
            qtyexec: 0,
            priceexec: 0.0,
            execmkt: String::new(),
            cumqty: 0,
            qtyleaves: 0,
            clorderid: String::new(),
            clorderidorig: String::new(),
            root: String::new(),
            exp: String::new(),
            strike: String::new(),
            ordercp: String::new(),
            clientid: String::new(),
            firmid: String::new(),
            poseff: String::new(),
            tradeid: String::new(),
            execid: String::new(),
            datasource: String::new(),
            datasubsource: String::new(),
            ext: String::new(),
            smp: String::new(),
            moi: String::new(),
            stopprice: 0.0,
            ordertext: String::new(),
            ordervo: String::new(),
            route: String::new(),
            ordertf: String::new(),
            issued: String::new(),
            imidrpt: String::new(),
            imidrcv: String::new(),
            dir: false,
            held: false,
            opid: String::new(),
            filename: String::new(),
            id: 0,
            tif: String::new(),
            isblotter: false,
            extclorderid: String::new(),
            trader_name: String::new(),
            created_date: 0,
        };

        let mut counter = 0;
        for line in reader.lines() {
            counter += 1;
            if counter == 1 {
                continue;
            }
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect(); // Assuming CSV format

            // Assign values to struct fields
            data.orderdate = i64::from_str(parts[0])?;
            data.ordertime = i64::from_str(parts[1])?;
            data.accountnumber = parts[2].to_string();
            data.accountname = parts[3].to_string();
            data.traderid = parts[4].to_string();
            data.symbol = parts[5].to_string();
            data.ordercc = parts[6].to_string();
            data.orderit = parts[7].to_string();
            data.orderid = parts[8].to_string();
            data.orderidseq = parts[9].to_string();
            data.porderid = parts[10].to_string();
            data.action = parts[11].to_string();
            data.side = parts[12].to_string();
            data.qty = i64::from_str(parts[13])?;
            data.maxfloor = i32::from_str(parts[14])?;
            data.price = f64::from_str(parts[15])?;
            data.type_ = parts[16].to_string();
            data.dest = parts[17].to_string();
            data.qtyexec = i64::from_str(parts[18])?;
            data.priceexec = f64::from_str(parts[19])?;
            data.execmkt = parts[20].to_string();
            data.cumqty = i32::from_str(parts[21])?;
            data.qtyleaves = i32::from_str(parts[22])?;
            data.clorderid = parts[23].to_string();
            data.clorderidorig = parts[24].to_string();
            data.root = parts[25].to_string();
            data.exp = parts[26].to_string();
            data.strike = parts[27].to_string();
            data.ordercp = parts[28].to_string();
            data.clientid = parts[29].to_string();
            data.firmid = parts[30].to_string();
            data.poseff = parts[31].to_string();
            data.tradeid = parts[32].to_string();
            data.execid = parts[33].to_string();
            data.datasource = parts[34].to_string();
            data.datasubsource = parts[35].to_string();
            data.ext = parts[36].to_string();
            data.smp = parts[37].to_string();
            data.moi = parts[38].to_string();
            data.stopprice = f64::from_str(parts[39])?;
            data.ordertext = parts[40].to_string();
            data.ordervo = parts[41].to_string();
            data.route = parts[42].to_string();
            data.ordertf = parts[43].to_string();
            data.issued = parts[44].to_string();
            data.imidrpt = parts[45].to_string();
            data.imidrcv = parts[46].to_string();
            data.dir = bool::from_str(parts[47])?;
            data.held = bool::from_str(parts[48])?;
            data.opid = parts[49].to_string();
            data.filename = parts[50].to_string();
            data.id = i64::from_str(parts[51])?;
            data.tif = parts[52].to_string();
            data.isblotter = bool::from_str(parts[53])?;
            data.extclorderid = parts[54].to_string();
            data.trader_name = parts[55].to_string();
            data.created_date = i64::from_str(parts[56])?;
        }

        Ok(data)
    }
}
