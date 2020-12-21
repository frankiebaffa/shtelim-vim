use regex::Regex;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::fs::write;
use std::io::{BufReader, BufRead};

fn chk_d(num_str: &str) -> String {
    let mut ret_str = String::new();
    if num_str.len() == 1 {
        ret_str.push_str("0");
    }
    ret_str.push_str(num_str);
    return ret_str;
}

fn main() {
    if args().len() > 2 {
        println!("This program only takes one arg: file name");
        exit(0);
    }

    let mut file_name = String::new();
    let mut arg_i = 0;
    for arg in args() {
        if arg_i == 1 {
            file_name = arg.to_string();
            break;
        }
        arg_i = arg_i + 1;
    }

    if file_name.is_empty() {
        println!("Arg: file name, cannot be empty");
        exit(0);
    }

    let file_pat =
        r"^[a-zA-Z\-]+__[a-zA-Z\-]+_[0-9]{1,2}_[0-9]{1,2}_[0-9]{4}__[0-9]{1,2}_[0-9]{1,2}_[0-9]{1,2}__(AM|PM).txt";
    let file_reg = Regex::new(file_pat).unwrap();
    if !file_reg.is_match(file_name.as_str()) {
        println!(
            "File name {} may be in the wrong format, consider renaming",
            file_name);
        exit(0);
    }

    // first__last_M_d_yyyy__h_mm_ss__AA.txt
    let file_name_cl = file_name.clone();
    let mut file_name_str = file_name_cl.as_str();
    let first_pat = r"^[a-zA-Z\-]+";
    let first_reg = Regex::new(first_pat).unwrap();
    let first = first_reg.find(file_name_str).unwrap().as_str();

    let first_rem = first_reg.replace(file_name_str, "").to_string();
    file_name_str = first_rem.as_str();
    // __last_M_d_yyyy__h_mm_ss__AA.txt
    let first_und_rem: String = file_name_str.chars().skip(2).take(file_name_str.len() - 2).collect();
    file_name_str = first_und_rem.as_str();

    // last_M_d_yyyy__h_mm_ss__AA.txt
    let last = first_reg.find(file_name_str).unwrap().as_str();

    let last_rem = first_reg.replace(file_name_str, "").to_string();
    file_name_str = last_rem.as_str();
    let last_und_rem: String = file_name_str.chars().skip(1).take(file_name_str.len() - 1).collect();
    file_name_str = last_und_rem.as_str();

    // M_d_yyyy__h_mm_ss__AA.txt
    let num_pat = r"^[0-9]+";
    let num_reg = Regex::new(num_pat).unwrap();

    let month = num_reg.find(file_name_str).unwrap().as_str();

    let month_rem = num_reg.replace(file_name_str, "").to_string();
    file_name_str = month_rem.as_str();
    let month_und_rem: String = file_name_str.chars().skip(1).take(file_name_str.len() - 1).collect();
    file_name_str = month_und_rem.as_str();

    // d_yyyy__h_mm_ss__AA.txt
    let day = num_reg.find(file_name_str).unwrap().as_str();

    let day_rem = num_reg.replace(file_name_str, "").to_string();
    file_name_str = day_rem.as_str();
    let day_und_rem: String = file_name_str.chars().skip(1).take(file_name_str.len() - 1).collect();
    file_name_str = day_und_rem.as_str();

    // yyyy__h_mm_ss__AA.txt
    let year = num_reg.find(file_name_str).unwrap().as_str();

    let year_rem = num_reg.replace(file_name_str, "").to_string();
    file_name_str = year_rem.as_str();
    let year_und_rem: String = file_name_str.chars().skip(2).take(file_name_str.len() - 2).collect();
    file_name_str = year_und_rem.as_str();

    // h_mm_ss__AA.txt
    let mut hour = num_reg.find(file_name_str).unwrap().as_str();

    let hour_rem = num_reg.replace(file_name_str, "").to_string();
    file_name_str = hour_rem.as_str();
    let hour_und_rem: String = file_name_str.chars().skip(1).take(file_name_str.len() - 1).collect();
    file_name_str = hour_und_rem.as_str();

    // mm_ss__AA.txt
    let minute = num_reg.find(file_name_str).unwrap().as_str();

    let minute_rem = num_reg.replace(file_name_str, "").to_string();
    file_name_str = minute_rem.as_str();
    let minute_und_rem: String = file_name_str.chars().skip(1).take(file_name_str.len() - 1).collect();
    file_name_str = minute_und_rem.as_str();

    // ss__AA.txt
    let second = num_reg.find(file_name_str).unwrap().as_str();

    let second_rem = num_reg.replace(file_name_str, "").to_string();
    file_name_str = second_rem.as_str();
    let second_und_rem: String = file_name_str.chars().skip(2).take(file_name_str.len() - 2).collect();
    file_name_str = second_und_rem.as_str();

    // AA.txt
    let a_p_pat = r"AM|PM";
    let a_p_reg = Regex::new(a_p_pat).unwrap();
    let a_p = a_p_reg.find(file_name_str).unwrap().as_str();

    let hour_string;
    if a_p.eq("PM") {
        let mut hour_i = hour.parse::<i16>().unwrap();
        hour_i = hour_i + 12;
        if hour_i == 24 {
            hour = "00";
        } else {
            hour_string = hour_i.to_string();
            hour = hour_string.as_str();
        }
    }

    // yyyyMMdd.HHmmss.last.first.shtelim
    let new_file_name =
        format!(
            "{}{}{}.{}{}{}.{}.{}.shtelim",
            chk_d(year), chk_d(month), chk_d(day),
            chk_d(hour), chk_d(minute), chk_d(second),
            last,
            first);

    let reader = BufReader::new(File::open(file_name).unwrap());
    let mut re_form = String::new();
    for line_res in reader.lines() {
        let line_string = line_res.unwrap();
        let line= line_string.as_str();

        let msg_head_pat =
            r"^[a-zA-Z\s]+\([0-9]{2}/[0-9]{2}/[0-9]{4}\s[0-9]{1,2}:[0-9]{2}:[0-9]{2}\s(PM|AM)\):\s";
        let msg_head_reg = Regex::new(msg_head_pat).unwrap();

        let msg_stat_pat = r"^\S[\s\S]+\.$";
        let msg_stat_reg = Regex::new(msg_stat_pat).unwrap();

        let is_head = msg_head_reg.is_match(line);
        let is_stat = msg_stat_reg.is_match(line);
        if is_head {
            let msg_head = msg_head_reg.find(line).unwrap();
            let msg_line = msg_head_reg.replace(line, "").to_string();
            re_form.push_str(msg_head.as_str().trim());
            re_form.push('\n');

            let mut i = 1;
            let mut j = 1;
            let mut line_new = String::new();
            for ch in msg_line.chars() {
                if i == 1 {
                    line_new.push('\t');
                }
                if i >= 70 && (ch.eq(&' ') || ch.eq(&'-')) {
                    line_new.push_str("\n");
                    re_form.push_str(line_new.as_str());
                    line_new = String::new();
                    i = 0;
                } else if j == msg_line.chars().count() {
                    line_new.push(ch);
                    line_new.push_str("\n");
                    re_form.push_str(line_new.as_str());
                    line_new = String::new();
                    i = 0;
                } else {
                    line_new.push(ch);
                }

                i = i + 1;
                j = j + 1;
            }
        } else if is_stat {
            let msg_stat = msg_stat_reg.find(line).unwrap();
            re_form.push_str(msg_stat.as_str());
            re_form.push('\n');
        }
    }

    write(new_file_name, re_form.as_str()).unwrap();
}
