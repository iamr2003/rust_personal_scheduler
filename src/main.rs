
// use integer times for now
//changing names is complex
struct Event{
    priority: i32, //eventually can become a trait/enum or something
    time_start: i64, //going to make everything inclusive for now, dw abt it
    time_end: i64,   //eventually will make start or end exclusive
    name: String //don't understand strings properly yet
}

impl Event{
    fn new(p:i32,t_s:i64,t_e:i64,n:String)->Event{
        Event{
            priority:p,
            time_start:t_s,
            time_end:t_e,
            name:n
        }
    }
    fn clone(&self)->Self{
        Self::new(self.priority,self.time_start,self.time_end,self.name.clone())
    }
}

struct EventRequest{
    priority: i32,
    duration: i64,
    name:String
}

struct Schedule{
    overall_start:i64,
    overall_end:i64,
    events: Vec<Event>
}

impl Schedule{
    //cloning everything for now, eventually refs would be preferred
    fn get_event(&self,i:i64)->Option<Event>{
        for e in &self.events{
            if e.time_start <= i && i >= e.time_end {
                return Some(e.clone());
            }
        }
        None
    }
    fn add_event(&mut self,e:EventRequest)->bool{
        //don't include priority yet, just insert where possible
        //go through a scan for openings to insert
        false
    }
}

fn main(){
    println!("Hello, world!");
}