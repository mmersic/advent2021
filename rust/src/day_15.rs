use std::cmp::Ordering;
use took::Timer;
use std::collections::BinaryHeap;

//cost, position
struct Node { cost: usize, position: usize }

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        return self == other;
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost < other.cost { Ordering::Greater }
        else if self.cost == other.cost { Ordering::Equal }
        else { Ordering::Less }
    }
}

pub fn part_one() {
    let timer = Timer::new();

    let x_len = 100;
    let y_len = 100;
    
    let graph = include_bytes!("../inputs/day15.input").to_vec().iter()
        .filter(|x| **x != b'\n')
        .map(|x| *x)
        .map(|x| x - b'0')
        .collect();

    let dist = dijkstra(&graph, 0*y_len+0, y_len*(y_len-1)+(x_len-1), x_len, y_len);
    
    println!("day 15, part 1: {} in time {}us", dist, timer.took().as_std().as_micros());
}

fn dijkstra(graph : &Vec<u8>, start : usize, end : usize, x_len: usize, y_len: usize) -> usize {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    dist[start] = 0;
    
    let mut heap = BinaryHeap::new();
    heap.push(Node{cost: 0, position: start});

    while let Some(Node { cost, position}) = heap.pop() {
        if position == end { 
            return cost; 
        }
        if cost > dist[position] { continue; }
        let x = position % x_len;
        let y = (position-x)/y_len;
        if x < (x_len-1) {
            let xx = x+1;
            let n1 = Node {cost: cost + graph[y*y_len+xx] as usize, position: y*y_len+xx};
            
            if n1.cost < dist[n1.position] {
                dist[n1.position] = n1.cost;
                heap.push(n1);
            }
        }
        if y < (y_len-1) {
            let yy = y+1;
            let n1 = Node {cost: cost + graph[yy*y_len+x] as usize, position: yy*y_len+x};

            if n1.cost < dist[n1.position] {
                dist[n1.position] = n1.cost;
                heap.push(n1);
            }
        }
        if x > 0 {
            let xx = x-1;
            let n1 = Node {cost: cost + graph[y*y_len+xx] as usize, position: y*y_len+xx};

            if n1.cost < dist[n1.position] {
                dist[n1.position] = n1.cost;
                heap.push(n1);
            }
        }
        if y > 0 {
            let yy = y-1;
            let n1 = Node {cost: cost + graph[yy*y_len+x] as usize, position: yy*y_len+x};

            if n1.cost < dist[n1.position] {
                dist[n1.position] = n1.cost;
                heap.push(n1);
            }
        }
    }

    return 1;
}


pub fn part_two() {
    let timer = Timer::new();

    let x_len = 100;
    let y_len = 100;

    let graph = include_bytes!("../inputs/day15.input").to_vec().iter()
        .filter(|x| **x != b'\n')
        .map(|x| *x)
        .map(|x| x - b'0')
        .collect();
    
    let e_graph = expand(graph, x_len, y_len, 5, 5);
    println!("took: {}", timer.took().as_std().as_micros());
    let dist = dijkstra(&e_graph, 0, (x_len*5*y_len*5)-1, x_len*5, y_len*5);

    // for y in 0 .. 50 {
    //     for x in 0 .. 50 {
    //         print!("{}", e_graph[y*50+x]);
    //     }
    //     println!();
    // } 
    
    println!("day 15, part 2: {} in time {}us", dist, timer.took().as_std().as_micros());
}


fn expand(graph : Vec<u8>, x_len : usize, y_len : usize, x_blocks : usize, y_blocks : usize) -> Vec<u8> {
    let mut e_graph : Vec<u8> = vec![0 ; y_len*x_len*5*5];
    
    for y_offset in 0 .. y_blocks {
        for x_offset in 0 .. x_blocks {
            let adj  = x_offset + y_offset;
            for yy in 0 .. y_len { 
                for xx in 0 .. x_len {
                    let mut val = graph[yy*y_len + xx] + adj as u8;
                    if val > 9 {
                        val %= 10;
                        val += 1;
                    }
                    e_graph[(y_blocks*x_len*(y_offset*y_len))+xx+(yy*x_blocks*x_len)+(x_offset as usize *x_len)] = val;
                }
            }
        }
    }
    
    return e_graph;
}
