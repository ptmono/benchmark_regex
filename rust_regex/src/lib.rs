#![feature(core_intrinsics)]
#[macro_use]
extern crate cpython;
extern crate timeit;
extern crate regex;


use cpython::{Python, PyResult, PyList, PyUnicode};
use regex::Regex;

use cpython::PythonObject;



fn _print_type_of<T>(_: &T) {
    println!("{}", {std::intrinsics::type_name::<T>()});
}

fn _typeid<T: std::any::Any>(_: &T) {
    println!("{:?}", std::any::TypeId::of::<T>());
}


fn findall(_py: Python, regex: &str, text: &str) -> PyResult<PyList> {
    let re = Regex::new(regex).unwrap();
    let result = PyList::new(_py, &[]);

    for cap in re.captures_iter(text) {
        let dd = PyUnicode::new(_py, &cap[0]).into_object();
        result.append(_py, dd);
    }
    Ok(result)
}


fn regex_findall(regex: &str, text: &str) -> String {
    let mut result = String::new();
    let re = Regex::new(regex).unwrap();

    for cap in re.captures_iter(text) {
        result.push_str("\n");
        result.push_str(&cap[0]);
    }
    return result
}


fn findall2(_py: Python, regex: &str, text: &str, count: i64) -> PyResult<PyList> {
    let result = PyList::new(_py, &[]);

    for x in 0..count {
        regex_findall(regex, text);
    }
    Ok(result)
}


py_module_initializer!(librustlibs, initlibrustlibs, PyInit_rustlibs, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    r#try!(m.add(py, "findall_with_rust_regex", py_fn!(py, findall(regex: &str, text: &str))));
    r#try!(m.add(py, "findall_loop_on_rust", py_fn!(py, findall2(regex: &str, text: &str, count: i64))));
    Ok(())
});


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_findall() {
        let regex = r"(\d{4})-(\d{2})-(\d{2})";
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";

        let dd = regex_findall(regex, text);
        println!("{}", dd);
    }

    #[test]
    fn test_regex_findall_timeit() {
        let regex = r"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)";
        let text = "(서울=뉴스1) 이동희 기자 = 서울 집값이 39주 만에 하락했다. 정부 규제에 신종 코로나바이러스 감염증(코로나19) 여파로 매수 심리가 더욱 위축하면서 지난 2주간 상승세를 멈췄던 아파트값이 하락 전환했다.

한국감정원이 2일 발표한 '2020년 3월 5주(30일 기준) 전국 주간 아파트 가격동향'에 따르면 서울 아파트값 변동률은 -0.02%를 기록했다.

서울 집값이 하락세를 기록한 것은 지난해 7월 1주 이후 39주 만에 처음이다.

감정원 관계자는 \"코로나19 사태로 촉발된 대내외 경제 불확실성과 자금출처 증빙 강화, 보유세 부담 증가 등으로 매수심리가 크게 위축했다\"라면서 \"강남권에 이어 강북 대표 지역도 하락하며 서울 전체가 하락 전환했다\"고 설명했다.

강남3구(강남·서초·송파구)는 하락폭이 더 확대했고, 강동구(-0.01%)도 중대형 단지 위주로 매물이 증가하며 하락 전환했다. 마포(-0.02%)·용산(-0.01%)·성동구(-0.01%) 등 강북 대표 지역도 하락세로 나타났다. 그동안 상대적으로 상승률이 높았던 노원(0.04%)·도봉(0.05%)·강북구(0.05%)는 상승세가 둔화했다.

서울을 제외한 수도권 역시 상승폭이 축소했다. 인천은 0.42%에서 0.34%로, 경기는 0.28%에서 0.19%로 상승세가 둔화한 것으로 나타났다. 다만 경기에서 안산 단원구(0.61%), 군포(0.55%), 시흥(0.53%), 구리(0.53%) 등은 비교적 높은 상승률을 기록했다.

지방은 대전(0.2%), 울산(0.02%) 등은 상승폭이 축소했고, 부산(-0.02%), 대구(-0.04%) 등은 하락세를 이어갔다. 세종은 1주 전보다 0.03%포인트(p) 낮은 0.24%를 기록했다.

서울 전셋값은 강동구 등 일부 지역을 제외하곤 전반적으로 상승폭이 축소해 0.03%를 기록했다. 강동구는 신규 입주단지의 매물이 소진하면서 0.04% 올랐다. 강남구(0.05%) 등 강남3구는 정비사업 이주수요로 상승세를 유지했다. 강북은 직주근접 등 수요가 꾸준한 마포구(0.06%)와 성동구(0.06%)를 중심으로 상승했다. 3045가구 규모의 신규 입주단지 물량이 쏟아진 양천구(-0.02%)는 4주째 하락했다. goojoa@news1.kr

인천과 경기는 각각 0.19%, 0.04%를 기록했다. 경기에서 과천시는 청약요건 강화와 공급물량 영향으로 0.88% 하락했다. 지방은 대전(0.05%), 울산(0.05%) 등은 오르고, 부산(-0.01%), 광주(-0.01%) 등은 내렸다. 대구는 보합을 기록했다.

yagoojoa@news1.kr";

        timeit!({
            for i in 0..20000 {
                regex_findall(regex, text);
            }
        });
    }
}
