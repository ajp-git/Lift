use std::{collections::HashMap, cmp::{min, max}};

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    if queues.iter().all(|queue| queue.is_empty()) {
        let v=vec![0];
        return v;
    }
    let mut l=Lift::new(queues, capacity);
    l.run()
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
}

struct Lift{
    capacity: u32,
    queues:HashMap<u32,Vec<u32>>,
    direction: Direction,
    inside:Vec<u32>,
    level: u32,
    floors_visited:Vec<u32>,
}

impl Lift {
    fn new (queues: &[Vec<u32>], capacity: u32) -> Self {
        let mut t_queues=HashMap::new();

        for (i,v) in queues.iter().enumerate(){
            t_queues.insert(i as u32, v.clone());
        }

        println!("Lift.queue : {:?}", t_queues);
    
        Lift { capacity: capacity, queues: t_queues, direction: Direction::Up, inside: Vec::new(), level: 0, floors_visited: Vec::new() }
    }

    fn open_door(&mut self){
        self.floors_visited.push(self.level);
        self.let_out();
        self.let_in();
    }

    fn close_door(&self){

    }

    fn get_first_up(&mut self) -> Option<u32> {

        println!("going : {:?} at level {}", self.direction, self.level);

        #[derive(Debug)]
        struct Waiting_Outside{
            floor:u32,
            destination:u32,
        }

        let mut up_list:Vec<Waiting_Outside>=Vec::new(); // first u32 is level where people wait, second is destination floor

        for (&floor, queue) in &self.queues {
            // We read each waiting queue
            for &destination in queue{
                up_list.push(Waiting_Outside { floor, destination });
            }
        }

        // We have a Vec of WO
        // We need to find the fist floor above us going up

        let mut above_floors_up:Vec<&Waiting_Outside>=up_list.iter().filter(|f|f.destination> f.floor&&f.floor!=self.level).collect();
       // let mut above_floors_down:Vec<&Waiting_Outside>=up_list.iter().filter(|f|f.destination>self.level && f.floor> self.level && f.destination<f.floor).collect();
        let mut above_floors_down:Vec<&Waiting_Outside>=up_list.iter().filter(|f|f.destination<f.floor&&f.floor!=self.level).collect();
        
        let mut min_above_floor_up:Option<u32>=None;
        let mut max_above_floor_down:Option<u32>=None;
        
        if !above_floors_up.is_empty(){
            above_floors_up.sort_by_key(|f|f.floor);
            min_above_floor_up=Some(above_floors_up[0].floor);
        }
        if !above_floors_down.is_empty(){
            above_floors_down.sort_by_key(|f|f.floor);
            above_floors_down.reverse();
            max_above_floor_down=Some(above_floors_down[0].floor);
        }

        //println!("Above floors up: {:?}",min_above_floor_up);
        //println!("Above floors down: {:?}",max_above_floor_down);

        //println!("Processing 'inside' list...");

        let mut min_inside_floor_requested:Option<u32>=None;

        for &i in self.inside.iter() {
            if i > self.level {
                if min_inside_floor_requested.is_none() || i<min_inside_floor_requested.unwrap(){
                    min_inside_floor_requested=Some(i);
                    //println!("\tInside lift : {} is the nearest floor requested from {}", i, self.level);
                }
            }
        }

        let result_floor:Option<u32>=match (min_above_floor_up,max_above_floor_down, min_inside_floor_requested) {
            (Some(a),_,Some(b)) => Some(min(a,b)),
            (Some(a),_,None) => Some(a),
            (None, _, Some(c)) => Some(c),
            (None,Some(a),None) => {self.change_direction(); Some(a)},
            (None,None,None) => None,       
        };
//        println!("get_first_up end : {:?} at level {} going to {:?}", self.direction, self.level, result_floor);
        println!("get_first_up end going to {:?}", result_floor);
        println!("\tmin_above_floor_up :{min_above_floor_up:?}");
        println!("\tmax_above_floor_down :{max_above_floor_down:?}");
        println!("\tmin_inside_floor_requested :{min_inside_floor_requested:?}");
        /*
        if result_floor.is_some() && result_floor.unwrap()<self.level{
            self.change_direction();
        } */
        result_floor
    }
    
    
    fn get_first_down(&mut self)->Option<u32>{
 
        println!("Going : {:?} at level {}", self.direction, self.level);

        #[derive(Debug)]
        struct Waiting_Outside{
            floor:u32,
            destination:u32,
        }

        let mut down_list:Vec<Waiting_Outside>=Vec::new(); // first u32 is level where people wait, second is destination floor

        for (&floor, queue) in &self.queues {
            // We read each waiting queue
            for &destination in queue{
                down_list.push(Waiting_Outside { floor, destination });
            }
        }

        // We have a Vec of WO
        // We need to find the fist floor above us going up

        let mut under_floors_up:Vec<&Waiting_Outside>=down_list.iter().filter(|f|f.destination>f.floor&&f.floor!=self.level).collect();
        let mut under_floors_down:Vec<&Waiting_Outside>=down_list.iter().filter(|f|f.destination<f.floor&&f.floor!=self.level).collect();
        
        let mut min_under_floor_up:Option<u32>=None;
        let mut max_under_floor_down:Option<u32>=None;
        
        if !under_floors_up.is_empty(){
            under_floors_up.sort_by_key(|f|f.floor);
            min_under_floor_up=Some(under_floors_up[0].floor);
        }
        if !under_floors_down.is_empty(){
            under_floors_down.sort_by_key(|f|f.floor);
            under_floors_down.reverse();
            max_under_floor_down=Some(under_floors_down[0].floor);
        }

        let mut max_inside_floor_requested:Option<u32>=None;

        for &i in self.inside.iter() {
            if i < self.level {
                if max_inside_floor_requested.is_none() || i>max_inside_floor_requested.unwrap(){
                    max_inside_floor_requested=Some(i);
                }
            }
        }

        let result_floor:Option<u32>=match (min_under_floor_up,max_under_floor_down, max_inside_floor_requested) {
            (Some(a),_,Some(b))=> Some(min(a,b)),
            (None,_,Some(a))=>Some(a),
            (Some(a),_,None)=>Some(a),
            (None,Some(a),None) => {self.change_direction(); Some(a)},
            (None,None,None) => None,
        };
        println!("get_first_up end : {:?} at level {}", self.direction, self.level);
        /*
        if result_floor.is_some() && result_floor.unwrap()>self.level{
            self.change_direction();
        } */
        result_floor
    }

    fn change_direction(&mut self) {
        if self.direction==Direction::Up {
            self.direction=Direction::Down;
           // self.go_next_level();
        } else {
            self.direction=Direction::Up;
            //self.go_next_level();
        }
    }

    fn go_next_level(&mut self){

        println!("Go next level start : {:?} at level {}", self.direction, self.level);

        if !self.is_someone_waiting(){
            if self.level!=0{
                self.level=0;
                self.floors_visited.push(0);
            }
            return;
        }
        match self.direction {
            Direction::Up => {
                if let Some(l)=self.get_first_up(){
                    self.level=l;
                }        
            },
            Direction::Down => {
                if let Some(l)=self.get_first_down(){
                    self.level=l;
                }       
            },
        }
        println!("Go next level end : {:?} at level {}", self.direction, self.level);
    }
    
    fn remaining_slots(&self) -> bool{
        self.capacity-self.inside.len() as u32 >0 
    }

    fn let_in(&mut self) {
        println!("Let in at level {}", self.level);
        let mut next_level_queue: Vec<usize> = Vec::new(); // Use usize for indexing

        if let Some(current_level_queue) = self.queues.get_mut(&self.level) {
            let mut i = 0;
            while i < current_level_queue.len() {
                // Calculate remaining slots directly here
                let remaining_slots = self.capacity as usize - self.inside.len();
                if remaining_slots == 0 {
                    println!("Lift is full at {}",self.level);
                    break; // No more slots available, exit the loop
                }
                let l = current_level_queue[i];
                match self.direction {
                    Direction::Up => {
                        if l > self.level {
                            println!("{} is entering lift Up at level {}", l, self.level);
                            self.inside.push(l);
                            next_level_queue.push(i); // Mark for removal
                        }
                    },
                    Direction::Down => {
                        if l < self.level {
                            println!("{} is entering lift Down at level {}", l, self.level);
                            self.inside.push(l);
                            next_level_queue.push(i); // Mark for removal
                        }
                    },
                }
                i += 1; // Increment index
            }

            // Remove elements in reverse order to avoid shifting the remaining elements
            for &index in next_level_queue.iter().rev() {
                current_level_queue.remove(index);
            }
        }
    }
       
    fn let_out(&mut self){

        let out = self.inside.iter().filter(|&&f|f==self.level).count();
        println!("\nLet Out at level {} {} going out", self.level, out );

        self.inside.retain(|&f| f!=self.level);

    }

    fn run(&mut self) -> Vec<u32>{
        while self.is_someone_waiting() {
            self.open_door();
            self.go_next_level();
        }
        self.floors_visited.clone()
    }

    fn is_someone_waiting(&self)->bool {
        
        let mut q_wait:u32=0;
        for (l,q) in &self.queues{
            q_wait+=q.len() as u32;
        }
        self.inside.len()!=0 || q_wait !=0
    }
}

#[cfg(test)]
mod tests {
    use super::the_lift;
    
    fn print_queues(queues: &[Vec<u32>], capacity: u32) -> String {
        let mut result = format!("\nLift capacity = {capacity}\n\n Floor    Queue");
        for (i, q) in queues.iter().enumerate().rev() {
            result.push_str(&format!("\n{i:>4} .... {q:?}"));
        }
        result
    }

    fn do_test(queues: &[Vec<u32>], capacity: u32, expected: &[u32]) {
        let actual = the_lift(queues, capacity);
        assert_eq!(actual, expected,
            "\nYour result (left) did not match expected output (right) for the given queues:\n{}\n",
            print_queues(queues, capacity));
    }
    /*
    #[test]
    fn test_get_first_up() {
        // Create a new Lift instance with test data
        let mut lift = Lift {
            capacity: 5,
            queues: HashMap::new(),
            direction: Direction::Up,
            inside: HashMap::new(),
            level: 2,
            floors_visited: Vec::new(),
                        // ... (rest of the Lift struct initialization if needed)
        };

        // Populate the queues with test data
        lift.queues.insert(1, vec![3, 4]);
        lift.queues.insert(3, vec![5]);
        lift.queues.insert(4, vec![5, 6, 7]);

        // Call the get_first_up method
        let result = lift.get_first_up();

        // Define the expected result
        let expected = Some(3); // Assuming this is the expected 'up' value based on the test data

        // Assert that the result matches the expected value
        assert_eq!(result, expected, "The get_first_up method did not return the expected result.");
    }
 */
    #[test]
    fn test_up() {
        do_test(&[vec![], vec![], vec![5,5,5],vec![],vec![],vec![],vec![]], 5, &[0, 2, 5, 0]);
    }
    #[test]
    fn test_down() {
        do_test(&[vec![],vec![],vec![1],vec![],vec![],vec![],vec![]], 5, &[0, 2, 1, 0]);
    }
    #[test]
    fn test_up_and_up() {
        do_test(&[vec![],vec![3],vec![4],vec![],vec![5],vec![],vec![]], 5, &[0, 1, 2, 3, 4, 5, 0]);
    }
    #[test]
    fn test_fire() {
        do_test(&[vec![],vec![],vec![],vec![],vec![],vec![],vec![0,0,0,0,0,0,0,0,0]], 5, &[0,6,0,6,0]);
    }
    #[test]
    fn test_empty_buiding() {
        do_test(&[vec![],vec![],vec![],vec![],vec![],vec![],vec![]], 5, &[0]);
    }
    #[test]
    fn test_down_and_down() {
        do_test(&[vec![],vec![0],vec![],vec![],vec![2],vec![3],vec![]], 5, &[0, 5, 4, 3, 2, 1, 0]);
    }
    #[test]
    fn test_yoyo() {
        do_test(&[vec![],vec![],vec![4,4,4,4],vec![],vec![2,2,2,2],vec![],vec![]], 2, &[0, 2, 4, 2, 4, 2, 0]);
    }
    #[test]
    fn test_lift_full_up_and_down() {
        do_test(&[vec![3, 3, 3, 3, 3, 3],vec![],vec![],vec![],vec![],vec![4, 4, 4, 4, 4, 4],vec![]], 5, &[0, 3, 5, 4, 0, 3, 5, 4, 0]);
    }
    #[test]
    fn test_highlander() {
        do_test(&[vec![],vec![2],vec![3,3,3],vec![1],vec![],vec![],vec![]], 1, &[0, 1, 2, 3, 1, 2, 3, 2, 3, 0]);
    }
}
