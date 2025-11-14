// rustup target add wasm32-unknown-unknown
// cargo build --package mtsp --target wasm32-unknown-unknown --release
// 결과물이 ./target/wasm32-unknown-unknown/release/[이름].wasm
// 이를 ./lib로 복사
// Copilot 도움을 받아 코드 작성

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Item {
    name: String,
    addr: String,
    lng: f64,
    lat: f64,
    g: usize,
}

#[derive(Debug, Clone)]
struct LngLat(f64, f64);

fn euclidean_distance(a: &LngLat, b: &LngLat) -> f64 {
    return ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
}

fn kmeans(mut items: Vec<Item>, mut k: usize, max_iters: usize) -> Vec<Item> {
    //-> k < 2 이면, 그룹핑 필요 없다고 판단 그냥 리턴
    if k < 2 {
        return items;
    }

    //-> centeroids 선택, items 를 lng -> lat 순으로 정렬하여, 균등 간격으로 k 개 설정
    k = if items.len() < k { items.len() } else { k };
    items.sort_by(|a, b| {
        return a.lng.partial_cmp(&b.lng).unwrap()
            .then_with(|| a.lat.partial_cmp(&b.lat).unwrap());
    });

    let step = items.len() / k;
    let mut centeroids = (0..k)
        .map(|i| LngLat(items[i * step].lng, items[i * step].lat))
        .collect::<Vec<LngLat>>();

    //-> 반복 구문으로 centeroids 변경해가며 그룹핑
    for _ in 0..max_iters {
        //--> 가까운 centeroids 인덱스로 그룹지정
        for item in items.iter_mut() {
            let mut min_dist = f64::MAX;
            let mut group = 0;

            for (i, c) in centeroids.iter().enumerate() {
                let dist = euclidean_distance(&LngLat(item.lng, item.lat), c);
                if dist < min_dist {
                    min_dist = dist;
                    group = i;
                }
            }
            item.g = group;
        }

        //--> 새로운 centeroids 계산
        let mut lngs = vec![0.0; k];
        let mut lats = vec![0.0; k];
        let mut counts = vec![0; k];

        for item in items.iter() {
            lngs[item.g] += item.lng;
            lats[item.g] += item.lat;
            counts[item.g] += 1;
        }

        let mut new_centeroids = vec![LngLat(0.0, 0.0); k];
        for i in 0..k {
            if counts[i] > 0 {
                new_centeroids[i] = LngLat(lngs[i] / (counts[i] as f64), lats[i] / (counts[i] as f64));
            } else {
                new_centeroids[i] = centeroids[i].clone();
            }
        }

        //--> 변화가 "거의" 없을 경우 break
        if centeroids.iter().zip(new_centeroids.iter())
            .all(|(a, b)| (a.0 - b.0).abs() < 1e-4 && (a.1 - b.1).abs() < 1e-4)
            {
                break;
            }

        //--> 새로운 centeroids 로 갱신 후 다시 반복
        centeroids = new_centeroids;
    }

    return items;
}

fn tsp_nearest_neighbor(mut g_items: Vec<Item>) -> Vec<Item> {
    if g_items.is_empty() {
        return g_items;
    }

    let mut visited = vec![false; g_items.len()];
    let mut route = Vec::with_capacity(g_items.len());

    let mut current = 0;
    visited[current] = true;
    route.push(g_items[current].clone());

    for _ in 1..g_items.len() {
        let mut next = None;
        let mut min_dist = f64::MAX;

        for (i, p) in g_items.iter().enumerate() {
            if !visited[i] {
                let dist = ((g_items[current].lng - p.lng).powi(2)
                    + (g_items[current].lat - p.lat).powi(2))
                .sqrt();
                if dist < min_dist {
                    min_dist = dist;
                    next = Some(i);
                }
            }
        }

        if let Some(n) = next {
            visited[n] = true;
            route.push(g_items[n].clone());
            current = n;
        }
    }

    return route;
}

fn tsp_by_groups(items: Vec<Item>) -> Vec<Item> {
    use std::collections::HashMap;

    //-> 그룹별 분리
    let mut groups = HashMap::<usize, Vec<Item>>::new();
    for item in items {
        groups.entry(item.g).or_default().push(item);
    }

    //-> 그룹별로 tsp 실행하고 그 결과를 합쳐서 리턴
    let mut result = Vec::<Item>::new();
    let mut keys = groups.keys().cloned().collect::<Vec<_>>();
    keys.sort();

    for key in keys {
        let g_items = groups.remove(&key).unwrap();
        let ordered_g_items = tsp_nearest_neighbor(g_items);
        result.extend(ordered_g_items);
    }

    return result;
}


#[unsafe(no_mangle)]
pub extern "C" fn get_mtsp(items: &str, k: usize) -> String {
    //-> items 문자열을 Vec<Item> 으로 변환하여 kmeans, tsp 솔빙 실행
    let parsed_items = serde_json::from_str::<Vec<Item>>(items).unwrap();
    let kmeans_result = kmeans(parsed_items, k, 100);
    let result = tsp_by_groups(kmeans_result);
    return serde_json::to_string(&result).unwrap();
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let items = vec![
            Item { name: "A".into(), addr: "Seoul".into(),    lng: 126.9780, lat: 37.5665, g: 0 },
            Item { name: "B".into(), addr: "Incheon".into(),  lng: 126.7052, lat: 37.4563, g: 0 },
            Item { name: "C".into(), addr: "Suwon".into(),    lng: 127.0286, lat: 37.2636, g: 0 },
            Item { name: "D".into(), addr: "Busan".into(),    lng: 129.0756, lat: 35.1796, g: 0 },
            Item { name: "E".into(), addr: "Daegu".into(),    lng: 128.6014, lat: 35.8714, g: 0 },
            Item { name: "F".into(), addr: "Daejeon".into(),  lng: 127.3845, lat: 36.3504, g: 0 },
            Item { name: "G".into(), addr: "Gwangju".into(),  lng: 126.8530, lat: 35.1595, g: 0 },
            Item { name: "H".into(), addr: "Ulsan".into(),    lng: 129.3114, lat: 35.5384, g: 0 },
            Item { name: "I".into(), addr: "Jeonju".into(),   lng: 127.1500, lat: 35.8242, g: 0 },
            Item { name: "J".into(), addr: "Gangneung".into(),lng: 128.8960, lat: 37.7519, g: 0 },
        ];


        let r = kmeans(items, 2, 100);
        let r = tsp_by_groups(r);

        dbg!(r);
    }
}