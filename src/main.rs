use graph::Graph;

fn main() {
    let graph = Graph::new(
        vec![
            "Dirthmouth",           // 0
            "Forgotten Crossroads", // 1
            "Greenpath",            // 2
            "Fog Canion",           // 3
            "Fungal Wastes",        // 4
            "City of Tears",        // 5
            "Howling Cliffs",       // 6
            "Deepnest",             // 7
            "Kingdom's Edge",       // 8
            "Royal Waterways",      // 9
            "Crystal Peak",         // 10
            "Resting Grounds",      // 11
            "The Hive",             // 12
            "Ancient Basin",        // 13
            "Queen's Garden",       // 14
        ],
        &mut [
            (0, 1),
            (1, 2),
            (2, 3),
            (2, 4),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 8),
            (5, 9),
            (5, 10),
            (7, 11),
            (8, 12),
            (9, 13),
            (9, 14),
            (10, 11),
            (13, 14),
        ],
    );

    println!("{:?}", graph.topology_search());
}
