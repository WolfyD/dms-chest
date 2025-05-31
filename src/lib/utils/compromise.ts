import nlp from 'compromise';

function makeSlightlyDifferent(sentence: string): string {
  const doc = nlp(sentence);
  // Swap adjectives or verbs if desired
  return doc.sentences().toNegative().text(); // just an example of transformation
}