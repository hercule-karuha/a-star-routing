use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response,
};
use serde::{de::value, Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Serialize, Deserialize)]
pub struct MapInfo {
    pub map: Vec<Vec<usize>>,
    pub start: Vec<usize>,
    pub end: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub route: Vec<Vec<usize>>,
}

pub async fn a_star_routing(
    Json(map_info): Json<Value>,
) -> Result<response::Json<Value>, (StatusCode, String)> {
    let map_info: MapInfo = serde_json::from_value(map_info.clone()).map_err(|err| {
        println!("{:?}", map_info.to_string());
        (StatusCode::BAD_REQUEST, err.to_string())
    })?;

    let route = Route {
        route: a_star_path_finding(
            &map_info.map,
            (map_info.start[0] as usize, map_info.start[1] as usize),
            (map_info.end[0] as usize, map_info.end[1] as usize),
        ),
    };

    let return_json = serde_json::to_value(route)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(return_json))
}

fn a_star_path_finding(
    map: &Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<usize>> {
    let mut open_set = VecDeque::<(usize, usize)>::new();
    open_set.push_back(start.clone());
    let mut colse_set = HashSet::<(usize, usize)>::new();
    let mut camefrom = HashMap::<(usize, usize), (usize, usize)>::new();

    let mut g_cost = vec![vec![0.0; map.first().unwrap().len()]; map.len()];
    let mut h_cost = vec![vec![0.0; map.first().unwrap().len()]; map.len()];
    let mut f_cost = vec![vec![0.0; map.first().unwrap().len()]; map.len()];

    let col = map.len();
    let row = map.first().unwrap().len();

    while !open_set.is_empty() {
        let current = open_set.pop_front().unwrap();

        if current == end {
            return find_path(&camefrom, end);
        }

        let diagonal_neighbors = get_diagonal_neighbors(col, row, current);
        let neighbors = get_neighbors(col, row, current);

        let _unused = colse_set.insert(current);
        for (x, y) in &diagonal_neighbors {
            if !open_set.contains(&(*x, *y)) && map[*x][*y] != 1 && !colse_set.contains(&(*x, *y)) {
                g_cost[*x][*y] = g_cost[current.0][current.1] + 1.4;
                h_cost[*x][*y] = manhattan_distance((*x, *y), end);
                f_cost[*x][*y] = g_cost[*x][*y] + h_cost[*x][*y];
                let _unused = camefrom.entry((*x, *y)).or_insert(current);
                open_set.push_back((*x, *y));
            }
        }
        for (x, y) in &neighbors {
            if !open_set.contains(&(*x, *y)) && map[*x][*y] != 1 && !colse_set.contains(&(*x, *y)) {
                g_cost[*x][*y] = g_cost[current.0][current.1] + 1.0;
                h_cost[*x][*y] = manhattan_distance((*x, *y), end);
                f_cost[*x][*y] = g_cost[*x][*y] + h_cost[*x][*y];
                let _unused = camefrom.entry((*x, *y)).or_insert(current);
                open_set.push_back((*x, *y));
            }
        }
        open_set.make_contiguous().sort_unstable_by(|p1, p2| {
            f_cost[p1.0][p1.1].partial_cmp(&f_cost[p2.0][p2.1]).unwrap()
        });
    }

    Vec::new()
}

fn manhattan_distance(point1: (usize, usize), point2: (usize, usize)) -> f64 {
    let x_distance = (point1.0 as isize - point2.0 as isize).abs() as f64;
    let y_distance = (point1.1 as isize - point2.1 as isize).abs() as f64;

    x_distance + y_distance
}

fn find_path(
    camefrom: &HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
) -> Vec<Vec<usize>> {
    let mut current = start;
    let mut path = Vec::new();

    while let Some(&prev) = camefrom.get(&current) {
        path.push(prev);
        current = prev;
    }

    // Add the starting point
    path.push(start);
    // Reverse the path to have it in the correct order
    path.reverse();

    // Convert the path to a Vec<Vec<usize>>
    let path_as_vec_of_vec: Vec<Vec<usize>> = path.into_iter().map(|(x, y)| vec![x, y]).collect();

    path_as_vec_of_vec
}

fn get_diagonal_neighbors(col: usize, row: usize, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    // 左上角
    if point.0 > 0 && point.1 > 0 {
        neighbors.push((point.0 - 1, point.1 - 1));
    }

    // 右上角
    if point.0 < col - 1 && point.1 > 0 {
        neighbors.push((point.0 + 1, point.1 - 1));
    }

    // 左下角
    if point.0 > 0 && point.1 < row - 1 {
        neighbors.push((point.0 - 1, point.1 + 1));
    }

    // 右下角
    if point.0 < col - 1 && point.1 < row - 1 {
        neighbors.push((point.0 + 1, point.1 + 1));
    }

    neighbors
}

fn get_neighbors(col: usize, row: usize, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    // 上方
    if point.1 > 0 {
        neighbors.push((point.0, point.1 - 1));
    }

    // 下方
    if point.1 < row - 1 {
        neighbors.push((point.0, point.1 + 1));
    }

    // 左侧
    if point.0 > 0 {
        neighbors.push((point.0 - 1, point.1));
    }

    // 右侧
    if point.0 < col - 1 {
        neighbors.push((point.0 + 1, point.1));
    }

    neighbors
}
