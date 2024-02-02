//#![windows_subsystem = "windows"]

use rodio::source::Buffered;
use rodio::*;

use std::io;
use std::io::{Cursor, Write};

type BufferedFile = Buffered<Decoder<Cursor<Vec<u8>>>>;

pub struct Generator {
    sources: Vec<BufferedFile>,
    current_silence: BufferedFile,
    silence: BufferedFile,
    click: Vec<BufferedFile>,
    release: Vec<BufferedFile>,
    soft_click: Vec<BufferedFile>,
    soft_release: Vec<BufferedFile>,
    hard_click: Vec<BufferedFile>,
    hard_release: Vec<BufferedFile>,
    actions: Vec<(bool, u64)>,
    current_sample: u64,
    action_index: usize,
    click_index: usize,
    release_index: usize,
    last_click: Option<u64>,
    last_soft: bool,
    last_hard: bool,
    soft_time: f32,
    hard_time: f32,
    soft_click_index: usize,
    soft_release_index: usize,
    hard_click_index: usize,
    hard_release_index: usize,
}

impl Generator {
    #[link_section = ".code"]
    pub fn new(
        silence: BufferedFile,
        click: Vec<BufferedFile>,
        release: Vec<BufferedFile>,
        soft_click: Vec<BufferedFile>,
        soft_release: Vec<BufferedFile>,
        hard_click: Vec<BufferedFile>,
        hard_release: Vec<BufferedFile>,
        soft_time: f32,
        hard_time: f32,
        actions: Vec<(bool, u64)>,
    ) -> Self {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        let mut clicks_iter = click.iter();

        let first_click = clicks_iter.next().expect("Needs at least one click!");

        let sample_rate = first_click.sample_rate();

        let channels = first_click.channels();

        for c in clicks_iter {
            if c.sample_rate() != sample_rate {
                println!("Sample rates must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(c.sample_rate(), sample_rate);
            }

            if c.channels() != channels {
                println!("Channels must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(c.channels(), channels);
            }
        }

        let releases_iter = release.iter();

        for r in releases_iter {
            if r.sample_rate() != sample_rate {
                println!("Sample rates must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(r.sample_rate(), sample_rate);
            }

            if r.channels() != channels {
                println!("Channels must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(r.channels(), channels);
            }
        }

        let soft_clicks_iter = soft_click.iter();

        for sc in soft_clicks_iter {
            if sc.sample_rate() != sample_rate {
                println!("Sample rates must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(sc.sample_rate(), sample_rate);
            }

            if sc.channels() != channels {
                println!("Channels must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(sc.channels(), channels);
            }
        }

        let soft_releases_iter = soft_release.iter();

        for sr in soft_releases_iter {
            if sr.sample_rate() != sample_rate {
                println!("Sample rates must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(sr.sample_rate(), sample_rate);
            }

            if sr.channels() != channels {
                println!("Channels must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(sr.channels(), channels);
            }
        }

        let hard_clicks_iter = hard_click.iter();

        for hc in hard_clicks_iter {
            if hc.sample_rate() != sample_rate {
                println!("Sample rates must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(hc.sample_rate(), sample_rate);
            }

            if hc.channels() != channels {
                println!("Channels must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(hc.channels(), channels);
            }
        }

        let hard_releases_iter = hard_release.iter();

        for hr in hard_releases_iter {
            if hr.sample_rate() != sample_rate {
                println!("Sample rates must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(hr.sample_rate(), sample_rate);
            }

            if hr.channels() != channels {
                println!("Channels must be the same!");

                stdout.flush().unwrap();
                let mut wait = String::new();
                stdin.read_line(&mut wait).unwrap();

                assert_eq!(hr.channels(), channels);
            }
        }

        Self {
            sources: Vec::new(),
            current_silence: silence.clone(),
            silence,
            click,
            release,
            soft_click,
            soft_release,
            hard_click,
            hard_release,
            actions,
            last_click: None,
            last_soft: false,
            last_hard: false,
            soft_time,
            hard_time,
            current_sample: 0,
            action_index: 0,
            click_index: 0,
            release_index: 0,
            soft_click_index: 0,
            soft_release_index: 0,
            hard_click_index: 0,
            hard_release_index: 0,
        }
    }

    #[link_section = ".code"]
    fn next_silence(&mut self) -> i16 {
        if let Some(sample) = self.current_silence.next() {
            sample
        } else {
            self.current_silence = self.silence.clone();
            self.current_silence.next().unwrap_or(0)
        }
    }

    #[link_section = ".code"]
    pub fn from_clicks_and_releases(
        silence: BufferedFile,
        click: Vec<BufferedFile>,
        release: Vec<BufferedFile>,
        soft_click: Vec<BufferedFile>,
        soft_release: Vec<BufferedFile>,
        hard_click: Vec<BufferedFile>,
        hard_release: Vec<BufferedFile>,
        soft_time: f32,
        hard_time: f32,
        clicks: &[u64],
        releases: &[u64],
        fps: u32,
    ) -> Self {
        let mut actions = Vec::<(bool, u64)>::with_capacity(clicks.len() + releases.len());

        let sample_rate = silence.sample_rate();

        for click in clicks {
            actions.push((true, *click * sample_rate as u64 / fps as u64));
        }

        for release in releases {
            actions.push((false, *release * sample_rate as u64 / fps as u64));
        }

        actions.sort_by_key(|x| x.1);

        //println!("{:?}", actions);

        Self::new(
            silence,
            click,
            release,
            soft_click,
            soft_release,
            hard_click,
            hard_release,
            soft_time,
            hard_time,
            actions,
        )
    }

    #[link_section = ".code"]
    fn do_click(&mut self) {
        self.sources.push(self.click[self.click_index].clone());

        self.click_index = (self.click_index + 1) % self.click.len();
    }

    #[link_section = ".code"]
    fn do_release(&mut self) {
        self.sources.push(self.release[self.release_index].clone());

        self.release_index = (self.release_index + 1) % self.release.len();
    }

    #[link_section = ".code"]
    fn do_soft_click(&mut self) {
        self.sources
            .push(self.soft_click[self.soft_click_index].clone());

        self.soft_click_index = (self.soft_click_index + 1) % self.soft_click.len();
    }

    #[link_section = ".code"]
    fn do_soft_release(&mut self) {
        self.sources
            .push(self.soft_release[self.soft_release_index].clone());

        self.soft_release_index = (self.soft_release_index + 1) % self.soft_release.len();
    }

    #[link_section = ".code"]
    fn do_hard_click(&mut self) {
        self.sources
            .push(self.hard_click[self.hard_click_index].clone());

        self.hard_click_index = (self.hard_click_index + 1) % self.hard_click.len();
    }

    #[link_section = ".code"]
    fn do_hard_release(&mut self) {
        self.sources
            .push(self.hard_release[self.hard_release_index].clone());

        self.hard_release_index = (self.hard_release_index + 1) % self.hard_release.len();
    }

    #[link_section = ".code"]
    fn should_change(&mut self) -> Option<bool> {
        if let Some((click, target)) = self.actions.get(self.action_index) {
            if self.current_sample >= *target {
                self.action_index += 1;

                //println!("action index: {}\n actions length: {}\n sources length {}\n current sample {}\n target {}",
                //self.action_index, self.actions.len(), self.sources.len(), self.current_sample, target);

                return Some(*click);
            }
        }

        None
    }

    #[link_section = ".code"]
    fn should_do_soft(&self) -> bool {
        if let Some(l) = self.last_click {
            let diff = self.current_sample - l;

            diff < (self.soft_time * self.sample_rate() as f32) as u64
        } else {
            false
        }
    }

    #[link_section = ".code"]
    fn should_do_hard(&self) -> bool {
        if let Some(l) = self.last_click {
            let diff = self.current_sample - l;

            diff > (self.hard_time * self.sample_rate() as f32) as u64
        } else {
            false
        }
    }
}

impl Iterator for Generator {
    type Item = i16;

    #[link_section = ".code"]
    fn next(&mut self) -> Option<i16> {
        if self.action_index >= self.actions.len() && self.sources.is_empty() {
            return None;
        }

        let res;

        if let Some(click) = self.should_change() {
            if click {
                let is_soft = self.should_do_soft();
                let is_hard = self.should_do_hard();

                self.last_click = Some(self.current_sample);

                self.last_soft = is_soft;
                self.last_hard = is_hard;

                //print!("{}", is_soft);

                if is_soft {
                    self.do_soft_click();
                } else if is_hard {
                    self.do_hard_click();
                } else {
                    self.do_click();
                }
            } else {
                if self.last_soft {
                    self.do_soft_release()
                } else if self.last_hard {
                    self.do_hard_release()
                } else {
                    self.do_release();
                }
            }
        }

        if self.sources.is_empty() {
            res = Some(self.next_silence());
        } else {
            let mut removal_shift: usize = 0;

            let mut removals: Vec<usize> = Vec::with_capacity(self.sources.len());

            let mut result: i16 = 0;

            //println!("forloop start");

            for (i, source) in self.sources.iter_mut().enumerate() {
                if let Some(sample) = source.next() {
                    result = result.saturating_add(sample);

                    //println!("adding result: {} and sample: {}", result, sample);
                } else {
                    removals.push(i);
                }
            }

            //println!("forloop end");

            for r in removals {
                self.sources.remove(r - removal_shift);

                removal_shift += 1;
            }

            res = Some(result);
        }

        self.current_sample += 1;

        res
    }
}

impl Source for Generator {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.silence.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.silence.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
