//extern crate koyomi;
use chrono::prelude::{DateTime, Local, Datelike, Weekday};
use std::{fmt, io::LineWriter};
//use std::borrow::BorrowMut;
//use std::cell::RefCell;
//use std::rc::Rc;
use crate::infrastructure::doubly_linked_list::{LinkedList};
use num::traits::FromPrimitive;

#[derive(Debug,PartialEq)]
pub enum MONTH {
	Jan,
	Feb,
	Mar,
	Apr,
	May,
	Jun,
	Jul,
	Aug,
	Sep,
	Oct,
	Nov,
	Dec,
}

impl num::traits::FromPrimitive for MONTH {
    #[inline]
    fn from_i32(n: i32) -> Option<MONTH> {
        match n {
            1 => Some(MONTH::Jan),
            2 => Some(MONTH::Feb),
            3 => Some(MONTH::Mar),
            4 => Some(MONTH::Apr),
            5 => Some(MONTH::May),
            6 => Some(MONTH::Jun),
            7 => Some(MONTH::Jul),
            8 => Some(MONTH::Aug),
            9 => Some(MONTH::Sep),
            10 => Some(MONTH::Oct),
            11 => Some(MONTH::Nov),
            12 => Some(MONTH::Dec),
            _ => None,
        }
    }
    fn from_i64(n: i64) -> Option<MONTH> {
        match n {
            1 => Some(MONTH::Jan),
            _ => None,
        }
    }
    fn from_u64(n: u64) -> Option<MONTH> {
        match n {
            1 => Some(MONTH::Jan),
            _ => None,
        }
    }
}

impl fmt::Display for MONTH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MONTH::Jan => write!(f, "Jan"),
            MONTH::Feb => write!(f, "Feb"),
            MONTH::Mar => write!(f, "Mar"),
            MONTH::Apr => write!(f, "Apr"),
            MONTH::May => write!(f, "May"),
            MONTH::Jun => write!(f, "Jun"),
            MONTH::Jul => write!(f, "Jul"),
            MONTH::Aug => write!(f, "Aug"),
            MONTH::Sep => write!(f, "Sep"),
            MONTH::Oct => write!(f, "Oct"),
            MONTH::Nov => write!(f, "Nov"),
            MONTH::Dec => write!(f, "Dec"),
        }
    }
}

const DAYS_IN_MONTH: u32 = 31;
const DAY_LEN: u32 = 3;
const WEEK_LEN: u32 = 3;

const days_in_month: [[i32; 13]; 2] = [
    [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
];

// In Japan, the Gregorian calender was introduced January 1st in 1873 and it was Wednesday.
// Then I use the origin of Gregorian calender as 1873/1/1, Wednesday.

// mod operation which does not return zero or negative value
fn mod_positive(operand: i32, period: i32) -> i32 {    
    if period <= 0 {
        panic!("Invarid argument for mod_positive, period : {}", period);
    };
    
    let mut tmp = if operand < 0 {
        operand + 1
    } else {
        operand
    };
    tmp %= period;
    if tmp == 0 { 
        tmp = period 
    } else {
        while tmp <= 0 {
            tmp += period;
        }
    }       
    //println!("mod_positive: {}", tmp);
    tmp
}
// mod operation which does not return negative value
fn mod_non_negative(operand: i32, period: i32) -> i32 {
    if period <= 0 {
        panic!("Invarid argument for mod_positive, period : {}", period);
    };
    let mut tmp = operand % period;
    while tmp < 0 {
        tmp += period;
    }
    tmp
}

#[test]
fn test_mod_non_negative() {    
    assert_eq!(mod_non_negative(11,10), 1);
    assert_eq!(mod_non_negative(101,10), 1);
    assert_eq!(mod_non_negative(-5,10), 5);
}
#[test]
fn test_mod_positive() {    
    assert_eq!(mod_positive(11,12), 11);
    assert_eq!(mod_positive(-1,12), 12);
    assert_eq!(mod_positive(-2,12), 11);
    assert_eq!(mod_positive(-12,12), 1);
    assert_eq!(mod_positive(-24,12), 1);
}

fn is_leap_year(year: i32) -> bool {
    let bleap = if year % 4 != 0 {
        false
    } else {
        if year % 100 == 0 && year % 400 != 0 {
            false
        } else {
            true
        }            
    };
    bleap
}

fn wday(date: CalDate) -> Option<Weekday> {    
    let mut month = date.month;
    let mut year = date.year;
    if month == 1 || month == 2 {
        month += 12;
        year -= 1;
    }
    let year_mod_100: i32 = year % 100;
    let term2: i32 = 26*(month + 1)/10;
    let term4: i32 = year_mod_100/4;
    let term5: i32 = - 2 * (year/100 as i32);
    let tmp: i32 = (date.mday as i32) + term2 + year_mod_100 + term4 + term5 + (year/400 as i32);
    let iday: i32 = mod_non_negative(tmp - 2, 7);
    //println!("debug wday: year:{} month:{} year_mod_100:{} term2:{} term4:{} term5:{} tmp:{} iday:{}", date.year, date.month, year_mod_100, term2, term4, term5, tmp, iday);
    Weekday::from_i32(iday)
    // 0 --> Mon
    // 1 --> Tue
    // 2 --> Wed
}

struct CalDate {    
    year: i32,
    month: i32,
    mday: i32,
    wday: Weekday,
}

const origin_date: CalDate = CalDate {
    year: 1873,
    month: 1,
    mday: 1,
    wday: Weekday::Wed,
};

impl CalDate {

    pub fn new(year: i32, month: i32, mday: i32, wday: Weekday) -> CalDate {
        CalDate {
            year, month, mday, wday
        }
    }

    pub fn today() -> CalDate {
        let dt: DateTime<Local> = Local::now();
        println!("{}",dt);
        CalDate {
            year: dt.year(),
            month: dt.month() as i32,
            mday: dt.day() as i32,
            wday: dt.weekday(),
        }
    }
}

impl fmt::Display for CalDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data: {}-{}-{} {}", self.year, self.month, self.mday, self.wday)
    }
}

#[test]
// 2022/1/1 is Saturday
fn test_new_caldate() {
    let caldate = CalDate::new(2022,1,1, Weekday::Sat);
    assert_eq!(caldate.year, 2022);
    assert_eq!(caldate.month, 1);
    assert_eq!(caldate.mday, 1);
    assert_eq!(caldate.wday, Weekday::Sat);
}

// 1ヶ月の日付・曜日の情報をベクタで保持する
#[derive(Debug,PartialEq,Clone)]
struct Month {
    year: i32,
    month: i32,
    length_of_days: i32,
    initial_day_of_week: Weekday,
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Month: {}-{} len:{} init:{}", self.year, self.month, self.length_of_days, self.initial_day_of_week)
    }
}

impl Month {
    // month_shift = -1 : one month before
    // month_shift = 0 : this month    
    // month_shift = 1 : the next month
    pub fn new(today: &CalDate, month_shift: i32) -> Month {                        
        let month_shift_mod: i32 = month_shift % 12;
        let mut year_shift: i32 = month_shift / 12;
        if month_shift < 0 {
            year_shift -= 1;
            if month_shift_mod == 0 {
                year_shift += 1;
            }
        }
        
        let year = today.year + year_shift;
        let month = mod_positive(today.month + month_shift_mod, 12); 
        let length_of_days = if is_leap_year(year) {
            days_in_month[1][(month as usize)]
        } else {
            days_in_month[0][(month as usize)]
        };
        let first_day = CalDate::new(year,month,1,Weekday::Sun); // weekday will not be used in wday();      
        let initial_day_of_week = wday(first_day).unwrap();
        Month {
            year,
            month,
            length_of_days,
            initial_day_of_week,
        }
    }

    pub fn this_month() -> Month {
        let today = CalDate::today();
        Month::new(&today, 0)
    }

    pub fn calc_linked_list_of_month(today: &CalDate, number_of_next_month: i32, number_of_previous_month: i32) -> LinkedList<Month> {
        if number_of_next_month < 0 || number_of_previous_month < 0 {
            panic!("Invalid argument: number_of_next_month {} number_of_previous_month {}", number_of_next_month, number_of_previous_month);
        }
        //let today = CalDate::today();
        let mut list = LinkedList::new();
        list.append(Month::new(&today,0));
        (0..number_of_next_month).for_each(|n| list.append(Month::new(&today, n+1)));
        (0..number_of_previous_month).for_each(|n| list.prepend(Month::new(&today, -n-1)));
        list
    }    
}

fn cal_output_header(list_month: &LinkedList<Month>) {
    let mut iter = list_month.iter().peekable();
    
    for i in iter {
        print!("      {} {}               ", i.year, MONTH::from_i32(i.month).unwrap());
    }
    print!("\n");
    for i in (0..list_month.len()) {
        print!("Sun Mon Tue Wed Thu Fri Sat ");
        if i < list_month.len() {
            print!("  ");
        }        
    }
    print!("\n");
    
}

fn num_to_string_day(day: i32) -> String {
    if day == 0 {
        return "".to_string()        
    } else {
        return day.to_string()
    }
}

fn cal_output_month(list_month: &LinkedList<Month>) {    
    let mut iter = list_month.iter().peekable();    
    let len_month: usize = list_month.len() as usize;
    let width: usize = 7 * len_month;
    let height: usize = 6;
    let mut vec_days = Vec::with_capacity(width * height);
    vec_days.resize(width*height, 0_i32);

    // index(x,y) denotes following position in the array of day in the calender
    // +-----+-----+-----+ ... +-----------+
    // |(0,0)|(1,0)|(2,0)| ... |(width-1,0)|
    // |(0,1)|(1,1)|(2,1)| ... |(width-1,1)|
    //  ...
    
    // init_weekday : Sun --> offset = 0
    // init_weekday : Mon --> offset = 1
    // ...
    // init_weekday : Sat --> offset = 6
    let mut count_month: usize = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    for v in iter {
        let offset: usize = (v.initial_day_of_week.num_days_from_sunday() as usize);
        let len_days: usize =  v.length_of_days as usize;
        for day in (0..len_days) {
            x = (day + offset) % 7;
            y = (day + offset) / 7;
            //println!("{} {}",x,y);
            vec_days[x + count_month * 7 + width * y] = day as i32;
        }
        count_month += 1;
    }

    for y in (0..height) {
        for imonth in (0..len_month) {        
            for iweekday in (0..7) {                
                print!("{: >3} ", num_to_string_day(vec_days[iweekday + 7*imonth + width * y]));
            }
            if imonth < len_month {
                print!("  ");
            }            
        }
        if y < height {
            print!("\n")
        }        
    }
}

pub fn cli_calender() {
    monthly();
}

fn monthly() {
    // 数カ月分のカレンダーを表示する
    // 縦表示、横表示 : vertical
    // 表示する月数 : number_of_month = 3 default    
    let today = CalDate::today();
    let list_month = Month::calc_linked_list_of_month(&today, 1, 1);
    let vertical = false;
    if vertical == false { // horizontal        
        let mut iter = list_month.iter().peekable();
        cal_output_header(&list_month);
        cal_output_month(&list_month);
        

    } else {
        cal_output_vertical_header();
        cal_output_vertical_month();        
    }

}


fn cal_output_vertical_header() {

}

fn cal_output_vertical_month() {

}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]    
    fn test_new_month() {
        // 2022/1/1 is Saturday
        let mut caldate = CalDate::new(2022, 1, 1, Weekday::Sat);
        let mut month = Month::new(&caldate, 0);
        let mut month_golden = Month {
            year: 2022,
            month: 1,
            length_of_days: 31,
            initial_day_of_week: Weekday::Sat,
        };        
    
        // 2021/12/1 is Wednesday
        let mut caldate = CalDate::new(2021, 12, 31, Weekday::Fri);
        let mut month = Month::new(&caldate, 0);
        month_golden = Month {
            year: 2021,
            month: 12,
            length_of_days: 31,
            initial_day_of_week: Weekday::Wed,
        };              
        //println!("{}", month);
        //println!("{}", month_golden);
        assert_eq!(month, month_golden);    
    }

    #[test]    
    fn test_new_previous_month() {
        // 2021/12/1 is Wednesday
        let mut caldate = CalDate::new(2022, 1, 1, Weekday::Sat);
        let mut month = Month::new(&caldate, -1);
        let mut month_golden = Month {
            year: 2021,
            month: 12,
            length_of_days: 31,
            initial_day_of_week: Weekday::Wed,
        };
        assert_eq!(month, month_golden);   
        
        // 2021/1/1 is Friday
        let mut month = Month::new(&caldate, -12);
        let mut month_golden = Month {
            year: 2021,
            month: 1,
            length_of_days: 31,
            initial_day_of_week: Weekday::Fri,
        };
        assert_eq!(month, month_golden);   
        
        // 2020/12/1 is Tuesday
        let mut month = Month::new(&caldate, -13);
        let mut month_golden = Month {
            year: 2020,
            month: 12,
            length_of_days: 31,
            initial_day_of_week: Weekday::Tue,
        };
        assert_eq!(month, month_golden);           
    }

    #[test]    
    // test pointer to the next month
    fn test_calc_next_month() {
        // 2022/2/1 is Tuesday
        let mut caldate = CalDate::new(2022, 1, 1, Weekday::Sat);
        let mut month = Month::new(&caldate, 1);
        let mut month_golden = Month {
            year: 2022,
            month: 2,
            length_of_days: 28,
            initial_day_of_week: Weekday::Tue,
        };
        assert_eq!(month, month_golden);   
        
        // 2022/12/1 is Thursday
        let mut month = Month::new(&caldate, 11);
        let mut month_golden = Month {
            year: 2022,
            month: 12,
            length_of_days: 31,
            initial_day_of_week: Weekday::Thu,
        };
        assert_eq!(month, month_golden);   
        
        // 2020/12/1 is Tuesday
        let mut month = Month::new(&caldate, 12);
        let mut month_golden = Month {
            year: 2023,
            month: 1,
            length_of_days: 31,
            initial_day_of_week: Weekday::Sun,
        };
        assert_eq!(month, month_golden);  
    }

    #[test]
    fn test_linked_list_of_month () {
        let caldate = CalDate::new(2022, 1, 1, Weekday::Sat);
        let list = Month::calc_linked_list_of_month(&caldate, 1, 1);
        let mut list_golden = LinkedList::new();
        list_golden.append(Month::new(&CalDate::new(2021,12,1,Weekday::Sun), 0));
        list_golden.append(Month::new(&CalDate::new(2022,1,1,Weekday::Sun), 0));
        list_golden.append(Month::new(&CalDate::new(2022,2,1,Weekday::Sun), 0));
        println!("test_linked_list_of_month:");
        println!("list:        {}", list);
        println!("list_golden: {}", list_golden);
        //println!("bool: {}",(list==list_golden));
        assert_eq!(list, list_golden);
    }

}

