use rand::{distributions::Alphanumeric, Rng};
use std::{fs::File, io::Write};
use chrono::{Utc, Local};
use indicatif::{ProgressBar, ProgressStyle};

const NUM_RECORDS: usize = 4_500_000; // Adjusted to approximately 2.25GB

fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn generate_dummy_data(file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    // Write the header row
    writeln!(file, "orderdate_epoch,ordertime_epoch,accountnumber,accountname,traderid,symbol,ordercc,orderit,orderid,orderidseq,porderid,action,side,qty,maxfloor,price,type,dest,qtyexec,priceexec,execmkt,cumqty,qtyleaves,clorderid,clorderidorig,root,exp,strike,ordercp,clientid,firmid,poseff,tradeid,execid,datasource,datasubsource,ext,smp,moi,stopprice,ordertext,ordervo,route,ordertf,issued,imidrpt,imidrcv,dir,held,opid,filename,id,tif,isblotter,extclorderid,trader_name,created_date_epoch")?;

    // Initialize progress bar
    let pb = ProgressBar::new(NUM_RECORDS as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    for id in 1..=NUM_RECORDS {
        let now = Utc::now();
        let epoch_seconds = now.timestamp();
        let local_time = Local::now();
        let local_epoch_seconds = local_time.timestamp();

        let record = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.2},{},{},{},{:.2},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            epoch_seconds,                                     // orderdate_epoch
            local_epoch_seconds,                              // ordertime_epoch
            random_string(10),                                // accountnumber
            random_string(10),                                // accountname
            random_string(10),                                // traderid
            random_string(5),                                 // symbol
            random_string(3),                                 // ordercc
            random_string(3),                                 // orderit
            random_string(12),                                // orderid
            random_string(12),                                // orderidseq
            random_string(12),                                // porderid
            "BUY",                                           // action
            "SELL",                                          // side
            rand::thread_rng().gen_range(1..=10_000),         // qty
            rand::thread_rng().gen_range(1..=1_000),          // maxfloor
            rand::thread_rng().gen_range(1.0..=100.0),        // price
            "LIMIT",                                         // type
            "NYSE",                                          // dest
            rand::thread_rng().gen_range(1..=10_000),         // qtyexec
            rand::thread_rng().gen_range(1.0..=100.0),        // priceexec
            random_string(5),                                 // execmkt
            rand::thread_rng().gen_range(1..=10_000),         // cumqty
            rand::thread_rng().gen_range(1..=10_000),         // qtyleaves
            random_string(15),                                // clorderid
            random_string(15),                                // clorderidorig
            random_string(5),                                 // root
            "2025-01",                                      // exp
            "50",                                           // strike
            random_string(5),                                 // ordercp
            random_string(10),                                // clientid
            random_string(10),                                // firmid
            "POS",                                          // poseff
            random_string(10),                                // tradeid
            random_string(10),                                // execid
            "DATA_SOURCE",                                  // datasource
            "SUBSOURCE",                                    // datasubsource
            random_string(5),                                 // ext
            random_string(5),                                 // smp
            random_string(5),                                 // moi
            rand::thread_rng().gen_range(1.0..=100.0),        // stopprice
            random_string(50),                                // ordertext
            random_string(5),                                 // ordervo
            "ROUTE",                                        // route
            random_string(5),                                 // ordertf
            random_string(10),                                // issued
            random_string(10),                                // imidrpt
            random_string(10),                                // imidrcv
            rand::thread_rng().gen_bool(0.5),                 // dir
            rand::thread_rng().gen_bool(0.5),                 // held
            random_string(10),                                // opid
            random_string(10),                                // filename
            id,                                              // id
            "GTC",                                          // tif
            rand::thread_rng().gen_bool(0.5),                 // isblotter
            random_string(12),                                // extclorderid
            random_string(10),                                // trader_name
            epoch_seconds                                     // created_date_epoch
        );

        file.write_all(record.as_bytes())?;
        file.write_all(b"\n")?; // Add newline after each record

        pb.inc(1); // Increment the progress bar
    }

    pb.finish_with_message("Data generation complete.");
    Ok(())
}

fn main() {
    let file_path = "dummy_data_.csv";

    match generate_dummy_data(file_path) {
        Ok(_) => println!("Dummy data successfully written to {}", file_path),
        Err(e) => eprintln!("Error generating dummy data: {}", e),
    }
}
