use flatbuffers::FlatBufferBuilder;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use yohsin::order_generated::daily_blotter_life_cycle::{DailyBlotter, DailyBlotterArgs};
use yohsin::order_struct::DailyBlotterData;

mod order_generated;
mod order_struct;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data from a file into the struct
    let data = DailyBlotterData::load_from_file("../dataBaker/data/dummy_data_.csv")?;

    // Serialize the struct to a file
    let serialize_time = serialize_daily_blotter_to_file("data.bin", &data)?;
    println!("Serialization took: {:?}", serialize_time);

    // Deserialize the struct from the file
    let deserialize_time = deserialize_daily_blotter_from_file("data.bin")?;
    println!("Deserialization took: {:?}", deserialize_time);

    println!("Serialization and deserialization complete!");
    Ok(())
}

fn serialize_daily_blotter_to_file(
    file_path: &str,
    data: &DailyBlotterData,
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    let mut builder = FlatBufferBuilder::new();

    // Start timing serialization
    let start = Instant::now();

    // Create strings for all string fields
    let accountnumber_offset = builder.create_string(&data.accountnumber);
    let accountname_offset = builder.create_string(&data.accountname);
    let traderid_offset = builder.create_string(&data.traderid);
    let symbol_offset = builder.create_string(&data.symbol);
    let ordercc_offset = builder.create_string(&data.ordercc);
    let orderit_offset = builder.create_string(&data.orderit);
    let orderid_offset = builder.create_string(&data.orderid);
    let orderidseq_offset = builder.create_string(&data.orderidseq);
    let porderid_offset = builder.create_string(&data.porderid);
    let action_offset = builder.create_string(&data.action);
    let side_offset = builder.create_string(&data.side);
    let type_offset = builder.create_string(&data.type_);
    let dest_offset = builder.create_string(&data.dest);
    let execmkt_offset = builder.create_string(&data.execmkt);
    let clorderid_offset = builder.create_string(&data.clorderid);
    let clorderidorig_offset = builder.create_string(&data.clorderidorig);
    let root_offset = builder.create_string(&data.root);
    let exp_offset = builder.create_string(&data.exp);
    let strike_offset = builder.create_string(&data.strike);
    let ordercp_offset = builder.create_string(&data.ordercp);
    let clientid_offset = builder.create_string(&data.clientid);
    let firmid_offset = builder.create_string(&data.firmid);
    let poseff_offset = builder.create_string(&data.poseff);
    let tradeid_offset = builder.create_string(&data.tradeid);
    let execid_offset = builder.create_string(&data.execid);
    let datasource_offset = builder.create_string(&data.datasource);
    let datasubsource_offset = builder.create_string(&data.datasubsource);
    let ext_offset = builder.create_string(&data.ext);
    let smp_offset = builder.create_string(&data.smp);
    let moi_offset = builder.create_string(&data.moi);
    let ordertext_offset = builder.create_string(&data.ordertext);
    let ordervo_offset = builder.create_string(&data.ordervo);
    let route_offset = builder.create_string(&data.route);
    let ordertf_offset = builder.create_string(&data.ordertf);
    let issued_offset = builder.create_string(&data.issued);
    let imidrpt_offset = builder.create_string(&data.imidrpt);
    let imidrcv_offset = builder.create_string(&data.imidrcv);
    let opid_offset = builder.create_string(&data.opid);
    let filename_offset = builder.create_string(&data.filename);
    let tif_offset = builder.create_string(&data.tif);
    let extclorderid_offset = builder.create_string(&data.extclorderid);
    let trader_name_offset = builder.create_string(&data.trader_name);

    // Build the DailyBlotter object
    let blotter = DailyBlotter::create(
        &mut builder,
        &DailyBlotterArgs {
            orderdate: data.orderdate,
            ordertime: data.ordertime,
            accountnumber: Some(accountnumber_offset),
            accountname: Some(accountname_offset),
            traderid: Some(traderid_offset),
            symbol: Some(symbol_offset),
            ordercc: Some(ordercc_offset),
            orderit: Some(orderit_offset),
            orderid: Some(orderid_offset),
            orderidseq: Some(orderidseq_offset),
            porderid: Some(porderid_offset),
            action: Some(action_offset),
            side: Some(side_offset),
            qty: data.qty,
            maxfloor: data.maxfloor,
            price: data.price,
            type_: Some(type_offset),
            dest: Some(dest_offset),
            qtyexec: data.qtyexec,
            priceexec: data.priceexec,
            execmkt: Some(execmkt_offset),
            cumqty: data.cumqty,
            qtyleaves: data.qtyleaves,
            clorderid: Some(clorderid_offset),
            clorderidorig: Some(clorderidorig_offset),
            root: Some(root_offset),
            exp: Some(exp_offset),
            strike: Some(strike_offset),
            ordercp: Some(ordercp_offset),
            clientid: Some(clientid_offset),
            firmid: Some(firmid_offset),
            poseff: Some(poseff_offset),
            tradeid: Some(tradeid_offset),
            execid: Some(execid_offset),
            datasource: Some(datasource_offset),
            datasubsource: Some(datasubsource_offset),
            ext: Some(ext_offset),
            smp: Some(smp_offset),
            moi: Some(moi_offset),
            stopprice: data.stopprice,
            ordertext: Some(ordertext_offset),
            ordervo: Some(ordervo_offset),
            route: Some(route_offset),
            ordertf: Some(ordertf_offset),
            issued: Some(issued_offset),
            imidrpt: Some(imidrpt_offset),
            imidrcv: Some(imidrcv_offset),
            dir: data.dir,
            held: data.held,
            opid: Some(opid_offset),
            filename: Some(filename_offset),
            id: data.id,
            tif: Some(tif_offset),
            isblotter: data.isblotter,
            extclorderid: Some(extclorderid_offset),
            trader_name: Some(trader_name_offset),
            created_date: data.created_date,
        },
    );

    // Finish the buffer
    builder.finish(blotter, None);

    // End timing serialization
    let serialize_duration = start.elapsed();

    // Write the serialized buffer to a file
    let serialized_data = builder.finished_data();
    let mut file = File::create(file_path)?;
    file.write_all(serialized_data)?;

    Ok(serialize_duration)
}

fn deserialize_daily_blotter_from_file(
    file_path: &str,
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    use flatbuffers::root;
    use memmap2::MmapOptions;
    use std::fs::File;

    // Open the file
    let file = File::open(file_path)?;

    // Memory-map the file
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    // Start timing deserialization
    let start = Instant::now();

    // Deserialize the FlatBuffer
    let blotter = root::<DailyBlotter>(&mmap[..])?;

    // End timing deserialization
    let deserialize_duration = start.elapsed();

    // Access fields
    println!("Order Date: {}", blotter.orderdate());
    println!("Account Number: {}", blotter.accountnumber().unwrap());
    println!("Symbol: {}", blotter.symbol().unwrap());
    println!("Quantity: {}", blotter.qty());
    println!("Price: {}", blotter.price());

    Ok(deserialize_duration)
}
