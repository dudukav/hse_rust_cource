#![forbid(unsafe_code)]

#[derive(Debug, Eq, PartialEq)]
pub struct Statistics {
    pub average: i32,
    pub median: i32,
    pub min: i32,
    pub max: i32,
}

// Тут просто заменяем на ссылку на срез, так как нам не не нужен сам 
// объект, мы его только читаем. 
// было: fn calculate_average(data: Vec<i32>)
fn calculate_average(data: &[i32]) -> i32 {
    let mut sum = 0;
    let mut cnt = 0;
    for x in data {
        sum += x;
        cnt += 1;
    }
    return sum / cnt;
}

// Тут мы проводим манипуляции с самим объектом, поэтому нам нужна изменяемая ссылка на
// него (владеть им нам не надо).
fn calculate_median(data: &mut Vec<i32>) -> i32 {
    data.sort();
    return data[data.len() / 2];
}

// Здесь аналогично первому примеру, только так как у нас происходят сравнения с итератором, 
// на него тоже нужно наложить ссылку, иначе типы будут не совпдать.
// было: fn calclulate_minmax(data: Vec[i32])
fn calculate_minmax(data: &[i32]) -> (i32, i32) {
    let mut min = data[0];
    let mut max = data[0];
    for &x in data {
        if min > x {
            min = x;
        }
        if max < x {
            max = x;
        }
    }
    return (min, max);
}

// Здесь тоже ссылка на срез, так как 2/3 функцию принимают именно ссылку на срез,
// а 3 просто превращаем в вектор и берем на него изменяемую ссылку.
pub fn calculate_statistics(data: &[i32]) -> Statistics {
    let average = calculate_average(data);
    let median = calculate_median(&mut data.to_owned());
    let (min, max) = calculate_minmax(data);

    return Statistics {
        average,
        median,
        min,
        max,
    };
}