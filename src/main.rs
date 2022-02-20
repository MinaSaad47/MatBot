use  matbot::{
    config::Config,
    materials::MatRow
};

fn main() {
    let conf = Config::from_json_file("settings.json");

    let conf = match conf {
        Ok(conf) => conf,
        Err(error) => panic!("{}", error)
    };

    println!("config:\n{:?}", conf);

    let table = MatRow::vec_from_database(&conf.database_path,
                                          &conf.material_types[0].0);
    let table = match table {
        Ok(table) => table,
        Err(error) => panic!("{}", error)
    };

    println!("table [{:?}]:\n{:?}",
             conf.material_types[0], table);
}
