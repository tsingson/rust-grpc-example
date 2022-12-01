use chrono::{DateTime, Local};
use tokio;
use tokio_postgres::NoTls;

use crate::database::db::DB;
use crate::models::DataDevice;

pub async fn getbydevice(_device: String) -> Result<DataDevice, tokio_postgres::Error> {
    let (client, connection) =
            tokio_postgres::connect(DB::url().url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut datas: DataDevice = DataDevice { id: 0, device: "".to_string(), value: "".to_string(), date: "".to_string() };
    let mut device: String;
    let mut value: String;
    let mut date: DateTime<Local>;
    let rows = client.query("select * from datas where device = $1", &[&_device]).await?;
    for row in rows {
        device = row.get(1);
        value = row.get(2);
        date = row.get(3);

        datas.device = device;
        datas.date = date.format("%Y-%m-%d %H:%M:%S").to_string();
        datas.value = value;
    }
    Ok(datas)
}

