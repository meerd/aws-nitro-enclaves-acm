// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use log::{Level, LevelFilter, Log, Metadata, Record};

extern crate chrono;
use chrono::offset::Local;

pub struct Logger;

impl Logger {
    pub fn init() {
        let boxed_logger = Box::new(Self {});
        let res = log::set_boxed_logger(boxed_logger);
        match res {
            Ok(_) => log::set_max_level(LevelFilter::Trace),
            Err(_) => trace!("Logger already initialized"),
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        //metadata.level() <= Level::Error
        true
    }

    fn flush(&self) {}

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "{:8} {} | {}:{} {}",
                record.metadata().level(),
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            );
        }
    }
}
