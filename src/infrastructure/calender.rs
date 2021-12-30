extern crate koyomi;
use koyomi::{Calendar, Date};


#[derive(Debug,PartialEq)]
enum DayOfWeek {
    SUNDAY,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,    
    NONEDAY,
}

#[derive(Debug,PartialEq)]
enum MONTH {
	JANUARY,
	FEBRUARY,
	MARCH,
	APRIL,
	MAY,
	JUNE,
	JULY,
	AUGUST,
	SEPTEMBER,
	OCTOBER,
	NOVEMBER,
	DECEMBER,
}

const DAYS_IN_MONTH: i32 = 31;
const DAY_LEN: i32 = 3;
const WEEK_LEN: i32 = 3;

const days_in_month: [[i32; 13]; 2] = [
    [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
];

struct Year {

}

#[derive(Debug,PartialEq)]
struct Month {
    number_of_days: i32,
    initial_day: DayOfWeek,
    // 何月か？
    // 年と月から日数が計算できる
}

#[derive(Debug,PartialEq)]
struct Day {
    day_in_month: i32,
    day_of_week: DayOfWeek,
}

fn calc_day_of_week(ymd: [i32;3]) -> DayOfWeek {
    DayOfWeek::MONDAY
}

fn calc_month(ym: [i32;2]) -> Month {
    let month = Month {number_of_days: 31, initial_day: DayOfWeek::MONDAY};
    month
}

fn calc_days_in_month(ym: [i32;2]) -> Vec<Day> {
    let day = Day { day_in_month: 1, day_of_week: DayOfWeek::MONDAY };
    Vec::from([day])
}

fn monthly() {
    // 数カ月分のカレンダーを表示する
    // 縦表示、横表示 : vertical
    // 表示する月数 : number_of_month = 3 default
    let vertical = false;
    if vertical == false {
        cal_output_header();
        cal_output_month();
    } else {
        cal_output_vertical_header();
        cal_output_vertical_month();        
    }

}

fn cal_output_header() {

}

fn cal_output_month() {

}

fn cal_output_vertical_header() {

}

fn cal_output_vertical_month() {

}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::calender::koyomi::calcDayOfWeek;

    #[test]
    // 2021/12/28 is Tuesday
    fn test_calc_day_of_week() {
        assert_eq!(calc_day_of_week([2021,12,28]), DayOfWeek::THURSDAY);
    }

    // test days in 2021/12
    fn test_calc_month() {
        assert_eq!(calc_month([2021,12]), Month {number_of_days: 31, initial_day: DayOfWeek::WEDNESDAY});
    }

    fn test_calc_days_in_month() {
        let aday = Day {day_in_month: 1, day_of_week: DayOfWeek::MONDAY};
        let days =  Vec::from([aday]);
        assert_eq!(calc_days_in_month([2021,12]), days);
    }

    // test pointer to the next month    
    fn test_calc_previous_month() {

    }

    fn test_calc_next_month() {

    }
}

