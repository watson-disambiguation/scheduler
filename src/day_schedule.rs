use crate::timespan::TimeSpan;


pub struct DaySchedule {
    timespans: Vec<TimeSpan>,
}

impl DaySchedule {
    pub fn new() -> Self{
        DaySchedule { timespans: Vec::new() }
    }

    pub fn add(&mut self, timespan: TimeSpan) -> bool{
        if self.timespans.is_empty() {
            self.timespans.push(timespan);
            return true;
        }
        for i in 0..self.timespans.len() {
            let curr = self.timespans[i];

            match curr.overlap(timespan) {
                Some(_) => return false,
                None => (),
            };

            if timespan.end_time() < curr.start_time() {
                self.timespans.insert(i, timespan);
                return true;
            }
        }
        self.timespans.push(timespan);
        return true;
    }

    pub fn intersect(&self, other: &DaySchedule) -> DaySchedule{
        let mut output: DaySchedule = DaySchedule::new();
        let mut self_counter: usize = 0;
        let mut other_counter: usize = 0;
        let self_len = self.timespans.len();
        let other_len = other.timespans.len();
        while self_counter < self_len && other_counter < other_len {
            let self_curr = self.timespans[self_counter];
            let other_curr = other.timespans[other_counter];
            let overlap = self_curr.overlap(other_curr);
            match overlap {
                Some(o) => {
                    output.timespans.push(o);

                },
                None => (),
            }
            if self_curr.end_time() < other_curr.end_time() {
                self_counter += 1;
            }
            else {
                other_counter += 1;
            }
        }
        return output;
    }
}


impl ToString for DaySchedule {
    fn to_string(&self) -> String {
        let mut output = String::new();
        for span in &self.timespans {
            output += span.to_string().as_str();
            output += "\n";
        }
        output
    }
}

