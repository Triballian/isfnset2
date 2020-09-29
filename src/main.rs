use std::collections::HashMap;
fn main() {
    let mut fnsets = HashMap::new();
    // I want to gather the data then process itm I don't want to process while gathering.
    fnsets.insert(1,2);
    fnsets.insert(6,4);
    fnsets.insert(1,4);

    let isfn: bool =  asfn(&fnsets);
    println!("fnsets: {:?} is a fn: {}", fnsets, isfn);

    

    fn asfn(fnsets: &HashMap<i32,i32>) -> bool{
        let mut isfn = true;
        let mut mdoms: HashMap<i32,Vec<i32>> = HashMap::new();
        for (k,v) in fnsets.iter() {
            if !mdoms.contains_key(&k) {
                // is the range is not empty and value is not already ther its not a function
                if mdoms.get(&k) != None {
                    if !mdoms.get(&k).unwrap().is_empty(){
                        if !mdoms.get(&k).unwrap().iter().any(|&r| r ==*v) { 
                            isfn = false;
                            return isfn
                        }
                    } else {
                    mdoms.get_mut(&k).unwrap().push(*v);
                    }
                }
                
            } else {
                mdoms.insert(*k,vec![*v]);
            }
            

        }
    return isfn


    }

    
}
