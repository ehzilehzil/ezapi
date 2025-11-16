
fn redistribute_clusters(mut items: Vec<Item>, max_size: usize) -> (Vec<Item>, String) {
    use std::collections::HashMap;

    // 그룹별로 아이템 모으기
    let mut groups: HashMap<usize, Vec<Item>> = HashMap::new();
    for item in items {
        groups.entry(item.g).or_default().push(item);
    }

    // 1. 초과 그룹 확인
    if groups.values().all(|v| v.len() <= max_size) {
        return (groups.into_values().flatten().collect(), "✅ 재배치 불필요".to_string());
    }

    // 중심점 계산 함수
    fn centroid(items: &Vec<Item>) -> (f64, f64) {
        let (sum_lng, sum_lat): (f64, f64) = items.iter().map(|i| (i.lng, i.lat)).fold((0.0, 0.0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        let n = items.len() as f64;
        (sum_lng / n, sum_lat / n)
    }

    // 중심점 맵
    let mut centroids: HashMap<usize, (f64, f64)> = HashMap::new();
    for (g, v) in &groups {
        centroids.insert(*g, centroid(v));
    }

    let mut moved = false;

    // 2. 초과 그룹 처리
    for (g, v) in groups.clone() {
        if v.len() > max_size {
            let center = centroids[&g];
            // 먼 아이템부터 정렬
            let mut sorted = v.clone();
            sorted.sort_by(|a, b| {
                let da = ((a.lng - center.0).powi(2) + (a.lat - center.1).powi(2)).sqrt();
                let db = ((b.lng - center.0).powi(2) + (b.lat - center.1).powi(2)).sqrt();
                db.partial_cmp(&da).unwrap() // 먼 것부터
            });

            for item in sorted.into_iter().skip(max_size) {
                // 가장 가까운 중심점 찾기
                let mut candidates: Vec<(usize, f64)> = centroids.iter()
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
    (groups.into_values().flatten().collect(), status.to_string())
}
