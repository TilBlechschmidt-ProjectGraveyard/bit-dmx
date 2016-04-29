use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{self, sleep};
use std::sync::mpsc;

use DmxValue;
use FadeTime;
use FADE_TICKS;
use ChannelGroupValue;

use Channel;
use FadeCurve;

// use rgb_to_hsv;
// use hsv_to_rgb;
//
// use get_fade_steps;
use get_fade_steps_int;
use stop_fade;

#[derive(Debug)]
pub struct RGBA{
    channel_r: Arc<Mutex<Channel>>,
    channel_g: Arc<Mutex<Channel>>,
    channel_b: Arc<Mutex<Channel>>,
    channel_a: Arc<Mutex<Channel>>,
    pub active_switches: Vec<(usize, ChannelGroupValue)>
}

impl RGBA {
    pub fn new(channel_r: Arc<Mutex<Channel>>, channel_g: Arc<Mutex<Channel>>, channel_b: Arc<Mutex<Channel>>, channel_a: Arc<Mutex<Channel>>) -> RGBA {
        RGBA{
            channel_r: channel_r,
            channel_g: channel_g,
            channel_b: channel_b,
            channel_a: channel_a,
            active_switches: Vec::new()
        }
    }

    // pub fn fade_rgb(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, end_a: DmxValue) {
    //     let steps = time*FADE_TICKS/1000;
    //     let (tx, rx) = mpsc::channel();
    //     let channel_r = self.channel_r.clone();
    //     let channel_g = self.channel_g.clone();
    //     let channel_b = self.channel_b.clone();
    //     let channel_a = self.channel_a.clone();
    //     stop_fade(channel_r.clone(), tx.clone());
    //     stop_fade(channel_g.clone(), tx.clone());
    //     stop_fade(channel_b.clone(), tx.clone());
    //     stop_fade(channel_a.clone(), tx.clone());
    //
    //     thread::spawn(move || {
    //         let mut channel_r_locked = channel_r.lock().unwrap();
    //         let mut channel_g_locked = channel_g.lock().unwrap();
    //         let mut channel_b_locked = channel_b.lock().unwrap();
    //         let mut channel_a_locked = channel_a.lock().unwrap();
    //         let (start_h, start_s, start_v) = rgb_to_hsv(channel_r_locked.get(), channel_g_locked.get(), channel_b_locked.get());
    //         let start_a = channel_a_locked.get();
    //         let (end_h, end_s, end_v) = rgb_to_hsv(end_r, end_g, end_b);
    //         for (((&h, &s), &v), &a) in get_fade_steps(start_h, end_h, steps, curve.clone()).iter().zip(get_fade_steps(start_s, end_s, steps, curve.clone()).iter()).zip(get_fade_steps(start_v, end_v, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_a, end_a, steps, curve.clone()).iter()) {
    //             let (r, g, b) = hsv_to_rgb(h, s, v);
    //             channel_r_locked.set(r);
    //             channel_g_locked.set(g);
    //             channel_b_locked.set(b);
    //             channel_a_locked.set(a);
    //             sleep(Duration::from_millis((time/steps) as u64));
    //         }
    //     });
    // }

    pub fn fade_simple(&mut self, curve: FadeCurve, time: FadeTime, end_r: DmxValue, end_g: DmxValue, end_b: DmxValue, end_a: DmxValue) {
        let steps = time*FADE_TICKS/1000;
        let (tx, rx) = mpsc::channel();
        let channel_r = self.channel_r.clone();
        let channel_g = self.channel_g.clone();
        let channel_b = self.channel_b.clone();
        let channel_a = self.channel_a.clone();
        stop_fade(channel_r.clone(), tx.clone());
        stop_fade(channel_g.clone(), tx.clone());
        stop_fade(channel_b.clone(), tx.clone());
        stop_fade(channel_a.clone(), tx.clone());


        thread::spawn(move || {
            let start_r = channel_r.lock().unwrap().get();
            let start_g = channel_g.lock().unwrap().get();
            let start_b = channel_b.lock().unwrap().get();
            let start_a = channel_a.lock().unwrap().get();
            for (((&r, &g), &b), &a) in get_fade_steps_int(start_r, end_r, steps, curve.clone()).iter().zip(get_fade_steps_int(start_g, end_g, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_b, end_b, steps, curve.clone()).iter()).zip(get_fade_steps_int(start_a, end_a, steps, curve.clone()).iter()) {
                {
                    if rx.try_recv().is_ok() { return }
                    channel_r.lock().unwrap().set(r);
                    channel_g.lock().unwrap().set(g);
                    channel_b.lock().unwrap().set(b);
                    channel_a.lock().unwrap().set(a);
                }
                sleep(Duration::from_millis((time/steps) as u64));
            }
        });
    }
}
