use std::io;

use crate::view::view_util::{show_header1, show_header2};
use crate::view::game_view;
use crate::domain_model::reversi_state::ReversiState;
use crate::service::reversi_service;

pub fn show() {
    show_header1("リバーシ");

    // タイトル画面のメインループ
    loop {
        show_header2("メニュー");
        println!("1. 新規ゲーム開始");
        println!("2. つづきから");
        println!("9. 終了");

        let error_message = "1～9を入力してください";
        // 入力ループ
        'input_loop: loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            let selection: i32 = match user_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("{}", error_message);
                    continue;
                }
            };

            match selection {
                1 => {
                    println!("新規ゲームを開始します");
                    let mut state = ReversiState::new();
                    game_view::show(&mut state);
                    break 'input_loop;
                },
                2 => {
                    println!("前回の状態をロードします");
                    match reversi_service::load() {
                        Ok(state) => { 
                            println!("ロードに成功しました"); 
                            let mut s = state;
                            game_view::show(&mut s);
                            break 'input_loop;
                        },
                        Err(e) => { 
                            println!("ロードに失敗しました: {}", e.message) 
                        }
                    }
                },
                9 => {
                    println!("アプリを終了します");
                    std::process::exit(0); // TODO: ここでいきなりexitっていいのかね？
                },
                _ => {
                    println!("{}", error_message);
                }
            }
        }
    }
}