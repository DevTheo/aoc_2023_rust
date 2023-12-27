// Uncomment the following to hide the logs
//#![windows_subsystem = "windows"]

use nwd::NwgUi;
use nwg::NativeUi;
use crate::days::*;

#[macro_use]extern crate log;

pub mod days;

#[derive(Default, NwgUi)]
pub struct AoCApp {
    #[nwg_control(size: (300, 315), position: (300, 300), title: "AoC Guid", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [AoCApp::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Day1")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_day01] )]
    day01_button: nwg::Button,

    #[nwg_control(text: "Day2")]
    #[nwg_layout_item(layout: grid, col: 1, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_day02] )]
    day02_button: nwg::Button,

    #[nwg_control(text: "Day3")]
    #[nwg_layout_item(layout: grid, col: 2, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_day03] )]
    day03_button: nwg::Button,

    #[nwg_control(text: "Day4")]
    #[nwg_layout_item(layout: grid, col: 3, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day04_button: nwg::Button,

    #[nwg_control(text: "Day5")]
    #[nwg_layout_item(layout: grid, col: 4, row: 0, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day05_button: nwg::Button,

    #[nwg_control(text: "Day6")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day06_button: nwg::Button,

    #[nwg_control(text: "Day7")]
    #[nwg_layout_item(layout: grid, col: 1, row: 1, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day07_button: nwg::Button,

    #[nwg_control(text: "Day8")]
    #[nwg_layout_item(layout: grid, col: 2, row: 1, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day08_button: nwg::Button,

    #[nwg_control(text: "Day9")]
    #[nwg_layout_item(layout: grid, col: 3, row: 1, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day09_button: nwg::Button,

    #[nwg_control(text: "Day10")]
    #[nwg_layout_item(layout: grid, col: 4, row: 1, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day10_button: nwg::Button,

    #[nwg_control(text: "Day11")]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day11_button: nwg::Button,

    #[nwg_control(text: "Day12")]
    #[nwg_layout_item(layout: grid, col: 1, row: 2, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day12_button: nwg::Button,

    #[nwg_control(text: "Day13")]
    #[nwg_layout_item(layout: grid, col: 2, row: 2, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day13_button: nwg::Button,

    #[nwg_control(text: "Day14")]
    #[nwg_layout_item(layout: grid, col: 3, row: 2, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day14_button: nwg::Button,

    #[nwg_control(text: "Day15")]
    #[nwg_layout_item(layout: grid, col: 4, row: 2, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day15_button: nwg::Button,

    #[nwg_control(text: "Day16")]
    #[nwg_layout_item(layout: grid, col: 0, row: 3, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day16_button: nwg::Button,

    #[nwg_control(text: "Day17")]
    #[nwg_layout_item(layout: grid, col: 1, row: 3, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day17_button: nwg::Button,

    #[nwg_control(text: "Day18")]
    #[nwg_layout_item(layout: grid, col: 2, row: 3, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day18_button: nwg::Button,

    #[nwg_control(text: "Day19")]
    #[nwg_layout_item(layout: grid, col: 3, row: 3, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day19_button: nwg::Button,

    #[nwg_control(text: "Day20")]
    #[nwg_layout_item(layout: grid, col: 4, row: 3, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day20_button: nwg::Button,

    #[nwg_control(text: "Day21")]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day21_button: nwg::Button,

    #[nwg_control(text: "Day22")]
    #[nwg_layout_item(layout: grid, col: 1, row: 4, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day22_button: nwg::Button,

    #[nwg_control(text: "Day23")]
    #[nwg_layout_item(layout: grid, col: 2, row: 4, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day23_button: nwg::Button,

    #[nwg_control(text: "Day24")]
    #[nwg_layout_item(layout: grid, col: 3, row: 4, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day24_button: nwg::Button,

    #[nwg_control(text: "Day25")]
    #[nwg_layout_item(layout: grid, col: 4, row: 4, row_span: 1)]
    #[nwg_events( OnButtonClick: [AoCApp::exec_nada] )]
    day25_button: nwg::Button


}

impl AoCApp {

    fn exec_day01(&self) {
        let result = day01::exec();
        nwg::modal_info_message(&self.window, "Day01", &format!("result: {}", result));
    }

    fn exec_day02(&self) {
        let result = day02::exec();
        nwg::modal_info_message(&self.window, "Day02", &format!("result: {}", result));
    }

    fn exec_day03(&self) {
        let result = day03::exec();
        nwg::modal_info_message(&self.window, "Day03", &format!("result: {}", result));
    }

    fn exec_nada(&self) {
        nwg::modal_info_message(&self.window, "GNDN", &format!("Nothing to see here"));
    }
    
    
    fn exit(&self) {
    //     nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
         nwg::stop_thread_dispatch();
    }

}

fn main() {
    sensible_env_logger::init!();

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = AoCApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}