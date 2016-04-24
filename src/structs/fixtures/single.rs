use DmxChannel;
use DmxValue;
use FadeCurve;
use FadeTime;
use std::sync::mpsc;
use get_fade_steps;
use FADE_TICKS;

use std::time::Duration;
use std::thread::sleep;

#[derive(Debug)]
pub struct Single {
    channel: DmxChannel,
    value: DmxValue,
    dmx_tx: mpsc::Sender<(DmxChannel, DmxValue)>
}

impl Single {
    pub fn new(channel: DmxChannel, dmx_tx: mpsc::Sender<(DmxChannel, DmxValue)>) -> Single {
        Single {
            channel: channel,
            value: 0,
            dmx_tx: dmx_tx
        }
    }
    pub fn fade(&mut self, curve: FadeCurve, time: FadeTime, end_value: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        for value in get_fade_steps(self.value, end_value, steps, curve) {
            println!("{:?}", value);
            sleep(Duration::from_millis((time/steps) as u64));
        }
    }
}