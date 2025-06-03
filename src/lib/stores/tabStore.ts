import { writable } from 'svelte/store';

export const activeTab = writable('worldbuilding');

// Define categories and their tools
export const categories = {
    worldbuilding: {
        icon: 'ri-earth-fill',
        name: 'World Building',
        description: 'Tools for crafting compelling settings, cultures, and legends — everything your world needs to feel ancient, alive, and a little dangerous.',
        tools: [
            ["Plot generator", "Plots galore — just add characters.",
             "Generate storylines, quest chains, or narrative seeds tailored to your setting, themes, and tone. Useful for both sandbox and structured campaigns."],
            ["Prophecy creator", "Cryptic mumblings with plot potential.",
             "Generate random or semi-guided prophecies with thematic flair and narrative hooks. Add mystery, fate, and doom to your world in seconds."],
            ["Region generator", "Need a place? We've got places.",
             "Creates unique regions complete with climate, terrain, settlements, and local flavor — fully customizable to suit your tone and geography."],
            ["World generator", "Birth an entire planet. No pressure.",
             "Generate a complete world with continents, biomes, major civilizations, and interwoven history. Randomized or controlled by parameters you set."],
            ["Culture tools", "Make your world feel lived-in.",
             "Craft cultures with defined rituals, holidays, economies, values, naming conventions, and social norms. Can also generate contrasts between regions."],
            ["Rumor builder", "Gossip, myths, and plot seeds!",
             "Creates local rumors, superstitions, tabloid gossip, or ancient legends to sprinkle flavor or hint at deeper lore. Great for taverns and travelers."],
            ["Timeline and history", "Every kingdom has a past.",
             "Auto-generate the major events, wars, rulers, discoveries, and disasters of any location — helps add depth and context to your world."],
            ["Language tools", "Because common gets boring.",
             "Build your own languages or naming schemes, generate simple ciphers, and structure linguistic rules for realism or fun."],
            ["Astronomical tools", "Worlds beyond your world.",
             "Manage multiple moons, stars, and constellations. Create celestial events like eclipses or comets with mechanical or narrative consequences."],
             ["Economy", "From copper commons to platinum politics — master your world's wealth.",
             "Manage your world's economic system, create price tables, simulate scarcity and trade, and dynamically generate item values based on regional or magical influences."]
          ]
    },
    campaign: {
        icon: 'ri-map-fill',
        name: 'Campaign',
        description: 'Your campaign control room: track quests, events, factions, and timelines without using up all your sticky notes.',
        tools: [
            ["Log taker", "Never forget what happened in 'that one session.'",
             "Create and manage session summaries, GM notes, prep checklists, and campaign-wide logs. Great for continuity and recaps."],
            ["Timeline & Arc Trackers", "Time is fake, but your story isn't.",
             "Track events across in-world time, chart character arcs and story beats. Includes visual timeline tools to show what happened when."],
            ["Quest tracker", "Quests: found, followed, or forgotten.",
             "Create and organize quests, their steps, status (open, ongoing, resolved), and associated NPCs or locations."],
            ["Faction info", "Because power has layers.",
             "Create detailed entries for organizations, factions, cults, guilds, and governments. Includes goals, rivalries, members, and public opinion."],
            ["Calendar tool", "What day is it in-universe again?",
             "Build in-world calendars with custom months, weeks, moons, and holidays. Link events and track dates across sessions."],
            ["Session 0 and safety tools", "Establish tone before the swordplay.",
             "Store and manage player preferences, session zero notes, tone agreements, lines/veils, and other safety tools for better group cohesion."],
            ["Player feedback", "The quiet ones have opinions too.",
             "Collect and store player feedback (anonymous or not), session reactions, and improvement suggestions. Great for long-term campaigns."]
          ]          
    },
    character: {
        icon: 'ri-user-fill',
        name: 'Characters',
        description: 'Everything you need to breathe life into your PCs and NPCs — or dig into their secrets, arcs, and grudges.',
        tools: [
            ["Name generator", "Your NPCs deserve more than Bob.",
             "Randomized name generator with cultural presets, fantasy tones, or alignment-based themes. Supports first/last name combos."],
            ["Backstory generator", "Tragic, mysterious, or just weird.",
             "Auto-generates character backstories with options for tone, origin, and drama level. Ideal for one-shots or quick NPCs."],
            ["NPC manager", "Juggle dozens of delightful weirdos.",
             "Create, edit, tag, and archive recurring or one-off NPCs. Store their stats, role, location, motives, and voice/personality quirks."],
            ["PC Goals", "Give your heroes something to strive for.",
             "Track and manage player characters' short-, mid-, and long-term goals. Helps you align quests to what they care about."],
            ["PC Backstory manager", "The lore behind the sheet.",
             "A centralized place to store and revisit player character backstories, motivations, and unresolved plot hooks."],
            ["Bonds", "Web of intrigue, or just who's dating whom.",
             "Create and track relationships between PCs and NPCs, including rivalries, romances, loyalty, history, and secret allegiances."],
            ["Progression tracker", "Level up with context.",
             "Track character growth across levels — includes skill improvements, key events, titles earned, and relationships developed over time."]
          ]          
    },
    mechanics: {
        icon: 'ri-settings-2-fill',
        name: 'Mechanics',
        description: "Customize your game's guts. From monster stats to magical loot, it's your system now.",
        tools: [
            ["Custom class builder", "Make your own weird fighter-wizard-rogue... thing.",
             "Build new classes or subclasses, complete with abilities, spellcasting, and progression. Includes balance warnings and export features."],
            ["Spell designer", "Fireball, but cooler.",
             "Design spells with parameters for damage, range, casting time, components, and thematic effects. Optionally auto-balance based on level."],
            ["Item creator", "Magic swords. Cursed teacups. Go wild.",
             "Create custom items — mundane, magical, or cursed — with effects, lore, and history. Includes item rarity and usage tracking."],
            ["Combat mat", "Organize chaos like a true warlord.",
             "Track initiative, HP, status effects, turn order, and behaviors. Includes tactical suggestions for enemy AI."],
            ["Encounter builder", "Balanced, brutal, or bananas.",
             "Generate encounters by CR, theme, terrain, or party strength. Includes loot suggestions and environmental features."],
            ["Trap, Puzzle & Hazard Creator", "Fun ways to hurt your party.",
             "Design puzzles, traps, environmental hazards, and skill challenges — includes triggers, clues, and fail consequences."],
            ["The hoard", "Shiny things and cursed artifacts await.",
             "Treasure hoard generator with coin counts, magical items, historical relics, and potential plot hooks tied to loot."],
            ["Monster brewer", "If it bites, we build it.",
             "Custom monster creator with statblock editing, lore writing, and template importing. Great for homebrew or reskinning existing creatures."],
            ["Dice tools", "Click clack math rock machine.",
             "Roll any combination of dice, add modifiers, apply advantage/disadvantage, or set up recurring custom rolls."]
          ]          
    },
    'audio-visual-aids': {
        icon: 'ri-music-2-fill',
        name: 'Media',
        description: 'The sights, sounds, and props that make your table immersive enough to smell the goblins.',
        tools: [
            ["Map generators", "Make maps faster than your players can derail them.",
             "Create maps for cities, dungeons, regions, or full worlds. Supports style presets (hand-drawn, parchment, minimalist)."],
            ["Scene setting tools", "Set the mood — literally.",
             "Generate weather, time of day, ambient sounds, and other environmental cues to bring scenes to life."],
            ["Props and handouts", "Feel it in your hands.",
             "Create printable or digital props like scrolls, letters, contracts, item cards, or puzzles. Stylized with in-world fonts and art."]
          ]
    },
    analytics: {
        icon: 'ri-bar-chart-2-fill',
        name: 'Analytics',
        description: 'See how your campaign ticks — and how to make it tick better.',
        tools: [
            ["Pacing Metrics", "Too much murder, not enough vibes?",
             "Track your campaign's balance of combat, roleplay, and exploration over time. Helps diagnose pacing issues and variety gaps."],
            ["Narrative Thread Health", "Plot lines in critical condition.",
             "Visualize which story arcs are active, neglected, or resolved. Great for keeping long-term threads alive."],
            ["PC Engagement Logs", "Who's zoning out and who's thriving?",
             "Track how much spotlight time each PC is getting. Helps keep engagement balanced and identify underused players."],
            ["Session Planning", "Prep with purpose.",
             "Plan out scenes, encounters, and beats for upcoming sessions with priority tagging, estimated durations, and PC involvement links."],
            ["Recap and debrief", "What just happened, and what did they think?",
             "Summarize key session events and collect player reactions, highlights, and issues post-game."],
            ["Player requests", "Your players want WHAT?",
             "Log what players have asked to do, see, or explore — helps inform session planning and prioritize content."]
          ]
    }
};

// Map of tab IDs to their content components
export const tabComponents = {
    worldbuilding: 'WorldBuilding',
    campaign: 'Campaign',
    character: 'Character',
    mechanics: 'Mechanics',
    'audio-visual-aids': 'AudioVisualAids',
    analytics: 'Analytics'
}; 