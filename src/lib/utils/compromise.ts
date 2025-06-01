import nlp from 'compromise';

export function makeSlightlyDifferent(sentence: string): string {
  const doc = nlp(sentence);
  let sentenceObject = getNormalizedSentenceObject(sentence);
  return renderSentence(sentenceObject);
}

export function getNormalizedSentenceObject(sentence: string): object {
  const doc = nlp(sentence);
  let sentenceObject = {
    inferred_tense: "",
    words: [] as Word[]
  };

  sentenceObject.inferred_tense = getSentenceTense(sentence);

  let document = doc.document;

  let wordIndex = 0;

  document.forEach(line => {
    //console.log("-LINE->", line);
    line.forEach(word => {
      if(!word || !word.tags){ return; }
      let normalized = word.normal;
      if (word.chunk == "Verb"){
        const verbDoc = nlp(word.text).verbs();
        if (verbDoc.found) {
          const conjugation = verbDoc.conjugate()[0];
          const infinitive = verbDoc.toInfinitive().text();
          console.log(`Original verb: ${word.text}, Infinitive: ${infinitive}`, conjugation);
      
          // Example: convert to singular form if applicable
          const singular = verbDoc.toInfinitive().text();
          normalized = singular;
        }
      }
      
      sentenceObject.words.push({ index: wordIndex, original: word.text, normal: normalized, synonyms: [], tags: word.tags || new Set(), isPlural: word.tags.has("Plural") });
      wordIndex++;
    });
  });

  //return Object.keys(nlp.model().one.lexicon).find(key => key == doc.random().text()) || sentence;

  console.log("-SENTENCE OBJECT->", sentenceObject);

  return sentenceObject;
}

interface Word {
  index: number;
  original: string;
  normal: string;
  synonyms: string[];
  tags: Set<string>;
  isPlural: boolean;
}

function getSentenceTense(sentence: string): string {
  const doc = nlp(sentence);
  const verbs = doc.verbs();

  if (verbs.length === 0) return 'Unknown';

  // Count verb tenses
  const tenses = {
    Past: 0,
    Present: 0,
    Future: 0
  };

  verbs.json().forEach(v => {
    const tags = v.terms?.[0]?.tags || [];
    if (tags.includes('PastTense')) tenses.Past++;
    else if (tags.includes('PresentTense')) tenses.Present++;
    else if (tags.includes('FutureTense')) tenses.Future++;
  });

  const mostCommon = Object.entries(tenses).sort((a, b) => b[1] - a[1])[0];
  return mostCommon[1] > 0 ? mostCommon[0] : 'Unknown';
}


export function renderSentence(sentenceObject: object): string {
  let sentence = "";
  
  

  return sentence;
}
