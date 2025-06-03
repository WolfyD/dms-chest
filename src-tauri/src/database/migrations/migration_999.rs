use super::super::Migration;

pub fn get_migration() -> Migration {
    Migration {
        version: 999,
        description: "Insert example data, create indexes, and triggers".to_string(),
        up_sql: "
            -- Data --
            -- Campaign data --
            INSERT INTO campaigns (name, description, settings, created_at, updated_at) VALUES 
            ('Example Campaign', 'Example campaign for experimenting', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- World data --
            INSERT INTO worlds (name, description, created_at, updated_at) VALUES 
            ('Example World', 'Example world for experimenting', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Calendar data --
            INSERT INTO calendars (name, description, created_at, updated_at) VALUES 
            ('Example Calendar', 'Example calendar for experimenting', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO calendar_details (calendar_id, days_in_year, months_in_year, days_in_week, weeks_in_month, 
            months, days_of_week, hours_in_day, minutes_in_hour, seconds_in_minute, 
            pre_epoch_prefix, post_epoch_prefix, epoch_term, important_holidays, notable_events, 
            major_astronomical_events, major_astrological_events, major_historical_events, moon_phases, moon_phases_in_month, moon_phase_at_0, 
            created_at, updated_at) VALUES 
            (1, 365, 12, 7, 4, 
            '[\"January\", \"February\", \"March\", \"April\", \"May\", \"June\", \"July\", \"August\", \"September\", \"October\", \"November\", \"December\"]', 
            '[\"Monday\", \"Tuesday\", \"Wednesday\", \"Thursday\", \"Friday\", \"Saturday\", \"Sunday\"]', 
            24, 60, 60, 
            'BC', 'AD', 'Jesus', '[{ \"name\": \"Christmas\", \"month\": 12, \"day\": 25, \"description\": \"The birth of Jesus Christ\" }, { \"name\": \"New Years Day\", \"month\": 1, \"day\": 1, \"description\": \"The first day of the year\" }]', 
            '[{\"name\": \"A friends Birthday\", \"month\": 10, \"day\": 14, \"description\": \"A friends birthday\"}, {\"name\":\"The day I found a penny\", \"year\": 1999, \"month\": 1, \"day\": 1, \"description\": \"I found a penny on the ground\"}]', 
            '[{ \"name\":\"Total lunar eclipse\", \"month\": 10, \"day\": 14, \"description\": \"A total lunar eclipse occurs when the Earth, Moon, and Sun are in a straight line, with the Earth casting a shadow on the Moon\"}, {\"name\":\"Total solar eclipse\", \"month\": 10, \"day\": 14, \"description\": \"A total solar eclipse occurs when the Moon, Earth, and Sun are in a straight line, with the Moon casting a shadow on the Earth\"}]', 
            '[{ \"name\":\"Summer Solstice\", \"month\": 6, \"day\": 21, \"description\": \"The longest day of the year\"}, {\"Winter Solstice\", \"month\": 12, \"day\": 21, \"description\": \"The shortest day of the year\"}]', 
            '[{ \"name\": \"First man on the moon\", \"year\": 1969, \"month\": 7, \"day\": 20, \"description\": \"Neil Armstrong and Buzz Aldrin became the first humans to land on the Moon\"}, {\"name\": \"Sliced bread invented\", \"year\": 1928, \"month\": 1, \"day\": 1, \"description\": \"The first sliced bread was invented by Otto Frederick Rohwedder\"}]', 
            '[\"Full moon\", \"New moon\", \"First quarter\", \"Last quarter\", \"Waxing gibbous\", \"Waning gibbous\", \"Waxing crescent\", \"Waning crescent\"]', 
            8, 'Waxing crescent', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            

            -- Terrestrial and Astronomical Areas --
            INSERT INTO area_types (name, level) VALUES
            -- Level 1: Internal Structures
            ('Room', 1),
            ('Apartment', 2),
            ('Floor', 2),
            -- Level 3: Building Components / Specific Locations
            ('Building', 3),
            -- Level 4: Immediate Surroundings / Infrastructure
            ('Street', 4),
            ('Road', 4),
            ('Avenue', 4),
            ('Boulevard', 4),
            ('Lane', 4),
            ('Alley', 4),
            ('Block', 4), -- An urban block of buildings
            -- Level 5: Settlements / Small Landmasses (can often be settlements)
            ('Neighborhood', 5),
            ('Borough', 5),
            ('Ward', 5),
            ('Hamlet', 5),
            ('Village', 5),
            ('Town', 5),
            ('City', 5),
            ('Isle', 5), -- A small island, often a settlement itself
            ('Island', 5), -- A larger island, can contain multiple settlements
            -- Level 6: Urban Agglomerations
            ('Agglomeration', 6), -- Metropolitan area, collection of cities/towns
            ('Metropolitan Area', 6),
            -- Level 7: Local Administrative Divisions
            ('District', 7),
            ('County', 7),
            ('Prefecture', 7),
            ('Department', 7),
            ('Shire', 7),
            -- Level 8: Major Sub-National Divisions
            ('Region', 8), -- As an administrative division (e.g., within a country)
            ('Province', 8),
            ('State', 8),
            ('Territory', 8),
            -- Level 9: Countries
            ('Country', 9),
            -- Level 10: Supranational Unions
            ('Union', 10), -- E.g., European Union
            -- Level 11: Major Terrestrial Features
            ('Continent', 11),
            ('Ocean', 11),
            ('Subcontinent', 11),
            ('Landmass', 11),
            -- Level 12: Planetary Bodies
            ('Planet', 12),
            ('Moon', 12), -- Same level as Planet, as it's a primary celestial body
            -- Level 13: Stellar Systems
            ('Stellar System', 13),
            ('Planetary System', 13),
            -- Level 14: Star Clusters
            ('Star Cluster', 14),
            ('Open Cluster', 14),
            ('Globular Cluster', 14),
            -- Level 15: Galaxies
            ('Galaxy', 15),
            -- Level 16: Galaxy Groups
            ('Galaxy Group', 16),
            -- Level 17: Galaxy Clusters
            ('Galaxy Cluster', 17),
            -- Level 18: Superclusters
            ('Supercluster', 18),
            -- Level 19: Universe
            ('Universe', 19);

            -- World details data --
            INSERT INTO world_details (world_id, genre, tone, tech_level, magic_level, dominant_species, other_species, religions, pantheon, notable_landmarks, history, planar_structure, calendar_id, established_material, created_at, updated_at) VALUES 
            (1, 'Fantasy', 'Heroic', 'Medieval', 'High', '[]', '[]', '[]', '[]', '[]', '', 'Material Plane', 1, 'D&D 5e', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Locations data --
            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES 
            (1, null, false, true, 'Earth', 'The planet we live on', 38, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 1, true, true, 'Europe', 'The continent of Europe', 34, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 2, true, true, 'United Kingdom', 'The United Kingdom', 33, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 3, true, true, 'Wales', 'The country of Wales', 32, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            
            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 3, true, true, 'Scotland', 'The country of Scotland', 32, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            
            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 3, true, true, 'England', 'The country of England', 32, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            
            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 6, true, true, 'London', 'The capital of the United Kingdom', 18, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO locations (world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 7, true, true, 'Palace of Westminster', 'London SW1A 0AA, United Kingdom', 4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- House rules data --
            INSERT INTO house_rules (campaign_id, name, description, rules, created_at, updated_at) VALUES 
            (1, 'Example House Rule', 'Example house rule for experimenting', '[\"Instant kill on a crit!\", \"On your birthday you get to roll with advantage\"]', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Campaign details data --
            INSERT INTO campaign_details (campaign_id, world_id, campaign_type, party_level, party_size, themes, tone, starting_location_name, starting_location_id, win_conditions, 
            session_zero_notes, player_agreements, calendar_id, house_rules_id, difficulty_level, created_at, updated_at) VALUES 
            (1, 1, 'Adventure', 1, 4, '{}', 'Heroic', 'London', 7, '{ \"win_condition\": \"The party must save the world from the evil empire\" }', 
            'No notes', 'One bag of chips per session', 1, 1, 'medium', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Race data --
            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Human', 'A human is a human', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Human Details
            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (1, 30, 'Medium', '{}', '[\"Common\", \"Draconic\"]', 'Normal', 30, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 70, '170-180 cm', '50-70 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Human Traits
            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (1, 'Ability score increase', 'You gain a +1 bonus to one ability score of your choice.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (1, 'Age', 'Humans reach adulthood in their late teens and live less than a century.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (1, 'Alignment', 'Humans tend to lean towards no particular alignment. The best and the worst are found among them.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (1, 'Size', 'Humans vary widely in height and build, from barely 5 feet to well over 6 feet tall. Regardless of your position in that range, your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (1, 'Speed', 'Your base walking speed is 30 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (1, 'Languages', 'You can speak, read, and write Common and one other language of your choice.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Dwarf', 'A dwarf is a dwarf', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Dwarf Details
            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (2, 25, 'Medium', '{}', '[\"Common\", \"Dwarvish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 350, '140-180 cm', '50-70 kg', 'Lawful Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- Dwarf Traits
            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (2, 'Ability score increase', 'You gain a +2 bonus to Constitution and a -2 penalty to Charisma.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (2, 'Age', 'Dwarves mature at the same rate as humans but live a few decades longer.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (2, 'Alignment', 'Most dwarves are lawful, believing firmly in the benefits of a well-ordered society. They tend to be good, just, and lawful, and they are very protective of their loved ones.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (2, 'Size', 'Dwarves are between 4 and 5 feet tall and average about 120 pounds. Your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (2, 'Speed', 'Your base walking speed is 25 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (2, 'Languages', 'You can speak, read, and write Common and Dwarvish.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Elf', 'An elf is an elf', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (3, 30, 'Medium', '{}', '[\"Common\", \"Elvish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '160-180 cm', '40-60 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (3, 'Ability score increase', 'You gain a +2 bonus to Dexterity and a -2 penalty to Constitution.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (3, 'Age', 'Elves mature at the same rate as humans but live much longer. An elf typically reaches adulthood around the age of 100. An elf can live to be 750 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (3, 'Alignment', 'Elves are neutral, tending to be good. They are a good people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (3, 'Size', 'Elves are between 5 and 18 feet tall and average about 100 pounds. Your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (3, 'Speed', 'Your base walking speed is 30 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (3, 'Languages', 'You can speak, read, and write Common and Elvish.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Gnome', 'A gnome is a gnome', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (4, 25, 'Small', '{}', '[\"Common\", \"Gnomish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '160-180 cm', '40-60 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (4, 'Ability score increase', 'You gain a +2 bonus to Intelligence and a -2 penalty to Constitution.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (4, 'Age', 'Gnomes mature at the same rate as humans but live much longer. A gnome typically reaches adulthood around the age of 20. A gnome can live to be 350 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (4, 'Alignment', 'Gnomes are neutral, tending to be good. They are a good people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (4, 'Size', 'Gnomes are between 3 and 4 feet tall and average about 30 pounds. Your size is Small.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (4, 'Speed', 'Your base walking speed is 25 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (4, 'Languages', 'You can speak, read, and write Common and Gnomish.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Half-Elf', 'A half-elf is a half-elf', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (5, 30, 'Medium', '{}', '[\"Common\", \"Elvish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 180, '160-180 cm', '40-60 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (5, 'Ability score increase', 'You gain a +2 bonus to Charisma and a +1 bonus to two other ability scores of your choice.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (5, 'Age', 'Half-elves mature at the same rate as humans but live much longer. A half-elf typically reaches adulthood around the age of 20. A half-elf can live to be 180 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (5, 'Alignment', 'Half-elves are neutral, tending to be good. They are a good people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (5, 'Size', 'Half-elves are between 5 and 6 feet tall and average about 100 pounds. Your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (5, 'Speed', 'Your base walking speed is 30 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (5, 'Languages', 'You can speak, read, and write Common and Elvish.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Half-Orc', 'A half-orc is a half-orc', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (6, 30, 'Medium', '{}', '[\"Common\", \"Orc\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 75, '180-200 cm', '70-90 kg', 'Neutral Evil', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (6, 'Ability score increase', 'You gain a +2 bonus to Strength and a +1 bonus to Constitution.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (6, 'Age', 'Half-orcs mature at the same rate as humans but live a few decades longer. A half-orc typically reaches adulthood around the age of 14. A half-orc can live to be 75 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (6, 'Alignment', 'Half-orcs are neutral, tending to be evil. They are a violent people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (6, 'Size', 'Half-orcs are between 5 and 7 feet tall and average about 180 pounds. Your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (6, 'Speed', 'Your base walking speed is 30 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (6, 'Languages', 'You can speak, read, and write Common and Orc.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Halfling', 'A halfling is a halfling', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (7, 25, 'Small', '{}', '[\"Common\", \"Halfling\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 150, '90-120 cm', '20-30 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (7, 'Ability score increase', 'You gain a +2 bonus to Dexterity.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (7, 'Age', 'Halflings mature at the same rate as humans but live much longer. A halfling typically reaches adulthood around the age of 20. A halfling can live to be 150 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (7, 'Alignment', 'Halflings are neutral, tending to be good. They are a good people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (7, 'Size', 'Halflings are between 2 and 4 feet tall and average about 30 pounds. Your size is Small.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (7, 'Speed', 'Your base walking speed is 25 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (7, 'Languages', 'You can speak, read, and write Common and Halfling.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Tiefling', 'A tiefling is a tiefling', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (8, 30, 'Medium', '{}', '[\"Common\", \"Infernal\"]', 'Darkvision', 60, true, '[\"Fire\"]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '160-180 cm', '40-60 kg', 'Neutral Evil', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (8, 'Ability score increase', 'You gain a +2 bonus to Charisma and a +1 bonus to Intelligence.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Age', 'Tieflings mature at the same rate as humans but live much longer. A tiefling typically reaches adulthood around the age of 20. A tiefling can live to be 100 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Alignment', 'Tieflings are neutral, tending to be evil. They are a violent people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Size', 'Tieflings are between 5 and 6 feet tall and average about 100 pounds. Your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Speed', 'Your base walking speed is 30 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Languages', 'You can speak, read, and write Common and Infernal.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Darkvision', 'You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You cant discern color in darkness, only shades of gray.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (8, 'Hellish Resistance', 'You have resistance to fire damage.', 'resistance', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Dragonborn', 'A dragonborn is a dragonborn', true, false, null, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (9, 30, 'Medium', '{}', '[\"Common\", \"Draconic\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 80, '180-200 cm', '70-90 kg', 'Lawful Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (9, 'Ability score increase', 'You gain a +2 bonus to Strength and a +1 bonus to Charisma.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Age', 'Dragonborn mature at the same rate as humans but live a few decades longer. A dragonborn typically reaches adulthood around the age of 15. A dragonborn can live to be 80 years old.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Alignment', 'Dragonborn are lawful, tending to be good. They are a good people, and they often help those in need. They are not greedy and rarely try to dominate others.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Size', 'Dragonborn are between 6 and 7 feet tall and average about 250 pounds. Your size is Medium.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Speed', 'Your base walking speed is 30 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Languages', 'You can speak, read, and write Common and Draconic.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Draconic Ancestry', 'You have draconic ancestry. Choose one type of dragon from the Draconic Ancestry table. Your breath weapon and damage resistance are determined by the dragon type.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Breath Weapon', 'You can use your action to exhale destructive energy. Your draconic ancestry determines the size, shape, and damage type of the exhalation.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (9, 'Damage Resistance', 'You have resistance to the damage type associated with your draconic ancestry.', 'resistance', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Rock Gnome', 'A rock gnome is a rock gnome', true, true, 4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (10, 25, 'Small', '{}', '[\"Common\", \"Gnomish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '90-120 cm', '20-30 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (10, 'Ability score increase', 'Your Intelligence score increases by 1.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (10, 'Artificers Lore', 'Whenever you make an Intelligence (History) check related to magic items, alchemical objects, or technological devices, you can add twice your proficiency bonus, instead of any proficiency bonus you normally apply.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (10, 'Tinker', 'You have proficiency with artisans tools (tinkers tools). Using those tools, you can spend 1 hour and 10 gp worth of materials to construct a Tiny clockwork device (AC 5, 1 hp).', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Forest Gnome', 'A forest gnome is a forest gnome', true, true, 4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (11, 25, 'Small', '{}', '[\"Common\", \"Gnomish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '90-120 cm', '20-30 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (11, 'Ability score increase', 'Your Dexterity score increases by 1.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (11, 'Natural Illusionist', 'You know the minor illusion cantrip. Intelligence is your spellcasting ability for it.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (11, 'Speak with Small Beasts', 'Through sounds and gestures, you can communicate simple ideas with Small or smaller beasts.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('High Elf', 'A high elf is a high elf', true, true, 3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (12, 30, 'Medium', '{}', '[\"Common\", \"Elvish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '160-180 cm', '40-60 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (12, 'Ability score increase', 'Your Intelligence score increases by 1.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (12, 'Elf Weapon Training', 'You have proficiency with the longsword, shortsword, shortbow, and longbow.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (12, 'Cantrip', 'You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (12, 'Extra Language', 'You can speak, read, and write one extra language of your choice.', 'language', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Wood Elf', 'A wood elf is a wood elf', true, true, 3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (13, 35, 'Medium', '{}', '[\"Common\", \"Elvish\"]', 'Normal', 60, true, '[]', '[]', '[]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '160-180 cm', '40-60 kg', 'Neutral Good', '[\"Anywhere\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (13, 'Ability score increase', 'Your Wisdom score increases by 1.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (13, 'Elf Weapon Training', 'You have proficiency with the longsword, shortsword, shortbow, and longbow.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (13, 'Fleet of Foot', 'Your base walking speed increases to 35 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (13, 'Mask of the Wild', 'You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO races (name, description, is_standard_race, is_subrace, parent_race_id, created_at, updated_at) VALUES 
            ('Dark Elf (Drow)', 'A dark elf is a dark elf', true, true, 3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_details (race_id, speed, size_category, ability_score_bonuses, languages, vision_type, vision_range, can_be_pc_race, resistances, immunities, weaknesses, innate_spellcasting, natural_armor, natural_attacks, proficiencies, 
            type, lifespan, average_height, average_weight, typical_alignment, homeland_locations, culture_notes, society_notes, religious_beliefs, naming_conventions, favored_classes, common_professions, created_at, updated_at) VALUES 
            (14, 30, 'Medium', '{}', '[\"Common\", \"Elvish\"]', 'Darkvision', 120, true, '[]', '[]', '[\"Sunlight Sensitivity\"]', '[]', '[]', '[]', '[]', 
            'Humanoid', 100, '160-180 cm', '40-60 kg', 'Neutral Evil', '[\"Underdark\"]', '', '', '', '', '', '', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            INSERT INTO race_traits (race_id, name, description, trait_type, created_at, updated_at) VALUES 
            (14, 'Ability score increase', 'Your Charisma score increases by 1.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (14, 'Superior Darkvision', 'Your darkvision has a radius of 120 feet.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (14, 'Sunlight Sensitivity', 'You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.', 'weakness', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (14, 'Drow Magic', 'You know the dancing lights cantrip. When you reach 3rd level, you can cast the faerie fire spell once per day. When you reach 5th level, you can also cast the darkness spell once per day. Charisma is your spellcasting ability for these spells.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
            (14, 'Drow Weapon Training', 'You have proficiency with rapiers, shortswords, and hand crossbows.', 'feature', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            
            




            -- Indexes --
            -- campaigns:
                CREATE INDEX IF NOT EXISTS idx_campaigns_name ON campaigns(name);

            -- campaign_details:
                CREATE INDEX IF NOT EXISTS idx_campaign_details_house_rules_id ON campaign_details(house_rules_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_calendar_id ON campaign_details(calendar_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_world_id ON campaign_details(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_campaign_id ON campaign_details(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_campaign_id ON campaign_details(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_world_id ON campaign_details(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_calendar_id ON campaign_details(calendar_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_house_rules_id ON campaign_details(house_rules_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_campaign_type ON campaign_details(campaign_type);
                CREATE INDEX IF NOT EXISTS idx_campaign_details_starting_location_name ON campaign_details(starting_location_name);

            -- campaign_arcs:
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_world_id ON campaign_arcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_campaign_id ON campaign_arcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_campaign_id ON campaign_arcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_world_id ON campaign_arcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_name ON campaign_arcs(name);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_arc_type ON campaign_arcs(arc_type);
                CREATE INDEX IF NOT EXISTS idx_campaign_arcs_status ON campaign_arcs(status);

            -- campaign_npcs:
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_npc_id ON campaign_npcs(npc_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_world_id ON campaign_npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_campaign_id ON campaign_npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_campaign_id ON campaign_npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_world_id ON campaign_npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_npc_id ON campaign_npcs(npc_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_npcs_nickname ON campaign_npcs(nickname);

            -- npcs:
                CREATE INDEX IF NOT EXISTS idx_npcs_class_id ON npcs(class_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_world_id ON npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_campaign_id ON npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_campaign_id ON npcs(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_world_id ON npcs(world_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_class_id ON npcs(class_id);
                CREATE INDEX IF NOT EXISTS idx_npcs_name ON npcs(name);

            -- npc_details:
                CREATE INDEX IF NOT EXISTS idx_npc_details_npc_id ON npc_details(npc_id);
                CREATE INDEX IF NOT EXISTS idx_npc_details_npc_id ON npc_details(npc_id);

            -- campaign_factions:
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_faction_id ON campaign_factions(faction_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_world_id ON campaign_factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_campaign_id ON campaign_factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_campaign_id ON campaign_factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_world_id ON campaign_factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_campaign_factions_faction_id ON campaign_factions(faction_id);

            -- factions:
                CREATE INDEX IF NOT EXISTS idx_factions_world_id ON factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_factions_campaign_id ON factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_factions_campaign_id ON factions(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_factions_world_id ON factions(world_id);
                CREATE INDEX IF NOT EXISTS idx_factions_name ON factions(name);

            -- faction_details:
                CREATE INDEX IF NOT EXISTS idx_faction_details_faction_id ON faction_details(faction_id);
                CREATE INDEX IF NOT EXISTS idx_faction_details_faction_id ON faction_details(faction_id);
                CREATE INDEX IF NOT EXISTS idx_faction_details_faction_type ON faction_details(faction_type);

            -- worlds:
                CREATE INDEX IF NOT EXISTS idx_worlds_name ON worlds(name);

            -- world_details:
                CREATE INDEX IF NOT EXISTS idx_world_details_world_id ON world_details(world_id);
                CREATE INDEX IF NOT EXISTS idx_world_details_world_id ON world_details(world_id);

            -- locations:
                CREATE INDEX IF NOT EXISTS idx_locations_world_id ON locations(world_id);
                CREATE INDEX IF NOT EXISTS idx_locations_world_id ON locations(world_id);
                CREATE INDEX IF NOT EXISTS idx_locations_has_parent ON locations(has_parent);
                CREATE INDEX IF NOT EXISTS idx_locations_has_children ON locations(has_children);
                CREATE INDEX IF NOT EXISTS idx_locations_name ON locations(name);
                CREATE INDEX IF NOT EXISTS idx_locations_area_type_id ON locations(area_type_id);

            -- location_details:
                CREATE INDEX IF NOT EXISTS idx_location_details_map_id ON location_details(map_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_location_id ON location_details(location_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_location_id ON location_details(location_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_map_id ON location_details(map_id);
                CREATE INDEX IF NOT EXISTS idx_location_details_has_map ON location_details(has_map);

            -- maps:
                CREATE INDEX IF NOT EXISTS idx_maps_name ON maps(name);

            -- map_details:
                CREATE INDEX IF NOT EXISTS idx_map_details_image_id ON map_details(image_id);
                CREATE INDEX IF NOT EXISTS idx_map_details_map_id ON map_details(map_id);
                CREATE INDEX IF NOT EXISTS idx_map_details_map_id ON map_details(map_id);

            -- calendars:
                CREATE INDEX IF NOT EXISTS idx_calendars_name ON calendars(name);

            -- calendar_details:
                CREATE INDEX IF NOT EXISTS idx_calendar_details_calendar_id ON calendar_details(calendar_id);
                CREATE INDEX IF NOT EXISTS idx_calendar_details_calendar_id ON calendar_details(calendar_id);

            -- house_rules:
                CREATE INDEX IF NOT EXISTS idx_house_rules_campaign_id ON house_rules(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_house_rules_campaign_id ON house_rules(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_house_rules_name ON house_rules(name);

            -- players:
                CREATE INDEX IF NOT EXISTS idx_players_name ON players(name);
                CREATE INDEX IF NOT EXISTS idx_players_nickname ON players(nickname);

            -- player_availability:
                CREATE INDEX IF NOT EXISTS idx_player_availability_player_id ON player_availability(player_id);
                CREATE INDEX IF NOT EXISTS idx_player_availability_campaign_id ON player_availability(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_player_availability_campaign_id ON player_availability(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_player_availability_player_id ON player_availability(player_id);

            -- characters:
                CREATE INDEX IF NOT EXISTS idx_characters_player_id ON characters(player_id);
                CREATE INDEX IF NOT EXISTS idx_characters_campaign_id ON characters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_characters_campaign_id ON characters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_characters_player_id ON characters(player_id);
                CREATE INDEX IF NOT EXISTS idx_characters_name ON characters(name);

            -- character_details:
                CREATE INDEX IF NOT EXISTS idx_character_details_character_id ON character_details(character_id);
                CREATE INDEX IF NOT EXISTS idx_character_details_character_id ON character_details(character_id);

            -- races:
                CREATE INDEX IF NOT EXISTS idx_races_parent_race_id ON races(parent_race_id);
                CREATE INDEX IF NOT EXISTS idx_races_custom_to_world_id ON races(custom_to_world_id);
                CREATE INDEX IF NOT EXISTS idx_races_custom_to_campaign_id ON races(custom_to_campaign_id);
                CREATE INDEX IF NOT EXISTS idx_races_name ON races(name);
                CREATE INDEX IF NOT EXISTS idx_races_is_standard_race ON races(is_standard_race);
                CREATE INDEX IF NOT EXISTS idx_races_is_subrace ON races(is_subrace);

            -- race_details:
                CREATE INDEX IF NOT EXISTS idx_race_details_race_id ON race_details(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_details_race_id ON race_details(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_details_size_category ON race_details(size_category);
                CREATE INDEX IF NOT EXISTS idx_race_details_vision_type ON race_details(vision_type);
                CREATE INDEX IF NOT EXISTS idx_race_details_type ON race_details(type);

            -- race_traits:
                CREATE INDEX IF NOT EXISTS idx_race_traits_race_id ON race_traits(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_traits_race_id ON race_traits(race_id);
                CREATE INDEX IF NOT EXISTS idx_race_traits_name ON race_traits(name);
                CREATE INDEX IF NOT EXISTS idx_race_traits_trait_type ON race_traits(trait_type);

            -- classes:
                CREATE INDEX IF NOT EXISTS idx_classes_world_id ON classes(world_id);
                CREATE INDEX IF NOT EXISTS idx_classes_campaign_id ON classes(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_classes_campaign_id ON classes(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_classes_world_id ON classes(world_id);
                CREATE INDEX IF NOT EXISTS idx_classes_name ON classes(name);

            -- class_details:
                CREATE INDEX IF NOT EXISTS idx_class_details_class_id ON class_details(class_id);
                CREATE INDEX IF NOT EXISTS idx_class_details_class_id ON class_details(class_id);

            -- monsters:
                CREATE INDEX IF NOT EXISTS idx_monsters_world_id ON monsters(world_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_campaign_id ON monsters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_campaign_id ON monsters(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_world_id ON monsters(world_id);
                CREATE INDEX IF NOT EXISTS idx_monsters_name ON monsters(name);

            -- monster_details:
                CREATE INDEX IF NOT EXISTS idx_monster_details_monster_id ON monster_details(monster_id);
                CREATE INDEX IF NOT EXISTS idx_monster_details_monster_id ON monster_details(monster_id);

            -- items:
                CREATE INDEX IF NOT EXISTS idx_items_world_id ON items(world_id);
                CREATE INDEX IF NOT EXISTS idx_items_campaign_id ON items(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_items_campaign_id ON items(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_items_world_id ON items(world_id);
                CREATE INDEX IF NOT EXISTS idx_items_name ON items(name);

            -- item_details:
                CREATE INDEX IF NOT EXISTS idx_item_details_item_id ON item_details(item_id);
                CREATE INDEX IF NOT EXISTS idx_item_details_item_id ON item_details(item_id);

            -- spells:
                CREATE INDEX IF NOT EXISTS idx_spells_world_id ON spells(world_id);
                CREATE INDEX IF NOT EXISTS idx_spells_campaign_id ON spells(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_spells_campaign_id ON spells(campaign_id);
                CREATE INDEX IF NOT EXISTS idx_spells_world_id ON spells(world_id);
                CREATE INDEX IF NOT EXISTS idx_spells_name ON spells(name);

            -- spell_details:
                CREATE INDEX IF NOT EXISTS idx_spell_details_spell_id ON spell_details(spell_id);
                CREATE INDEX IF NOT EXISTS idx_spell_details_spell_id ON spell_details(spell_id);

            -- terrestrial_and_astronomical_areas:
                CREATE INDEX IF NOT EXISTS idx_area_types_level ON area_types (level);



            -- Triggers --
            
            -- Trigger to update 'updated_at' on every change for area_types
            CREATE TRIGGER update_area_types_updated_at
            AFTER UPDATE ON area_types
            FOR EACH ROW
            BEGIN
                UPDATE area_types SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
            END;



        ".to_string(),
        down_sql: "
            DROP TABLE IF EXISTS classes;
            DROP TABLE IF EXISTS class_details;
            DROP TABLE IF EXISTS monsters;
            DROP TABLE IF EXISTS monster_details;
            DROP TABLE IF EXISTS items;
            DROP TABLE IF EXISTS item_details;
            DROP TABLE IF EXISTS spells;
            DROP TABLE IF EXISTS spell_details;
        ".to_string(),
    }
} 