#[macro_use] extern crate log;
extern crate env_logger;
extern crate structures;
extern crate net2;

use std::time::Duration;
use std::thread::{self, sleep};
use std::collections::HashMap;

mod interface_handler;
use interface_handler::*;

use structures::*;

fn main() {
    env_logger::init().unwrap();

    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let (tx, _interrupt_tx) = interface.unwrap().to_thread();
    let mut stage = Parser::new(Stage::new(tx)).parse();
    //println!("{:?}", stage);


    let mut test_values1 = HashMap::new();
    test_values1.insert((0, 1), (vec![255], FadeCurve::Squared, 1000));
    // test_values1.insert((0, 0), (vec![255, 100, 100], FadeCurve::Squared, 1000));
    // test_values1.insert((2, 0), (vec![100, 50, 255], FadeCurve::Squared, 10000));
    let test_switch1 = ValueCollection::new(test_values1);
    let id1 = stage.add_switch(test_switch1);

    //let mut test_values2 = HashMap::new();
    //test_values1.insert((2, 0), (vec![255, 255, 255, 100], FadeCurve::Squared, 1000));
    //test_values2.insert((0, 1), (vec![255], FadeCurve::Squared, 1000));
    //let test_switch2 = ValueCollection::new(test_values2);
    //let id2 = stage.add_switch(test_switch2);

    stage.activate_switch(id1, 255.0);
    sleep(Duration::from_millis(2000));
    match stage.fixtures[0].channel_groups[1] {
        ChannelGroup::Single(ref mut group) => {
            group.activate_preheat(FadeCurve::Squared, 1000);
        },
        _ => {}
    }
    sleep(Duration::from_millis(2000));

    println!("pre");

    println!("on");
    stage.deactivate_switch(id1);
    sleep(Duration::from_millis(2000));

    match stage.fixtures[0].channel_groups[1] {
        ChannelGroup::Single(ref mut group) => {
            group.deactivate_preheat(FadeCurve::Squared, 1000);
        },
        _ => {}
    }




    let socket = UDPSocket::new();
    socket.start_watchdog_server();
    let server = socket.start_backend_server(); //receiving updates (DMX values etc. from frontend)
    // let stage = Stage::new(tx);




    thread::spawn(move || {
        loop {
            let (d, _) = server.receive();
            debug!("{:?}", d); //TODO: do something with the data that isn't completely useless

            let address_type:u8 = d[0];
            let address: u16 = ((d[1] as u16) << 8) + (d[2] as u16);
            let value: u8 = d[3];

            if address_type == 0 {
                {stage.channels[address as usize].lock().unwrap().set(value);}
            }
            else if address_type == 1 {
                // Scene
            }
            else if address_type == 2 {
                // Switch
            }
            println!("{:?}, {:?}", address, value);



            //stage.fixtures.push();
            server.send_to_multicast(&d);
        }
    });

    thread::spawn(|| {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
        info!("listening started, ready to accept");
        for stream in listener.incoming() {
            thread::spawn(|| {
                let mut stream = stream.unwrap();
                stream.write(b"Hello World\r\n").unwrap();
            });
        }
    }).join().unwrap();
}


#[test]
fn test_fade_curve() {
    use std::time::Duration;
    use std::thread::sleep;
    use interface_handler::*;
    use structures::*;

    let interface = Interface::new().connect();
    if interface.is_err() { panic!(interface) }
    let (tx, interrupt_tx) = interface.unwrap().to_thread();

    //let curve = FadeCurve::Custom("-cos(1.5*6.28318530718*x)*0.5+0.5".to_string());
    let curve = FadeCurve::Squared;
    let curve = FadeCurve::Sin(0);
    let mut stage = Stage::new(tx);


    let mut test_group = ChannelGroup::Single(Single::new(stage.get_channel_object(1)));
    // let mut test_group = ChannelGroup::Moving2D(Moving2D::new(stage.get_channel_object(1), stage.get_channel_object(2)));
    // let mut test_group = ChannelGroup::RGB(RGB::new(stage.get_channel_object(1), stage.get_channel_object(2), stage.get_channel_object(3)));
    //let test_group = ChannelGroup::RGBA(RGBA::new(stage.get_channel_object(1), stage.get_channel_object(2), stage.get_channel_object(3), stage.get_channel_object(4)));

    match test_group {
        ChannelGroup::Single(mut group) => {
            //group.fade_simple(curve.clone(), 500, 255);
            //sleep(Duration::from_millis(2000));
            //group.activate_preheat(curve.clone(), 500);
            //println!("pre");
            group.fade_simple(curve.clone(), 5000, 255);
            //group.deactivate_preheat(curve.clone(), 500);
            //sleep(Duration::from_millis(2000));
            //group.fade_simple(curve.clone(), 1000, 0);
        },
        ChannelGroup::RGB(mut group) => {
            group.fade_rgb(curve.clone(), 1000, 255, 0, 0);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 255, 0);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 0, 255);
            sleep(Duration::from_millis(1000));
        },
        ChannelGroup::RGBA(mut group) => {
            group.fade_rgb(curve.clone(), 1000, 255, 0, 0, 1);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 255, 0, 1);
            sleep(Duration::from_millis(1000));
            group.fade_rgb(curve.clone(), 1000, 0, 0, 255, 1);
            sleep(Duration::from_millis(1000));
        },
        ChannelGroup::Moving2D(mut group) => {
            group.fade_simple(curve.clone(), 1000, 255, 255);
            sleep(Duration::from_millis(1000));
            group.fade_simple(curve.clone(), 1000, 0, 0);
        }//,
        //_ => {}
    }


    sleep(Duration::from_millis(500));
    println!("Disconnecting...");
    interrupt_tx.send(true).unwrap();
}
