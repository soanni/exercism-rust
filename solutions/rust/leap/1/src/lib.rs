pub fn is_leap_year(year: u64) -> bool {
    //let mut leap = false;
    //if year % 4 == 0 {
    //    leap = true;
    //    if year % 100 == 0 {
    //        if year % 400 == 0 {
    //            leap = true;
    //        } else {
    //            leap = false;
    //        }
    //    }
    //}

    let leap = match year % 4 {
        0 if year % 100 == 0 && year % 400 == 0 => true,
        0 if year % 100 != 0 => true,
        _ => false,
    };

    leap
}
