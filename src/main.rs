extern crate clap;

use clap::App;
use clap::Arg;
use clap::SubCommand;

fn main() {
    let x: App = App::new("kafka-unified-cli").version("v0.1")
        .author("Stéphane D. <stephane@ixonad.com>")
        .about("A unified Kafka command line")
        .subcommand(SubCommand::with_name("topic")
            .about("View and modify Kafka topics")
            .arg(Arg::with_name("topic")
                .short("t")
                .help("The topic to inspect or modify"))
            .subcommand(SubCommand::with_name("create"))
            .subcommand(SubCommand::with_name("update"))
            .subcommand(SubCommand::with_name("consume"))
            .subcommand(SubCommand::with_name("produce")))
        .subcommand(SubCommand::with_name("broker")
            .about("View and modify Kafka brokers")
            .arg(Arg::with_name("brokers")
                .short("b")
                .help("The broker to inspect or modify"))
            .subcommand(SubCommand::with_name("modify")))
        .subcommand(SubCommand::with_name("consumergroup")
            .about("View and modify Kafka consumer groups")
            .arg(Arg::with_name("consumer-group")
                .short("cg")
                .help("The consumer-group to inspect or modify")));
    ;

    let _y = x.get_matches();
}
