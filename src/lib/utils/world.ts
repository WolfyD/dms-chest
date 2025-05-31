// Define world setting types
type Genre = 'Fantasy' | 'Sci-Fi' | 'Post-Apocalyptic' | 'Modern' | 'Historical' | 'Other';
type Tone = 'Heroic' | 'Dark' | 'Grimdark' | 'Hopeful' | 'Tragic' | 'Comedic' | 'Mixed' | 'Other';
type TechLevel =
  | 'Stone Age' | 'Bronze Age' | 'Iron Age' | 'Medieval' | 'Renaissance' | 'Industrial' | 'Modern' | 'Near Future' | 'Far Future' | 'Magitech' | 'Other';
type MagicLevel = 'None' | 'Low' | 'Moderate' | 'High' | 'Wild' | 'Divine Only' | 'Unknown' | 'Other';

interface WorldSettings {
  genre: Genre;
  tone: Tone;
  techLevel: TechLevel;
  magicLevel: MagicLevel;
}

interface NameParts {
  prefix: string[];
  middle: string[];
  suffix: string[];
}

// Master name parts dictionary
const nameSets: Record<string, Partial<NameParts>> = {
  default: {
    prefix: ['Ael', 'Tor', 'Zan', 'Vel', 'Mor', 'Lor', 'Ser', 'Nar'],
    middle: ['dor', 'zan', 'lith', 'rak', 'tur', 'ven', 'mal', 'gor'],
    suffix: ['ia', 'on', 'ar', 'um', 'oth', 'en', 'el', 'or'],
  },
  Fantasy: {
    prefix: ['Eld', 'Myth', 'Thal', 'Drak', 'Aeth', 'Fael', 'Nyx', 'Orin'],
    suffix: ['or', 'wyn', 'dell', 'mere', 'thalas', 'enor'],
  },
  'Sci-Fi': {
    prefix: ['Xen', 'Zy', 'Neo', 'Cry', 'Tau', 'Andro', 'Vel', 'Qar'],
    suffix: ['ion', 'ex', 'os', 'aris', 'tron', 'ax'],
  },
  'Post-Apocalyptic': {
    prefix: ['Ash', 'Ru', 'Scrap', 'Noct', 'Blight', 'Grim'],
    suffix: ['dust', 'burn', 'waste', 'fall', 'core'],
  },
  'Stone Age': {
    prefix: ['Ur', 'Gha', 'Kro', 'Zog', 'Bak', 'Thok', 'Rug'],
    suffix: ['mak', 'ug', 'noth', 'gar'],
  },
  'Bronze Age': {
    prefix: ['Akk', 'Sume', 'Uruk', 'Nebu', 'Kish', 'Zana'],
    suffix: ['ad', 'ash', 'el', 'tu'],
  },
  Medieval: {
    prefix: ['Caer', 'Thorn', 'Bryn', 'Dun', 'Aldr', 'West', 'Glen'],
    suffix: ['helm', 'ford', 'wick', 'mere', 'stead'],
  },
  Magitech: {
    prefix: ['Aether', 'Runo', 'Volt', 'Arka', 'Mech', 'Spark', 'Chrono'],
    suffix: ['core', 'forge', 'spark', 'tech', 'drive'],
  },
  High: {
    prefix: ['Cel', 'Lum', 'Ely', 'Div', 'Seraph', 'Hal'],
    suffix: ['aris', 'iel', 'orne', 'ion'],
  },
  Heroic: {
    prefix: ['Val', 'Sol', 'Astra', 'Lum', 'Aeg', 'Glor', 'Thalor', 'Elan', 'Seren', 'Nobil'],
    suffix: ['ion', 'or', 'helm', 'eth', 'wyn', 'spire', 'reach', 'crest'],
  },
  Dark: {
    prefix: ['Noct', 'Dusk', 'Shad', 'Umb', 'Varn', 'Teneb', 'Gloam', 'Nyx', 'Velk', 'Murk'],
    suffix: ['mur', 'vex', 'bane', 'lok', 'gloom', 'veil', 'wretch'],
  },
  Grimdark: {
    prefix: ['Morb', 'Gor', 'Varn', 'Dreg', 'Skul', 'Nark', 'Blud', 'Rot', 'Karn', 'Damn'],
    suffix: ['thak', 'hul', 'gore', 'vorn', 'drak', 'blight', 'slag', 'doom'],
  },
  Hopeful: {
    prefix: ['Aure', 'Sol', 'Bright', 'Elen', 'Hal', 'Seren', 'Lum', 'Alva', 'Dawn', 'Ever'],
    suffix: ['light', 'rise', 'haven', 'wyn', 'song', 'hope', 'star', 'sky'],
  },
  Tragic: {
    prefix: ['Mourn', 'Lach', 'Sorrow', 'Eler', 'Thren', 'Ash', 'Fall', 'Mira', 'Wane', 'Fey'],
    suffix: ['lament', 'veil', 'shade', 'mourne', 'dirge', 'thorn', 'wane', 'dusk'],
  },
  Comedic: {
    prefix: ['Bop', 'Wig', 'Snor', 'Quib', 'Zan', 'Guff', 'Wob', 'Floop', 'Noodle', 'Blim'],
    suffix: ['oodle', 'snort', 'whack', 'flap', 'plop', 'nugget', 'bungle', 'kabob'],
  },
  Mixed: {
    prefix: ['Eclip', 'Mira', 'Twil', 'Junc', 'Chim', 'Flux', 'Zyph', 'Duo', 'Par', 'Myth'],
    suffix: ['gleam', 'rift', 'song', 'mourne', 'pulse', 'shade', 'bloom', 'warp'],
  },
  Other: {
    prefix: ['Xo', 'Qua', 'Jin', 'Omn', 'Nir', 'Yul', 'Tzi', 'Rhe', 'Azo', 'Kah'],
    suffix: ['ra', 'en', 'ul', 'ith', 'zum', 'osh', 'ari', 'vox'],
  },
};

// Merge helper
function mergeParts(keys: string[]): NameParts {
  const parts: NameParts = { prefix: [], middle: [], suffix: [] };

  for (const key of keys) {
    const set = nameSets[key];
    if (set) {
      parts.prefix.push(...(set.prefix ?? []));
      parts.middle.push(...(set.middle ?? []));
      parts.suffix.push(...(set.suffix ?? []));
    }
  }

  // Fallbacks
  const fallback = nameSets.default!;
  if (parts.prefix.length === 0) parts.prefix.push(...fallback.prefix!);
  if (parts.middle.length === 0) parts.middle.push(...fallback.middle!);
  if (parts.suffix.length === 0) parts.suffix.push(...fallback.suffix!);

  return parts;
}

export function stringsToSettings(strings: string[]): WorldSettings {
  return {
    genre: strings[0] as Genre,
    tone: strings[1] as Tone,
    techLevel: strings[2] as TechLevel,
    magicLevel: strings[3] as MagicLevel,
  };
}


// Main generator logic
export function getWorldNameParts(settings: WorldSettings): NameParts {
  const { genre, tone, techLevel, magicLevel } = settings;
  const activeKeys: string[] = [];

  console.log("settings", settings);

  // Priority order
  if (tone !== 'Other' && tone in nameSets) { activeKeys.push(tone); console.log("tone", tone); }
  if (techLevel !== 'Other' && techLevel in nameSets) { activeKeys.push(techLevel); console.log("techLevel", techLevel); }
  if (genre !== 'Other' && genre in nameSets) { activeKeys.push(genre); console.log("genre", genre); }
  if (magicLevel !== 'Other' && magicLevel in nameSets) { activeKeys.push(magicLevel); console.log("magicLevel", magicLevel); }

  if (activeKeys.length === 0) { activeKeys.push('default'); console.log("activeKeys", activeKeys); }

  return mergeParts(activeKeys);
}

// export function generateWorldName(strings: string[]) {
//   const settings = stringsToSettings(strings);
//   const parts = getWorldNameParts(settings);
//   return parts.prefix[Math.floor(Math.random() * parts.prefix.length)] + parts.middle[Math.floor(Math.random() * parts.middle.length)] + parts.suffix[Math.floor(Math.random() * parts.suffix.length)];
// }


// type Genre =
//   | 'Fantasy' | 'Sci-Fi' | 'Post-Apocalyptic' | 'Modern'
//   | 'Historical' | 'Other';

// type Tone =
//   | 'Heroic' | 'Dark' | 'Grimdark' | 'Hopeful'
//   | 'Tragic' | 'Comedic' | 'Mixed' | 'Other';

// type TechLevel =
//   | 'Stone Age' | 'Bronze Age' | 'Iron Age' | 'Medieval'
//   | 'Renaissance' | 'Industrial' | 'Modern'
//   | 'Near Future' | 'Far Future' | 'Magitech' | 'Other';

// type MagicLevel =
//   | 'None' | 'Low' | 'Moderate' | 'High'
//   | 'Wild' | 'Divine Only' | 'Unknown' | 'Other';

interface WorldSettings {
  name: string;
  genre: Genre;
  tone: Tone;
  techLevel: TechLevel;
  magicLevel: MagicLevel;
}

export const genreSnippets: Record<Genre, string[]> = {
  Fantasy: [
    'arcane kingdoms', 'enchanted forests', 'dragon-infested skies',
    'prophecies foretold', 'ancient ruins', 'forgotten gods',
    'mages and monsters', 'cursed lands', 'knightly orders',
    'elven realms', 'dwarven strongholds', 'mystic portals'
  ],
  'Sci-Fi': [
    'stellar empires', 'cybernetic life', 'AI overlords',
    'alien diplomacy', 'interstellar warzones', 'terraforming projects',
    'space piracy', 'quantum anomalies', 'galactic trade routes',
    'robotic revolutions', 'black hole mining', 'plasma rifles'
  ],
  'Post-Apocalyptic': [
    'radioactive ruins', 'scarcity and decay', 'mutated creatures',
    'warlord enclaves', 'shattered cities', 'broken satellites',
    'scrap-based technology', 'dust storms', 'barren wastelands',
    'raider clans', 'forgotten tech', 'hardened survivors'
  ],
  Modern: [
    'urban sprawls', 'contemporary dilemmas', 'political unrest',
    'global pandemics', 'corporate empires', 'digital surveillance',
    'tech addiction', 'climate anxiety', 'suburban secrets',
    'street protests', 'viral fame', 'cyber crimes'
  ],
  Historical: [
    'warring dynasties', 'plague-ridden cities', 'epic sieges',
    'royal intrigue', 'religious crusades', 'exploration fleets',
    'ancient philosophies', 'traditional garb', 'sword duels',
    'slave rebellions', 'cultural revolutions', 'naval supremacy'
  ],
  Other: [
    'twisting conventions', 'alien cultures', 'timeless myths',
    'unclassifiable worlds', 'meta-narratives', 'genre mashups',
    'dream logic', 'surreal realms', 'interdimensional flux',
    'living architecture', 'sentient landscapes', 'chaotic histories'
  ]
};

export const toneSnippets: Record<Tone, { toneAdj: string[], toneEffect: string[] }> = {
  Heroic: {
    toneAdj: ['noble', 'inspiring', 'grand', 'glorious', 'valiant', 'courageous', 'epic', 'resolute', 'chivalrous', 'steadfast', 'legendary', 'honorable'],
    toneEffect: ['valor is rewarded', 'evil is challenged', 'heroes rise', 'legends are born', 'sacrifice brings victory', 'justice prevails', 'the weak are protected', 'great deeds echo forever', 'hope shines bright', 'courage shapes destiny', 'honor matters', 'glory awaits']
  },
  Dark: {
    toneAdj: ['shadowed', 'ominous', 'hopeless', 'bleak', 'gloomy', 'murky', 'foreboding', 'grim', 'tenebrous', 'dismal', 'stifling', 'haunting'],
    toneEffect: ['danger lurks everywhere', 'peace is fleeting', 'trust is rare', 'fear dominates', 'secrets fester', 'betrayal is common', 'no one is safe', 'shadows conceal truths', 'light is swallowed', 'hope is crushed', 'truth is buried', 'silence screams']
  },
  Grimdark: {
    toneAdj: ['brutal', 'merciless', 'bleak', 'harsh', 'vicious', 'ruthless', 'miserable', 'bloody', 'savage', 'cruel', 'violent', 'unforgiving'],
    toneEffect: ['hope is a liability', 'death comes swiftly', 'power corrupts absolutely', 'survival is all that matters', 'empathy is weakness', 'violence begets violence', 'loyalty is exploited', 'suffering is common', 'justice is a lie', 'compassion is rare', 'victory is hollow', 'darkness reigns']
  },
  Hopeful: {
    toneAdj: ['uplifting', 'resilient', 'bright', 'optimistic', 'encouraging', 'light-hearted', 'inspired', 'motivated', 'cheerful', 'spirited', 'joyous', 'buoyant'],
    toneEffect: ['hope burns eternal', 'people rise together', 'light prevails', 'dreams are possible', 'kindness spreads', 'peace can be achieved', 'compassion thrives', 'change is coming', 'growth is possible', 'beauty emerges from pain', 'stars still shine', 'tomorrow brings light']
  },
  Tragic: {
    toneAdj: ['sorrowful', 'melancholic', 'fateful', 'lamentable', 'mournful', 'doomed', 'regretful', 'heartbreaking', 'bittersweet', 'somber', 'grieving', 'doleful'],
    toneEffect: ['loss is inevitable', 'glory fades', 'destiny is cruel', 'tears fall often', 'heroes die young', 'love is fleeting', 'joy is short-lived', 'memories haunt', 'mistakes linger', 'tragedy strikes', 'fates entwine in sorrow', 'every triumph costs']
  },
  Comedic: {
    toneAdj: ['absurd', 'wacky', 'light-hearted', 'silly', 'goofy', 'zany', 'irreverent', 'cheeky', 'whimsical', 'hilarious', 'quirky', 'nonsensical'],
    toneEffect: ['trouble is funny', 'chaos brings laughter', 'nothing is taken seriously', 'plans always fail in amusing ways', 'jokes abound', 'seriousness is mocked', 'missteps are common', 'luck is strange', 'weirdness is normal', 'blunders save the day', 'sarcasm reigns', 'giggles echo everywhere']
  },
  Mixed: {
    toneAdj: ['conflicted', 'nuanced', 'layered', 'shifting', 'blended', 'ambiguous', 'multifaceted', 'inconstant', 'dual-natured', 'mosaic', 'kaleidoscopic', 'complex'],
    toneEffect: ['joy and despair coexist', 'balance shifts constantly', 'nothing is black or white', 'meaning is subjective', 'truth is fluid', 'each story is unique', 'heroes have flaws', 'villains show grace', 'outcomes are uncertain', 'emotions collide', 'harmony and chaos intertwine', 'tension never ends']
  },
  Other: {
    toneAdj: ['uncategorized', 'surreal', 'alien', 'unusual', 'undefinable', 'bizarre', 'peculiar', 'esoteric', 'offbeat', 'eccentric', 'enigmatic', 'weird'],
    toneEffect: ['logic bends', 'emotion is strange', 'moods shift wildly', 'conventions dissolve', 'reality is unstable', 'understanding slips', 'narratives spiral', 'symbols dominate', 'time flows differently', 'names mean nothing', 'normalcy is absent', 'everything is symbolic']
  }
};

export const techTemplates: Record<TechLevel, string[]> = {
  'Stone Age': [
    'Primitive tools and tribal knowledge shape survival.',
    'Tribes live off the land with stone and bone tools.',
    'A time of fire and flint, where nature dominates all.'
  ],
  'Bronze Age': [
    'Early metallurgy and mythic rulers dominate civilization.',
    'Bronze tools and divine kings form early empires.',
    'City-states rise under the weight of myth and metal.'
  ],
  'Iron Age': [
    'Iron blades and ancient empires contend for power.',
    'The clash of iron shapes conquest and culture.',
    'Forged steel arms feudal lords and invaders alike.'
  ],
  Medieval: [
    'Knights, kingdoms, and superstition rule the land.',
    'Castles stand tall over peasant-filled fields.',
    'Faith and feudalism dictate daily life.'
  ],
  Renaissance: [
    'Innovation blooms amid intrigue and revolution.',
    'Science and art flourish in noble courts.',
    'The printing press spreads dangerous ideas.'
  ],
  Industrial: [
    'Steam and coal power factories and social upheaval.',
    'The engine of progress churns through smoke and steel.',
    'Industry awakens both hope and exploitation.'
  ],
  Modern: [
    'Technology is widespread and the world interconnected.',
    'Nations compete in media, warfare, and innovation.',
    'Every home pulses with the light of devices.'
  ],
  'Near Future': [
    'AI, spaceflight, and new frontiers loom.',
    'Humanity reaches for Mars amid digital wonders.',
    'The edge of tomorrow is always within sight.'
  ],
  'Far Future': [
    'Civilizations span galaxies and challenge reality itself.',
    'Ancient stars hold the secrets of posthuman fate.',
    'Mind and matter blend in the cosmic unknown.'
  ],
  Magitech: [
    'Magic and machinery merge into wondrous devices.',
    'Arcane cores drive mechanical marvels.',
    'Enchanted circuits pulse with power and mystery.'
  ],
  Other: [
    'Technological progress defies usual classification.',
    'The rules of invention twist in unrecognizable ways.',
    'No clear timeline defines development here.'
  ]
};

export const magicTemplates: Record<MagicLevel, string[]> = {
  None: [
    'Magic is absent, and only mundane forces shape the world.',
    'No supernatural forces interfere with life here.',
    'Reality runs on rules, not rituals.'
  ],
  Low: [
    'Rare and subtle magic exists, often misunderstood.',
    'Magic whispers in dark corners and forgotten places.',
    'Enchantment is scarce but impactful.'
  ],
  Moderate: [
    'Magic is present and known, but not all-powerful.',
    'Spells and rituals aid society without overwhelming it.',
    'Sorcery is a craft with limits and laws.'
  ],
  High: [
    'Magic permeates life and dictates the fate of nations.',
    'Wizards and witches shape reality as they please.',
    'The arcane rules over politics, war, and daily life.'
  ],
  Wild: [
    'Magic is unstable, powerful, and unpredictable.',
    'Casting spells is as dangerous as it is potent.',
    'Magical energy defies all attempts at control.'
  ],
  'Divine Only': [
    'Only deities or their chosen may wield supernatural power.',
    'Miracles replace spells in a world of divine will.',
    'The gods are the sole bearers of magical might.'
  ],
  Unknown: [
    'Magic exists in hidden places, undefined and mysterious.',
    'No one agrees on the truth of magic here.',
    'Legends hint at arcane power none understand.'
  ],
  Other: [
    'Magical laws differ from any known world.',
    'Magic behaves strangely, breaking familiar rules.',
    'This world rewrites the very definition of magic.'
  ]
};


