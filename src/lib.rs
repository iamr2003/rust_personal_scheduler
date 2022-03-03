//writing a lib, integrate with main/transfer code over at somepoint
use wasm_bindgen::prelude::*;

//function FROM JS
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

//function TO JS
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use integer times for now
//changing names is complex
#[derive(Copy, Clone,serde_derive::Serialize, serde_derive::Deserialize)]
struct Event{
    priority: i32, //eventually can become a trait/enum or something
    time_start: i64, //going to make everything inclusive for now, dw abt it
    time_end: i64,   //eventually will make start or end exclusive
    name: &'static str //don't understand &'static strs properly yet
}

#[wasm_bindgen]
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

#[wasm_bindgen]
struct Schedule{
    overall_start:i64,
    overall_end:i64,
    events: Vec<Event>, //should consider different data structures, but this is fine
    time_used: i64 //the times used overall
}

impl Schedule{
    //get event at a specific time
    //eventually consider a data struct for faster reads(multimap? binarysearch/BST thing?)
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
            // println!("No Space");
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
            // println!("Bad packing, can't find opening");
            false
        }
    }
    //for debugging
    fn print_all(&self)->(){
        println!();
        println!("Printing Schedule:");
        println!("Time used: {}",self.time_used);
        for e in &self.events{
            println!("Name: {} Priority: {} Time Start: {} Time End: {}",e.name,e.priority,e.time_start,e.time_end);
        }
    }
}

//not working properly

//for export
#[wasm_bindgen]
impl Schedule{
    pub fn new(end:i64)->Schedule{
        Schedule{
            overall_start:0,
            overall_end:end,
            events:Vec::new(),
            time_used:0
        }
    }

    //probably should create a JSON out for the whole Event cluster, nothing else needed I think
    //this serialization might be dumb
    pub fn toJSONString(&self)->String{
        serde_json::to_string(&self.events).unwrap()
        //should have a type check on here, but I'll implement later
    }

    //I would like to adapt things more with the structs, but will do this for simplicity for now
    pub fn add_event_raw(&mut self,p:i32,d:i64)->(){
        //strings are being dumb
        let request = EventRequest::new("TEST",p,d);
        self.add_event(&request);
    }
}