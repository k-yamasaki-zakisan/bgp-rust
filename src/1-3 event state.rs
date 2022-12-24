use rand::Rng;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum State {
    PowerOn,
    PowerOff,
}

#[derive(Debug)]
enum Event {
    PushedPowerButton,
    PushedVolumeIncreaseButton,
    PushedVolumeDecreaseButton,
}

struct TV {
    now_state: State,
    event_queue: EventQueue,
    volume: u8,
}

impl TV {
    pub fn new() -> Self {
        let now_state = State::PowerOff;
        let event_queue = EventQueue::new();
        let volume = 10;
        Self {
            now_state,
            event_queue,
            volume,
        }
    }

    pub fn be_pushed_power_button(&mut self) {
        self.event_queue.enqueue(Event::PushedPowerButton);
    }

    pub fn be_pushed_volume_increase_button(&mut self) {
        self.event_queue.enqueue(Event::PushedVolumeIncreaseButton);
    }

    pub fn be_pushed_volume_decrease_button(&mut self) {
        self.event_queue.enqueue(Event::PushedVolumeDecreaseButton);
    }

    pub fn handle_event(&mut self, event: Event) {
        match &self.now_state {
            &State::PowerOn => match event {
                Event::PushedPowerButton => {
                    self.now_state = State::PowerOff;
                }
                Event::PushedVolumeIncreaseButton => {
                    self.volume += 1;
                }
                Event::PushedVolumeDecreaseButton => {
                    self.volume -= 1;
                }
            },
            &State::PowerOff => match event {
                Event::PushedVolumeIncreaseButton => {
                    self.now_state = State::PowerOn;
                }
                _ => (),
            },
        }
    }
}

struct EventQueue(VecDeque<Event>);

impl EventQueue {
    pub fn new() -> Self {
        let d = VecDeque::new();
        EventQueue(d)
    }

    pub fn dequeue(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    pub fn enqueue(&mut self, event: Event) {
        self.0.push_back(event);
    }
}

fn push_random_button_of_tv(tv: &mut TV) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        1 => tv.be_pushed_power_button(),
        2 => tv.be_pushed_volume_increase_button(),
        3 => tv.be_pushed_volume_decrease_button(),
        _ => (),
    }
}

fn main() {
    let mut tv = TV::new();
    tv.be_pushed_power_button();
    loop {
        push_random_button_of_tv(&mut tv);
        if let Some(event) = tv.event_queue.dequeue() {
            println!(
                "tvinformation:{{now_state={:?},volume={}}}\n\
                input_event:{:?}",
                tv.now_state, tv.volume, event
            );
            tv.handle_event(event);
        }
        thread::sleep(Duration::from_secs(2));
    }
}
