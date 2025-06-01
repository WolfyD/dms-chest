import tracery from 'tracery-grammar'; 
import { stringsToSettings, getWorldNameParts, toneSnippets, genreSnippets, techTemplates, magicTemplates, type WorldSettings } from './world';
import { readWordNetJson } from './paths';
import fs from 'fs';
import path from 'path';

type Genre =
  | 'Fantasy' | 'Sci-Fi' | 'Post-Apocalyptic' | 'Modern'
  | 'Historical' | 'Other';

type Tone =
  | 'Heroic' | 'Dark' | 'Grimdark' | 'Hopeful'
  | 'Tragic' | 'Comedic' | 'Mixed' | 'Other';

type TechLevel =
  | 'Stone Age' | 'Bronze Age' | 'Iron Age' | 'Medieval'
  | 'Renaissance' | 'Industrial' | 'Modern'
  | 'Near Future' | 'Far Future' | 'Magitech' | 'Other';

type MagicLevel =
  | 'None' | 'Low' | 'Moderate' | 'High'
  | 'Wild' | 'Divine Only' | 'Unknown' | 'Other';

const grammar = tracery.createGrammar({
  tone_intro: [
    'Its tone is #toneAdj# and #toneEffect#.',
    'The tone is best described as #toneAdj#, where #toneEffect#.',
    '#toneAdj.capitalize# themes dominate, #toneEffect#.',
  ],
  toneAdj: ['harsh', 'grim', 'bleak', 'merciless'],
  toneEffect: [
    'even hope can be a danger',
    'compassion rarely goes unpunished',
    'darkness overshadows every triumph'
  ]
});

const sentence = grammar.flatten('tone_intro');


export function generateWorldName(strings: string[]): { name: string; grammar: tracery.Grammar } {
  const settings = stringsToSettings(strings);
  const parts = getWorldNameParts(settings);

  const grammar = tracery.createGrammar({
    origin: ['#name#'],
    name: [
      '#prefix##suffix#',
      '#prefix##middle##suffix#',
      '#prefix#\'#suffix#',
      '#prefix#-#suffix#',
    ],
    prefix: parts.prefix,
    middle: parts.middle,
    suffix: parts.suffix,
  });

  grammar.addModifiers(tracery.baseEngModifiers); // optional: capitalize, etc.
  const name = grammar.flatten('#origin#');

  return { name, grammar };
}

function getTraceryGrammar(tone: Tone, genre: Genre, name: string): tracery.Grammar {
  const toneData = toneSnippets[tone] ?? toneSnippets.Other;
  const genreData = genreSnippets[genre] ?? genreSnippets.Other;

  const grammar = tracery.createGrammar({
    origin: [
      'The world of #worldName# is a #toneAdj# #genre# setting with #genreDetail#. #toneDetail#',
      '#worldName# is known for its #toneAdj# nature and #genreDetail#. #toneDetail#',
      '#worldName#, a #toneAdj# world of #genreDetail#, is shaped by #toneDetail#',
      'In the #toneAdj# world of #worldName#, #genreDetail# dominate. #toneDetail#',
      'Behold #worldName#, a #genre# realm, #toneAdj# and filled with #genreDetail#. #toneDetail#',
      'Named #worldName#, this #toneAdj# #genre# setting thrives on #genreDetail# and where #toneDetail#',
      '#worldName#: a #genre# world that is as #toneAdj# as it is shaped by #genreDetail#. #toneDetail#',
      'Across the lands of #worldName#, #genreDetail# define the #toneAdj# spirit of the world. #toneDetail#',
      'They call it #worldName# — a #toneAdj# world of #genre# strangeness. #toneDetail#',
      'In #worldName#, a #toneAdj# tale of #genreDetail# unfolds. #toneDetail#'
    ],
    worldName: name,
    toneAdj: toneData.toneAdj,
    genre: [genre],
    genreDetail: genreData,
    toneDetail: toneData.toneEffect.map(effect => `It is a place where ${effect}.`)
  });

  return grammar;
}

async function getWordNetJson(): Promise<any> {
  const wordNetJson = await readWordNetJson();
  return wordNetJson;
}

export async function getSynonyms(word: string): Promise<string[]> {
  const wordNetJson = await getWordNetJson();
  const wordnetData: WordNetEntry[] = JSON.parse(
    fs.readFileSync(path.join(__dirname, 'data', 'wordnet.json'), 'utf-8')
  );
  const entry = wordnetData.find(entry => entry.word.toLowerCase() === word.toLowerCase());
  return entry ? entry.synonyms : [];
}

export async function getWorldDescription(strings: string[]): Promise<string> {
  const world = stringsToSettings(strings);
  let description = await generateWorldDescription(world).then((description) => { return description; });

  console.log("--DESCRIPTION--", description);

  return description;
}

export async function generateWorldDescription(world: WorldSettings): Promise<string> {
  const grammar = getTraceryGrammar(world.tone, world.genre, world.name);
  grammar.addModifiers(tracery.baseEngModifiers);
  grammar.pushRules('name', [world.name]);

  const intro = grammar.flatten('#origin#');
  const techOptions = techTemplates[world.techLevel];
  const magicOptions = magicTemplates[world.magicLevel];

  const tech = techOptions[Math.floor(Math.random() * techOptions.length)];
  const magic = magicOptions[Math.floor(Math.random() * magicOptions.length)];

  return `${intro}\nTechnology level: ${tech}\nMagic: ${magic}`;
}

interface WordNetEntry {
  word: string;
  synonyms: string[];
}