
// use integer times for now
//changing names is complex
#[derive(Copy, Clone)]
struct Event{
    priority: i32, //eventually can become a trait/enum or something
    time_start: i64, //going to make everything inclusive for now, dw abt it
    time_end: i64,   //eventually will make start or end exclusive
    name: &'static str //don't understand &'static strs properly yet
}

impl Event{
    fn new(p:i32,t_s:i64,t_e:i64,n:&'static str)->Event{
        Event{
            priority:p,
            time_start:t_s,
            time_end:t_e,
            name:n
        }
    }

}

struct EventRequest{
    priority: i32,
    duration: i64,
    name:&'static str
}

impl EventRequest{
    fn new(n:&'static str,p:i32,d:i64)->EventRequest{
        EventRequest{
            priority:p,
            duration:d,
            name:n
        }
    }
}

struct Schedule{
    overall_start:i64,
    overall_end:i64,
    events: Vec<Event>, //should consider different data structures, but this is fine
    time_used: i64 //the times used overall
}

impl Schedule{
    fn new(end:i64)->Schedule{
        Schedule{
            overall_start:0,
            overall_end:end,
            events:Vec::new(),
            time_used:0
        }
    }
    //cloning everything for now, eventually refs would be preferred
    fn get_event(&self,i:i64)->Option<Event>{
        for e in &self.events{
            if e.time_start <= i && i <= e.time_end {
                return Some(e.clone());
            }
        }
        None
    }

    //wonder if I ran refactor for shortness later
    fn add_event(&mut self,e:&EventRequest)->bool{
        //don't include priority yet, just insert where possible
        //go through a scan for openings to insert
        if self.events.is_empty(){
            if self.overall_end-self.overall_start>=e.duration{
                self.events.push(Event{
                    priority:e.priority,
                    time_start:0,
                    time_end:e.duration,
                    name:e.name
                });
                self.time_used+=e.duration;
                return true;
            }
            else{
                return false;
            }
        }
        //if there isn't space for antyhing, skip
        if self.overall_end-self.overall_start-self.time_used<=e.duration{
            println!("No Space");
            false
        }
        else{
            //fault with sub here
            //need to think about edge cases, check end as well
            for i in 0..self.events.len()-1{
                if self.events[i].time_end -self.events[i+1].time_start >= e.duration{
                    self.events.insert(i,Event{
                        priority:e.priority,
                        time_start:self.events[i].time_end+1,
                        time_end:self.events[i].time_end+1+e.duration,
                        name:e.name
                    });
                    self.time_used+=e.duration;
                    return true;
                }
            }
            //extra check for end
            if self.overall_end - self.events[self.events.len()-1].time_end>=e.duration{
                self.events.push(Event{
                    priority:e.priority,
                    time_start:self.events[self.events.len()-1].time_end+1,
                    time_end:self.events[self.events.len()-1].time_end+1+e.duration,
                    name:e.name
                });
                self.time_used+=e.duration;
                return true;
            }
            println!("Bad packing, can't find space");
            false
        }
    }
    fn print_all(&self)->(){
        println!();
        println!("Printing Schedule:");
        println!("Time used: {}",self.time_used);
        for e in &self.events{
            println!("Name: {} Priority: {} Time Start: {} Time End: {}",e.name,e.priority,e.time_start,e.time_end);
        }
    }
}

fn main(){
    println!("Hello, world!");
    let mut sched = Schedule::new(300);
    assert!(sched.get_event(10).is_none());
    let running = EventRequest::new("Running",1,10);
    let sleeping = EventRequest::new("Sleep",1, 50);
    if !sched.add_event(&running){
        println!("Failed setting running");
    }
    if !sched.add_event(&sleeping){
        println!("Failed setting sleeping");
    }

    sched.print_all();
    //a bit messy, need to think more about best ways
    // println!("{}",e_3.unwrap().name);
    // assert!(*e_3.unwrap().name == *"Running");
    // assert!(*e_21.unwrap().name == *"Sleep");
}