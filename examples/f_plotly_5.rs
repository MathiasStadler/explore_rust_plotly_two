#![allow(dead_code)]

// use std::env;
use std::path::PathBuf;

use plotly::common::TickFormatStop;
use plotly::layout::{Axis, RangeSelector, RangeSlider, SelectorButton, SelectorStep, StepMode};
use plotly::{Candlestick, Layout, Ohlc, Plot, Scatter};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct FinData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    // adjusted: f64,
    // dn: f64,
    // mavg: f64,
    // up: f64,
    // direction: String,
}

fn load_apple_data() -> Vec<FinData> {
    // let mut p = PathBuf::from("./assets/");
    ///home/user/workspace_rust/explore_rust_plotly/examples/assets
    let mut p = PathBuf::from("./assets");

    // p = p.join("finance_charts_apple.csv");
    // examples/assets/output_sma_7.csv
    p = p.join("output_sma_7.csv");

    println!("Path => {}", p.display());
    // let mut p = PathBuf::from("./assets").unwrap();
    let mut rdr = csv::Reader::from_path(p).unwrap();
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let d: FinData = result.unwrap();
        out.push(d);
    }

    out
}

fn time_series_with_range_slider() {
    let data = load_apple_data();
    let date: Vec<String> = data.iter().map(|d| d.date.clone()).collect();
    let high: Vec<f64> = data.iter().map(|d| d.high).collect();

    let trace = Scatter::new(date, high);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
        .title("Manually Set Date Range");
    plot.set_layout(layout);

    //plot.show();
    plot.write_html("out_ohcl.html");
}

fn main() {time_series_with_range_slider() 
    time_series_with_range_slider();
}

// cargo run --example f_plotly_4
