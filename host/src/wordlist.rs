
pub static WORDS: [&str; 90] = [
    "act", "ant", "ape", "arm", "ash", "ask", "bad", "bag", "bat", "bed", 
    "bee", "bet", "big", "bin", "bit", "box", "boy", "bun", "bus", "but", 
    "buy", "cab", "cat", "cow", "cry", "cut", "den", "dog", "dot", "dry", 
    "egg", "end", "eat", "far", "fat", "few", "fix", "fox", "fun", "gas", 
    "get", "guy", "hen", "hit", "hop", "hot", "hug", "jam", "jar", "jet",
    "kid", "lap", "let", "lid", "log", "lot", "man", "map", "mat", "net",
    "not", "nut", "oak", "oil", "opt", "pan", "pat", "pet", "pot", "put", 
    "rat", "rob", "rod", "row", "rub", "run", "sat", "set", "sip", "sit",
    "sob", "sun", "tap", "top", "toy", "use", "van", "vet", "won", "yet"
];

pub fn pick_word() -> &'static str {
    use rand::seq::SliceRandom;
    WORDS.choose(&mut rand::thread_rng()).unwrap()
}
