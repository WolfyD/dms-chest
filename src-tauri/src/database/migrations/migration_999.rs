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


            -- Set the world_id (assuming it exists in your 'worlds' table)
            -- For demonstration, we'll use 1.
            -- If you need to create a 'worlds' table, it would look something like:
            -- CREATE TABLE IF NOT EXISTS worlds (
            --     id INTEGER PRIMARY KEY,
            --     name TEXT NOT NULL UNIQUE,
            --     description TEXT,
            --     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            --     updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            -- );
            -- INSERT INTO worlds (name, description) VALUES ('Our Universe', 'The observable universe.');

            -- Pre-fetch area_type_ids for clarity and reuse in a single script
            -- In a real application, you might cache these in your code
            -- or use these subqueries directly as done below.

            -- Astronomical Area Type IDs
            -- SELECT id FROM area_types WHERE name = 'Supercluster';
            -- SELECT id FROM area_types WHERE name = 'Galaxy Cluster';
            -- SELECT id FROM area_types WHERE name = 'Galaxy Group';
            -- SELECT id FROM area_types WHERE name = 'Galaxy';
            -- SELECT id FROM area_types WHERE name = 'Stellar System';
            -- SELECT id FROM area_types WHERE name = 'Planet';

            -- Terrestrial Area Type IDs
            -- SELECT id FROM area_types WHERE name = 'Continent';
            -- SELECT id FROM area_types WHERE name = 'Country';
            -- SELECT id FROM area_types WHERE name = 'Province';
            -- SELECT id FROM area_types WHERE name = 'City';


            -- Start of INSERT statements for locations and their details
            -- Assigning explicit IDs for easier parent_id referencing

            -- 1. Laniakea Supercluster
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (1, 1, NULL, FALSE, TRUE, 'Laniakea Supercluster', 'The galaxy supercluster that is home to the Milky Way and approximately 100,000 other galaxies.', (SELECT id FROM area_types WHERE name = 'Supercluster'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (1, NULL, 'Home to the Milky Way', 'Cosmic Web filaments', 'Vacuum', 1, '[\"Virgo Supercluster\", \"Hydra-Centaurus Supercluster\"]', 'Defined in 2014, it represents the largest gravitationally bound structure our galaxy belongs to.', '[]', 'The boundaries are still under study.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 2. Virgo Cluster (Part of Laniakea, a prominent Galaxy Cluster)
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (2, 1, 1, TRUE, TRUE, 'Virgo Cluster', 'The largest galaxy cluster in the Virgo Supercluster, containing about 1300 (and possibly up to 2000) member galaxies.', (SELECT id FROM area_types WHERE name = 'Galaxy Cluster'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (2, NULL, 'Rich galaxy cluster', 'Dense galactic region', 'Vacuum', 2, '[\"Messier 87\", \"Messier 49\"]', 'A relatively young cluster, still forming and accreting galaxies.', '[]', 'Influences the motion of the Local Group.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 3. Local Group (Part of Laniakea, within the gravitational influence of Virgo Cluster, but often considered a distinct group)
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (3, 1, 1, TRUE, TRUE, 'Local Group', 'The galaxy group that includes the Milky Way and the Andromeda Galaxy.', (SELECT id FROM area_types WHERE name = 'Galaxy Group'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (3, NULL, 'The home of our galaxy', 'Three major galaxies, dozens of dwarf galaxies', 'Vacuum', 1, '[\"Milky Way\", \"Andromeda Galaxy\", \"Triangulum Galaxy\"]', 'Gravitationally bound, it is moving towards the Virgo Cluster.', '[\"Milky Way-Andromeda collision (future)\"]', 'Consists of over 50 galaxies.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 4. Milky Way Galaxy
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (4, 1, 3, TRUE, TRUE, 'Milky Way Galaxy', 'The galaxy containing our Solar System, distinguished by its spiral arms.', (SELECT id FROM area_types WHERE name = 'Galaxy'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (4, NULL, 'Our home galaxy', 'Spiral arms, central bulge, halo', 'Vacuum', 2, '[\"Sagittarius A*\", \"Orion Arm\", \"Perseus Arm\"]', 'Formed billions of years ago, actively merging with smaller galaxies.', '[\"Formation of new stars in spiral arms\"]', 'Estimated to contain 100-400 billion stars.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 5. Solar System
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (5, 1, 4, TRUE, TRUE, 'Solar System', 'The gravitationally bound system of the Sun and the objects that orbit it.', (SELECT id FROM area_types WHERE name = 'Stellar System'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (5, NULL, 'Home of Earth', 'Planets, dwarf planets, asteroids, comets, Kuiper Belt', 'Vacuum to diverse atmospheres', 1, '[\"Sun\", \"Jupiter\", \"Saturn''s Rings\"]', 'Formed about 4.6 billion years ago from a giant molecular cloud.', '[\"Formation of planets\", \"Late Heavy Bombardment\"]', 'Currently in the Orion Arm of the Milky Way.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 6. Planet Earth
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (6, 1, 5, TRUE, TRUE, 'Earth', 'The third planet from the Sun and the only astronomical object known to harbor life.', (SELECT id FROM area_types WHERE name = 'Planet'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (6, 8100000000, 'Life, blue planet', 'Diverse: continents, oceans, mountains, deserts', 'Varies globally', 1, '[\"Mount Everest\", \"Pacific Ocean\", \"Grand Canyon\"]', 'Formed 4.54 billion years ago, experienced several mass extinctions.', '[\"Industrial Revolution\", \"World Wars\", \"Space Age\"]', 'Has one natural satellite, the Moon.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 7. Continent Europe
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (7, 1, 6, TRUE, TRUE, 'Europe', 'A continent located entirely in the Northern Hemisphere and mostly in the Eastern Hemisphere.', (SELECT id FROM area_types WHERE name = 'Continent'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (7, 750000000, 'Diverse cultures, rich history', 'Plains, mountains (Alps), extensive coastlines', 'Temperate, Mediterranean, Arctic', 2, '[\"Eiffel Tower\", \"Colosseum\", \"Acropolis\"]', 'Cradle of Western civilization, site of numerous empires and conflicts.', '[\"Roman Empire\", \"Renaissance\", \"World Wars\", \"Formation of EU\"]', 'Second smallest continent by area, but third largest by population.', FALSE, '', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 8. Country Netherlands
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (8, 1, 7, TRUE, TRUE, 'Netherlands', 'A country in Northwestern Europe, known for its flat landscape, canals, tulip fields, windmills, and cycling routes.', (SELECT id FROM area_types WHERE name = 'Country'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (8, 17800000, 'Water management, windmills, cheese, art', 'Low-lying, mostly flat, polders', 'Temperate maritime', 1, '[\"Canals of Amsterdam\", \"Kinderdijk\", \"Rijksmuseum\"]', 'Golden Age of exploration and art, significant role in world trade.', '[\"80 Years'' War\", \"Dutch Golden Age\", \"Foundation of EU\"]', 'A constitutional monarchy.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/2/20/Karte_Niederlande.png', '{\"center_lat\": 52.1, \"center_lon\": 5.7}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 9. Province South Holland
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (9, 1, 8, TRUE, TRUE, 'South Holland', 'The most populous province of the Netherlands, located on the North Sea coast.', (SELECT id FROM area_types WHERE name = 'Province'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (9, 3800000, 'Ports, innovation, historical cities, flowers', 'Flat polder landscape, dunes, rivers', 'Temperate maritime', 1, '[\"Port of Rotterdam\", \"Keukenhof\", \"Binnenhof\"]', 'Historically a powerful maritime and trade region, home to major cities.', '[\"Growth of Rotterdam Port\", \"Development of horticulture\"]', 'Contains the Randstad conurbation.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/c/c5/Zuid-Holland_kaart_2015.svg/800px-Zuid-Holland_kaart_2015.svg.png', '{\"center_lat\": 52.0, \"center_lon\": 4.5}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);


            -- Municipalities of South Holland (Level: City)
            -- Note: Many municipalities are technically 'gemeenten' (municipalities)
            -- but for the purpose of our `area_types` table, 'City' at level 5 is the closest fit.
            -- Some larger municipalities might encompass several 'towns' or 'villages'.

            -- 10. Delft
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (10, 1, 9, TRUE, TRUE, 'Delft', 'A city in South Holland, Netherlands, known for its historic town centre, Delftware pottery, and its association with Johannes Vermeer.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (10, 108000, 'Delftware, Vermeer, canals, TU Delft', 'Flat, urban, canals', 'Temperate maritime', 1, '[\"Oude Kerk\", \"Nieuwe Kerk\", \"Market Square\", \"Royal Delft\"]', 'Home to the House of Orange-Nassau, important historical city.', '[]', 'The current location for this conversation.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/c/c6/Netherlands_municipality_Delft_2019.svg/800px-Netherlands_municipality_Delft_2019.svg.png', '{\"center_lat\": 52.0116, \"center_lon\": 4.3570}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 11. Rotterdam
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (11, 1, 9, TRUE, TRUE, 'Rotterdam', 'A major port city in the Dutch province of South Holland, known for its modern architecture.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (11, 650000, 'Europe''s largest port, modern architecture, Erasmus Bridge', 'Flat, delta region, heavily urbanized', 'Temperate maritime', 2, '[\"Erasmus Bridge\", \"Euromast\", \"Markthal\"]', 'Heavily bombed during WWII, rebuilt with innovative architecture.', '[\"Bombing of Rotterdam (WWII)\", \"Port expansion\"]', 'A diverse and multicultural city.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/f/f6/Netherlands_municipality_Rotterdam_2019.svg/800px-Netherlands_municipality_Rotterdam_2019.svg.png', '{\"center_lat\": 51.9225, \"center_lon\": 4.4792}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 12. The Hague (Den Haag)
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (12, 1, 9, TRUE, TRUE, 'The Hague', 'The seat of government of the Netherlands and capital of the province of South Holland.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (12, 560000, 'Government, international law, beaches', 'Coastal, urban, dunes', 'Temperate maritime', 1, '[\"Binnenhof\", \"Peace Palace\", \"Mauritshuis\", \"Scheveningen\"]', 'Historically a residential city for nobility, became the seat of government.', '[\"International Court of Justice founding\"]', 'Often called \"The Legal Capital of the World\".', TRUE, 'https://upload.wikimedia.org/wikipedia.org/commons/thumb/a/a2/Netherlands_municipality_Den_Haag_2019.svg/800px-Netherlands_municipality_Den_Haag_2019.svg.png', '{\"center_lat\": 52.0787, \"center_lon\": 4.2907}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 13. Leiden
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (13, 1, 9, TRUE, TRUE, 'Leiden', 'A city and municipality in the province of South Holland, Netherlands, known for its university and historical buildings.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (13, 127000, 'Leiden University, Rembrandt, canals', 'Flat, urban, canals', 'Temperate maritime', 1, '[\"Leiden University\", \"Burcht van Leiden\", \"Rijksmuseum van Oudheden\"]', 'Site of a Spanish siege in 1574, leading to the founding of its university.', '[\"Siege of Leiden (1574)\"]', 'A vibrant student city.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Netherlands_municipality_Leiden_2019.svg/800px-Netherlands_municipality_Leiden_2019.svg.png', '{\"center_lat\": 52.1601, \"center_lon\": 4.4970}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 14. Gouda
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (14, 1, 9, TRUE, TRUE, 'Gouda', 'A city and municipality in the province of South Holland, Netherlands, famous for its Gouda cheese, stroopwafels, and historic city center.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (14, 75000, 'Gouda cheese, stroopwafels, St. John''s Church', 'Flat, canals, urban', 'Temperate maritime', 1, '[\"Gouda Cheese Market\", \"St. John''s Church\", \"City Hall\"]', 'Developed around a fortified castle, received city rights in 1272.', '[]', 'Hosts an annual cheese market.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/1/1a/Netherlands_municipality_Gouda_2019.svg/800px-Netherlands_municipality_Gouda_2019.svg.png', '{\"center_lat\": 52.0150, \"center_lon\": 4.7083}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 15. Dordrecht
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (15, 1, 9, TRUE, TRUE, 'Dordrecht', 'The oldest city in the province of South Holland, Netherlands, located at the confluence of three rivers.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (15, 120000, 'Historic inner city, water, trade', 'River delta, urban', 'Temperate maritime', 1, '[\"Grote Kerk\", \"Groothoofd\", \"Hof van Holland\"]', 'Key role in the Dutch Golden Age, first free states assembly in 1572.', '[\"Synod of Dort (1618-1619)\"]', 'An island city surrounded by rivers.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/7/7b/Netherlands_municipality_Dordrecht_2019.svg/800px-Netherlands_municipality_Dordrecht_2019.svg.png', '{\"center_lat\": 51.8138, \"center_lon\": 4.6766}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 16. Alphen aan den Rijn
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (16, 1, 9, TRUE, TRUE, 'Alphen aan den Rijn', 'A town and municipality in the province of South Holland, located on the Oude Rijn (Old Rhine) river.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (16, 114000, 'Green Heart, bird park', 'Flat, polder, river', 'Temperate maritime', 1, '[\"Avifauna bird park\", \"Arcade bridge\"]', 'Historical Roman outpost, developed into a market town.', '[]', 'Part of the Green Heart region.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/c/cf/Netherlands_municipality_Alphen_aan_den_Rijn_2019.svg/800px-Netherlands_municipality_Alphen_aan_den_Rijn_2019.svg.png', '{\"center_lat\": 52.1245, \"center_lon\": 4.6599}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 17. Zoetermeer
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (17, 1, 9, TRUE, TRUE, 'Zoetermeer', 'A city in the province of South Holland, Netherlands, known for its modern urban planning and rapid growth.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (17, 126000, 'ICT city, modern architecture', 'Former polder, urban', 'Temperate maritime', 1, '[\"SnowWorld\", \"Dutch Water Dreams\"]', 'Transformed from a rural village into a large commuter town.', '[\"Rapid urban development in the late 20th century\"]', 'One of the youngest major cities in the Netherlands.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/9/90/Netherlands_municipality_Zoetermeer_2019.svg/800px-Netherlands_municipality_Zoetermeer_2019.svg.png', '{\"center_lat\": 52.0592, \"center_lon\": 4.4947}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 18. Westland
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (18, 1, 9, TRUE, TRUE, 'Westland', 'A municipality in the province of South Holland, Netherlands, famous for its horticulture and greenhouse industry.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (18, 114000, 'Greenhouses, horticulture, flowers', 'Flat, agricultural, coastal dunes', 'Temperate maritime', 1, '[\"Westland FloraHolland auction\"]', 'Long history of agricultural development, particularly greenhouse farming.', '[]', 'Known as the \"Greenhouse of Europe\".', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/b/b5/Netherlands_municipality_Westland_2019.svg/800px-Netherlands_municipality_Westland_2019.svg.png', '{\"center_lat\": 52.0000, \"center_lon\": 4.2000}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 19. Katwijk
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (19, 1, 9, TRUE, TRUE, 'Katwijk', 'A coastal municipality and town in the province of South Holland, Netherlands, known for its beach and fishing heritage.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (19, 66000, 'Beach, fishing, lighthouse', 'Coastal dunes, sandy beach', 'Temperate maritime', 1, '[\"Katwijk aan Zee lighthouse\"]', 'Historical fishing village, Roman settlement site.', '[]', 'Popular tourist destination in summer.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/b/b3/Netherlands_municipality_Katwijk_2019.svg/800px-Netherlands_municipality_Katwijk_2019.svg.png', '{\"center_lat\": 52.2030, \"center_lon\": 4.4070}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 20. Pijnacker-Nootdorp
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (20, 1, 9, TRUE, TRUE, 'Pijnacker-Nootdorp', 'A municipality in the province of South Holland, Netherlands, located between The Hague and Rotterdam.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (20, 57000, 'Green residential area', 'Polder landscape, suburban', 'Temperate maritime', 1, '[\"Dobbeplas recreation area\"]', 'Formed from a merger of former municipalities, experiencing growth.', '[]', 'Known for its green spaces and water recreation.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/8/87/Netherlands_municipality_Pijnacker-Nootdorp_2019.svg/800px-Netherlands_municipality_Pijnacker-Nootdorp_2019.svg.png', '{\"center_lat\": 52.0500, \"center_lon\": 4.4300}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 21. Rijswijk
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (21, 1, 9, TRUE, TRUE, 'Rijswijk', 'A municipality in the province of South Holland, Netherlands, bordering The Hague.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (21, 58000, 'Suburban, Treaty of Rijswijk', 'Urban, parks', 'Temperate maritime', 1, '[\"Huize Hofrust\", \"Museum Rijswijk\"]', 'Site of the Treaty of Rijswijk (1697) ending the Nine Years'' War.', '[\"Treaty of Rijswijk\"]', 'Close to The Hague, often considered part of its agglomeration.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/3/30/Netherlands_municipality_Rijswijk_2019.svg/800px-Netherlands_municipality_Rijswijk_2019.svg.png', '{\"center_lat\": 52.0500, \"center_lon\": 4.3300}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 22. Schiedam
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (22, 1, 9, TRUE, TRUE, 'Schiedam', 'A city and municipality in the province of South Holland, Netherlands, known for its jenever (gin) and tallest windmills.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (22, 79000, 'Jenever, windmills, historic inner city', 'Urban, canals', 'Temperate maritime', 1, '[\"De Nieuwe Palmboom\", \"Jenever Museum\"]', 'Historically a center for gin production due to its access to grain and water.', '[]', 'Part of the Rotterdam metropolitan area.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/7/7b/Netherlands_municipality_Schiedam_2019.svg/800px-Netherlands_municipality_Schiedam_2019.svg.png', '{\"center_lat\": 51.9167, \"center_lon\": 4.3833}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 23. Vlaardingen
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (23, 1, 9, TRUE, TRUE, 'Vlaardingen', 'A city and municipality in the province of South Holland, Netherlands, historically a fishing and herring industry center.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (23, 75000, 'Herring industry, historic port', 'Urban, riverine', 'Temperate maritime', 1, '[\"Grote Kerk\", \"Delta Hotel\"]', 'One of the oldest cities in Holland, important in early Dutch history.', '[\"Battle of Vlaardingen (1018)\"]', 'Part of the Rotterdam-Rijnmond metropolitan area.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/1/18/Netherlands_municipality_Vlaardingen_2019.svg/800px-Netherlands_municipality_Vlaardingen_2019.svg.png', '{\"center_lat\": 51.9069, \"center_lon\": 4.3414}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 24. Capelle aan den IJssel
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (24, 1, 9, TRUE, TRUE, 'Capelle aan den IJssel', 'A municipality in the province of South Holland, Netherlands, located on the river IJssel.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (24, 67000, 'Residential, suburban', 'Urban, riverine, polder', 'Temperate maritime', 1, '[\"Hitland recreation area\"]', 'Developed from a small settlement into a commuter town for Rotterdam.', '[]', 'Known for its shopping center and green spaces.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/c/ca/Netherlands_municipality_Capelle_aan_den_IJssel_2019.svg/800px-Netherlands_municipality_Capelle_aan_den_IJssel_2019.svg.png', '{\"center_lat\": 51.9333, \"center_lon\": 4.5833}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- 25. Barendrecht
            INSERT INTO locations (id, world_id, parent_id, has_parent, has_children, name, description, area_type_id, created_at, updated_at) VALUES
            (25, 1, 9, TRUE, TRUE, 'Barendrecht', 'A municipality in the province of South Holland, Netherlands, south of Rotterdam.', (SELECT id FROM area_types WHERE name = 'City'), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            INSERT INTO location_details (location_id, population, known_for, terrain, climate, danger_level, notable_landmarks, history, major_events, notes, has_map, map_image_url, map_location, created_at, updated_at) VALUES
            (25, 48000, 'Residential, logistics hub', 'Polder, suburban', 'Temperate maritime', 1, '[\"Carnisselande shopping center\"]', 'A historically agricultural area that became a residential and logistics hub.', '[]', 'Part of the Rotterdam-Rijnmond metropolitan area.', TRUE, 'https://upload.wikimedia.org/wikipedia/commons/thumb/3/36/Netherlands_municipality_Barendrecht_2019.svg/800px-Netherlands_municipality_Barendrecht_2019.svg.png', '{\"center_lat\": 51.8500, \"center_lon\": 4.4833}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

            -- You can continue adding more municipalities using the same pattern (e.g., Krimpenerwaard, Lansingerland, Maassluis, Midden-Delfland, Nissewaard, Papendrecht, Ridderkerk, Sliedrecht, Spijkenisse, Voorne aan Zee, Waddinxveen, Zederik, Zuidplas, etc.)
            -- Ensure you update the 'id' sequentially and reference the 'parent_id' correctly (which will be 9 for all these municipalities of South Holland).

            -- Final note on has_parent/has_children flags:
            -- For 'Room' level locations, has_children would be FALSE.
            -- For 'Supercluster' or the highest defined level, has_parent would be FALSE.
            -- For intermediate levels, both are typically TRUE as they contain smaller things and are contained by larger things.


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

            -- area_types:
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