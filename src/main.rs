use std::collections::HashMap;
use std::io;
fn main() {
    let mut numbers: Vec<i32> = Vec::new(); //数値を保存しておくベクタを用意

    loop {
        println!("\n\n入力された整数の一覧は以下の通りです");
        println!("{:?}\n", numbers);
        println!("行いたい操作の番号を選んでください");
        println!("0：整数を入力する\n1：平均値を表示\n2：中央値を表示\n3：最頻値を表示\n4：リセット\n5：終了");

        //ユーザーに操作を選んでもらう
        let mut operation_num = String::new();
        io::stdin()
            .read_line(&mut operation_num)
            .expect("入力に失敗しました");

        let operation_num: u8 = match operation_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("無効な値です。再度選択してください。");
                continue;
            }
        };

        //選んでもらった数字によって必要な関数を動かす
        match operation_num {
            0 => input_number(&mut numbers),
            1 => print_average(&numbers),
            2 => print_median(&numbers),
            3 => print_mode(&numbers),
            4 => numbers = Vec::new(),
            5 => break,
            _ => println!("無効な値です。再度選択してください。"),
        }
    }
}

fn input_number(numbers: &mut Vec<i32>) { //ベクタの可変参照を受け取る
    loop {
        println!("整数を入力してください");
        println!("qで入力を終了します");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力に失敗しました");

        let input = input.trim(); //入力値の改行を取り除く
        if &input[..] != "q" { //$str型で比較
            // 入力された整数値(String)をi32に型変換
            let input: i32 = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}は整数ではありません。再度入力してください。", input);
                    continue;
                }
            };
            numbers.push(input);
        } else {
            //qが入力された場合にループから抜け出す
            break;
        };
    }
}

fn print_average(numbers: &Vec<i32>) {
    let mut sum = 0;
    let len = numbers.len(); 
    for num in numbers {
        sum += num;
    }
    println!("平均値は{}です。", sum as f64 / len as f64);
}

fn print_median(numbers: &Vec<i32>) {
    let mut sort_numbers = numbers.clone(); //ソート用にベクタのクローンを作成
    sort_numbers.sort();
    let len = sort_numbers.len();
    let target = len / 2;
    let median: f64 = match len % 2 {
        0 => (&sort_numbers[target] + &sort_numbers[target - 1]) as f64 / 2.0,
        1 => (*&sort_numbers[target]) as f64,
        _ => {
            println!("エラー。もう一度やり直してください。");
            0 as f64
        }
    };
    println!("中央値は{}です。", median);
}

fn print_mode(numbers: &Vec<i32>) {
    let mut count_num_map = HashMap::new();
    let mut mode = Vec::new();

    //{整数値: 登場回数}という形式のハッシュマップを作成
    for num in numbers {
        let count = count_num_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut compare_num = 0;
    for (num, count) in &count_num_map {
        if count > &compare_num { //登場回数を比較。登場回数が単独トップであれば、ベクタをリセットして新たな整数値を格納。
            mode = Vec::new();
            mode.push(num);
            compare_num = *count;
        } else if count == &compare_num { //登場回数が同じであれば、ベクタに整数値を追加。
            mode.push(num);
        }
    }

    mode.sort();
    println!("最頻値は{:?}です", mode);
}
