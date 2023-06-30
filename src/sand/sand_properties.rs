use super::Sand;
use std::collections::{HashMap, HashSet};

pub struct SandProperties {
    pub can_replace: HashSet<Sand>,
    pub replace_with: HashMap<Sand, Sand>,
    pub can_sink_in: HashSet<Sand>,
}

pub struct SandSimulationProperties(HashMap<Sand, SandProperties>);

impl SandProperties {
    pub fn empty() -> Self {
        SandProperties {
            can_replace: HashSet::<Sand>::new(),
            replace_with: HashMap::<Sand, Sand>::new(),
            can_sink_in: HashSet::<Sand>::new(),
        }
    }

    pub fn from_vecs(
        can_replace: Option<Vec<Sand>>,
        replace_with: Option<Vec<(Sand, Sand)>>,
        can_sink_in: Option<Vec<Sand>>,
    ) -> Self {
        let mut properties = Self::empty();

        properties.add_replaceable(Sand::Air);

        match can_replace {
            Some(can_replace) => {
                can_replace
                    .into_iter()
                    .for_each(|sand| properties.add_replaceable(sand));
            }
            _ => {}
        }

        match replace_with {
            Some(replace_with) => {
                replace_with
                    .into_iter()
                    .for_each(|(sand1, sand2)| properties.add_replace_with(sand1, sand2));
            }
            _ => {}
        }

        match can_sink_in {
            Some(can_sink_in) => {
                can_sink_in
                    .into_iter()
                    .for_each(|sand| properties.add_can_sink_in(sand));
            }
            _ => {}
        }

        properties
    }

    pub fn add_replaceable(&mut self, sand: Sand) {
        self.can_replace.insert(sand);
    }

    pub fn add_replace_with(&mut self, can_replace: Sand, replace_with: Sand) {
        self.replace_with.insert(can_replace, replace_with);
    }

    pub fn add_can_sink_in(&mut self, sand: Sand) {
        self.can_sink_in.insert(sand);
    }

    pub fn replace(&self, sand: Sand, sand_to_replace: Sand) -> Sand {
        match self.replace_with.get(&sand_to_replace) {
            Some(s) => *s,
            _ => sand,
        }
    }
}

impl SandSimulationProperties {
    pub fn new() -> Self {
        Self(HashMap::<Sand, SandProperties>::new())
    }

    pub fn simulation_sand_properties() -> Self {
        let mut sand_sim_properties = Self::new();

        //Sand
        {
            let can_replace = vec![Sand::Fire];
            let replace_with = vec![(Sand::Acid, Sand::Acid)];
            let can_sink_in = vec![Sand::Water, Sand::Oil, Sand::Acid];

            let sand_property =
                SandProperties::from_vecs(Some(can_replace), Some(replace_with), Some(can_sink_in));
            sand_sim_properties.add_sand_property(Sand::Sand, sand_property);
        }

        //Water
        {
            let can_replace = vec![Sand::Fire, Sand::Lava];
            let replace_with = vec![(Sand::Lava, Sand::Stone)];
            let can_sink_in = vec![Sand::Oil];

            let sand_property =
                SandProperties::from_vecs(Some(can_replace), Some(replace_with), Some(can_sink_in));
            sand_sim_properties.add_sand_property(Sand::Water, sand_property);
        }

        //Wall
        {
            let sand_property = SandProperties::from_vecs(None, None, None);
            sand_sim_properties.add_sand_property(Sand::Wall, sand_property);
        }

        //Wood
        {
            let sand_property = SandProperties::from_vecs(None, None, None);
            sand_sim_properties.add_sand_property(Sand::Wood, sand_property);
        }

        //Fire
        {
            let can_replace = vec![Sand::Oil, Sand::Wood];
            let sand_property = SandProperties::from_vecs(Some(can_replace), None, None);
            sand_sim_properties.add_sand_property(Sand::Fire, sand_property);
        }

        //Oil
        {
            let can_replace = vec![Sand::Fire];
            let sand_property = SandProperties::from_vecs(Some(can_replace), None, None);
            sand_sim_properties.add_sand_property(Sand::Oil, sand_property);
        }

        //Acid
        {
            let can_sink_in = vec![Sand::Water, Sand::Oil];
            let can_replace = vec![Sand::Wood, Sand::Sand, Sand::Fire, Sand::Stone];

            let replace_with = vec![
                (Sand::Wood, Sand::Air),
                (Sand::Sand, Sand::Air),
                (Sand::Fire, Sand::Air),
                (Sand::Stone, Sand::Air),
            ];

            let sand_property =
                SandProperties::from_vecs(Some(can_replace), Some(replace_with), Some(can_sink_in));
            sand_sim_properties.add_sand_property(Sand::Acid, sand_property);
        }

        //Lava
        {
            let can_replace = vec![Sand::Water];
            let replace_with = vec![(Sand::Water, Sand::Stone)];
            let can_sink_in = vec![Sand::Water, Sand::Acid, Sand::Oil];

            let sand_property =
                SandProperties::from_vecs(Some(can_replace), Some(replace_with), Some(can_sink_in));
            sand_sim_properties.add_sand_property(Sand::Lava, sand_property);
        }

        //Stone
        {
            let can_sink_in = vec![Sand::Oil, Sand::Water, Sand::Acid];
            let replace_with = vec![(Sand::Acid, Sand::Acid)];

            let sand_property =
                SandProperties::from_vecs(None, Some(replace_with), Some(can_sink_in));
            sand_sim_properties.add_sand_property(Sand::Stone, sand_property);
        }

        //Explosive
        {
            let can_replace = vec![Sand::Fire];
            let can_sink_in = vec![Sand::Oil, Sand::Water, Sand::Acid];

            let sand_property =
                SandProperties::from_vecs(Some(can_replace), None, Some(can_sink_in));
            sand_sim_properties.add_sand_property(Sand::Explosive, sand_property);
        }

        //Explosion
        {
            let can_replace = vec![
                Sand::Water,
                Sand::Wood,
                Sand::Fire,
                Sand::Sand,
                Sand::Explosive,
                Sand::Lava,
                Sand::Oil,
                Sand::Acid,
            ];

            let explosion_property = SandProperties::from_vecs(Some(can_replace), None, None);
            sand_sim_properties.add_sand_property(Sand::Explosion, explosion_property);
        }

        sand_sim_properties
    }

    pub fn get_sand_property(&self, s: Sand) -> Option<&SandProperties> {
        self.0.get(&s)
    }

    pub fn add_sand_property(&mut self, s: Sand, properties: SandProperties) {
        self.0.insert(s, properties);
    }
}
