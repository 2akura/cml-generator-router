fn main() {
    struct FsmRow {
    next_index: usize,  
    token_name: &'static str,
    }

let fsm: Vec<FsmRow> = vec![
    FsmRow { next_index: 2, token_name: "OSPF" },
    FsmRow { next_index: 1, token_name: "PID" },
    FsmRow { next_index: usize::MAX, token_name: "END" },
];

let token_input = "OSPF";
let mut snapshot : Vec <&str> = Vec::new();
//let mut getter;
let mut i = 0;
loop {
    let row = &fsm[i];  

    if token_input == row.token_name {
      
        
        let track = row.token_name;
        snapshot.push(track);
        let end_ptr = track;
        
    
            if end_ptr == "END" {
                break;
            }
            i = row.next_index;
        }
        //println!("Snapshot after traversal: {:?}", snapshot);
        //hooray, snapshot still live here, thus i just need 1 binder
    }
    println!("Snapshot after traversal: {:?}", snapshot);
}