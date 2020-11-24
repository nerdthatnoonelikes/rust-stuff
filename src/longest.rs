pub fn run() {
    let words = "fishes,sam,gollum,sauron,frodo,balrog".split(",");
    let mut listOfWords: Vec<&str> = vec![];
    let mut answer: Vec<&str> = Vec::new();
    let mut longest = 0;
    for x in words {
        listOfWords.push(x);
    }
    
    for x in &listOfWords {
       if x.len() > longest {
            longest = x.len();
       } 
    }

    for x in listOfWords {
        if x.len() == longest {
            answer.push(x)
        }
    }

    println!("{:?}", answer);
}
