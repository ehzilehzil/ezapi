// Copilot 도움을 받아 코드 작성

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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

fn redistribute_clusters(mut items: Vec<Item>, max_size: usize) -> (Vec<Item>, String) {
    use std::collections::HashMap;

    //-> max_size 초과 그룹 여부 확인
    let mut groups = HashMap::<usize, usize>::new();
    for item in items.iter() {
        *groups.entry(item.g).or_insert(0) += 1;
    }
    if groups.values().all(|x| x <= &max_size) {
        return (items, "OK".to_owned());
    }

    //-> 그룹별 모음
    let mut groups = HashMap::<usize, Vec<Item>>::new();
    for item in items {
        groups.entry(item.g).or_default().push(item);
    }

    //-> 중심점 계산 함수
    fn centeroid(items: &Vec<Item>) -> (f64, f64) {
        let (sum_lng, sum_lat) = items.iter().map(|x| (x.lng, x.lat)).fold((0.0, 0.0), |a, x| (a.0 + x.0, a.1 + x.1));
        let n = items.len() as f64;
        return (sum_lng / n, sum_lat / n);
    }

    //-> 중심점 맵
    let mut centeroids = HashMap::<usize, (f64, f64)>::new();
    for (g, items) in &groups {
        centeroids.insert(*g, centeroid(items));
    }


    let mut moved = false;
    //-> 초과 그룹 처리
    for (g, v) in groups.clone() {
        if v.len() > max_size {
            let center = centeroids[&g];
            // 먼 아이템부터 정렬
            let mut sorted = v.clone();
            sorted.sort_by(|a, b| {
                let da = ((a.lng - center.0).powi(2) + (a.lat - center.1).powi(2)).sqrt();
                let db = ((b.lng - center.0).powi(2) + (b.lat - center.1).powi(2)).sqrt();
                db.partial_cmp(&da).unwrap() // 먼 것부터
            });

            for item in sorted.into_iter().skip(max_size) {
                // 가장 가까운 중심점 찾기
                let mut candidates: Vec<(usize, f64)> = centeroids.iter()
                    .filter(|(cg, _)| **cg != g)
                    .map(|(cg, c)| (*cg, ((item.lng - c.0).powi(2) + (item.lat - c.1).powi(2)).sqrt()))
                    .collect();
                candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                let mut moved_item = false;
                for (target_g, _) in candidates {
                    if groups[&target_g].len() < max_size {
                        let mut new_item = item.clone();
                        new_item.g = target_g;
                        groups.get_mut(&target_g).unwrap().push(new_item);
                        moved_item = true;
                        moved = true;
                        break;
                    }
                }

                if !moved_item {
                    return (groups.into_values().flatten().collect(), "⚠️ 모든 그룹이 꽉 차서 일부 초과 유지".to_string());
                }
            }
        }
    }

    let status = if moved { "✅ 재배치 완료" } else { "⚠️ 재배치 불가능" };
    return (groups.into_values().flatten().collect(), status.to_string());
}


// #[unsafe(no_mangle)]
// pub extern "C" fn get_mtsp(items: &str, k: usize) -> String {
#[wasm_bindgen]
pub fn get_mtsp(items: &str, k: usize) -> String {
    //-> items 문자열을 Vec<Item> 으로 변환하여 kmeans, tsp 솔빙 실행
    let parsed_items = serde_json::from_str::<Vec<Item>>(items).unwrap();
    let kmeans_result = kmeans(parsed_items, k, 100);
    let result = tsp_by_groups(kmeans_result);
    let result = redistribute_clusters(result, 30).0;
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