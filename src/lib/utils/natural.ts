import natural from 'natural';
const wordnet = new natural.WordNet();

function getSynonyms(word: string): Promise<string[]> {
  return new Promise((resolve) => {
    wordnet.lookup(word, (results) => {
      const synonyms = new Set<string>();
      results.forEach(result => result.synonyms.forEach(s => synonyms.add(s)));
      resolve([...synonyms]);
    });
  });
}