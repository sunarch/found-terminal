// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use rand::random;
use rand::Rng;
use rand::thread_rng;

// project
use crate::category;
use crate::terminalisp::station as tl_station;
use crate::terminalisp::menu::tli_menu;

// module
use crate::station::name::StationName;
use crate::station::components::{Name, SectionCounts, ModuleCounts,
                                 UpdateModules, Status, BreakSomething, Repair, PowerDown};


pub trait SectionGroups {
    const SECTION_GROUPS: u8;
}

pub struct Station {
    _name: StationName,
    pub version: u8,
    pub mission_day: u16,
    pub disabled: bool,

    pub sections_comm: category::comms::CommsCategory,
    pub sections_crew: category::crew::CrewCategory,
    pub sections_maneuver: category::maneuver::ManeuverCategory,
    pub sections_misc: category::misc::MiscCategory,
    pub sections_power: category::power::PowerCategory,
    pub sections_research: category::research::ResearchCategory,

    _total_sections: u16,
    _installed_sections: u16,
    _total_modules: u16,
    _active_modules: u16,
}

impl SectionGroups for Station {
    const SECTION_GROUPS: u8 = 6;
}

impl Station {
    pub fn new() -> Self {
        let mut station = Station {
            _name: random(),
            version: random(),
            mission_day: 0,
            disabled: false,

            sections_comm: category::comms::CommsCategory::new(1, 100),
            sections_crew: category::crew::CrewCategory::new(0, 100),
            sections_maneuver: category::maneuver::ManeuverCategory::new(1, 100),
            sections_misc: category::misc::MiscCategory::new(1, 100),
            sections_power: category::power::PowerCategory::new(1, 100),
            sections_research: category::research::ResearchCategory::new(0, 100),

            _total_sections: 0,
            _installed_sections: 0,
            _total_modules: 0,
            _active_modules: 0,
        };

        station._total_sections =
            station.sections_comm.total_sections() +
            station.sections_crew.total_sections() +
            station.sections_maneuver.total_sections() +
            station.sections_misc.total_sections() +
            station.sections_power.total_sections() +
            station.sections_research.total_sections();

        station._installed_sections =
            station.sections_comm.installed_sections() +
            station.sections_crew.installed_sections() +
            station.sections_maneuver.installed_sections() +
            station.sections_misc.installed_sections() +
            station.sections_power.installed_sections() +
            station.sections_research.installed_sections();

        station._total_modules =
            station.sections_comm.total_modules() +
            station.sections_crew.total_modules() +
            station.sections_maneuver.total_modules() +
            station.sections_misc.total_modules() +
            station.sections_power.total_modules() +
            station.sections_research.total_modules();

        station.update_active_modules();

        station.status(0, true, false);

        return station;
    }

    fn days_left(&self) -> u16 { self.active_modules() }

    pub fn is_shut_down(&self) -> bool {
        return self.days_left() < {1 as u16};
    }

    pub fn new_day(&mut self) {
        if self.disabled {
            return;
        }

        if self.active_modules() == 0 {
            tl_station::end_transmission();
            self.disabled = true;
            return;
        }

        self.increment_mission_day();

        self.status(0, false, false);
        self.break_something();
        tl_station::until_final_transmission(self.days_left() as u16);
    }

    pub fn science(&mut self) {
        self.break_something();
    }

    pub fn name_display(&self) -> String {
        return format!("Station \"{}\" v{}", &self.name(), &self.version);
    }

    pub fn mission_day_display(&self) -> String {
        let mission_day = self.mission_day;
        return format!("Mission Day {mission_day}");
    }

    fn increment_mission_day(&mut self) {
        self.mission_day = self.mission_day.saturating_add(1);
    }
}

impl Name for Station { fn name(&self) -> String { self._name.to_string() } }
impl SectionCounts for Station {
    fn total_sections(&self) -> u16 { self._total_sections }
    fn installed_sections(&self) -> u16 { self._installed_sections }
}
impl ModuleCounts for Station {
    fn total_modules(&self) -> u16 { self._total_modules }
    fn active_modules(&self) -> u16 { self._active_modules }
}

impl UpdateModules for Station {
    fn active_module_counts(&self) -> Vec<u16> {
        vec![
            self.sections_comm.active_modules(),
            self.sections_crew.active_modules(),
            self.sections_maneuver.active_modules(),
            self.sections_misc.active_modules(),
            self.sections_power.active_modules(),
            self.sections_research.active_modules()
        ]
    }

    fn update_active_modules(&mut self) {
        self._active_modules = self.active_module_sum();
    }
}

impl Station {
    pub fn status(&self, indent: u8, show_fields: bool, show_inner: bool) -> String {
        tl_station::status(
            String::from("station"),
            show_fields,
            vec![
                String::from(":name"),
                String::from(":version"),
                String::from(":mission-day"),
                String::from(":total-modules"),
                String::from(":active-modules")
            ],
            vec![
                format!("\"{}\"", self.name()),
                format!("{}", self.version),
                format!("{}", self.mission_day),
                format!("{}", self.total_modules()),
                format!("{}", self.active_modules())
            ],
            show_inner,
            String::from(":categories"),
            vec![
                self.sections_comm.status(indent + 2),
                self.sections_crew.status(indent + 2),
                self.sections_maneuver.status(indent + 2),
                self.sections_misc.status(indent + 2),
                self.sections_power.status(indent + 2),
                self.sections_research.status(indent + 2)
            ],
            indent
        )
    }
}

impl Station {
    fn break_something(&mut self) {
        let broken_module: Result<String, String>;

        match thread_rng().gen_range(1..=6) {
            1 => { broken_module = self.sections_comm.break_something(); },
            2 => { broken_module = self.sections_crew.break_something(); },
            3 => { broken_module = self.sections_maneuver.break_something(); },
            4 => { broken_module = self.sections_misc.break_something(); },
            5 => { broken_module = self.sections_power.break_something(); },
            6 => { broken_module = self.sections_research.break_something(); },
            _ => unreachable!(),
        }

        self.update_active_modules();

        match broken_module {
            Ok(v) => {
                self.update_active_modules();
                tl_station::section_failure(v);
            },
            Err(_) => {
                if self.active_modules() == self.total_modules() {
                    tl_station::sections_ok();
                }
            },
        }
    }
}

impl Repair for Station {
    fn repairable(&self) -> bool {
        self.active_modules() < self.total_modules()
    }

    fn repair(&mut self) {
        let prompts: Vec<String> = vec![
            self.sections_comm.repair_display(),
            self.sections_crew.repair_display(),
            self.sections_maneuver.repair_display(),
            self.sections_misc.repair_display(),
            self.sections_power.repair_display(),
            self.sections_research.repair_display(),
        ];

        let mut options:Vec<String> = vec![];
        if self.sections_comm.repairable()     { options.push(prompts[0].clone()) }
        if self.sections_crew.repairable()     { options.push(prompts[1].clone()) }
        if self.sections_maneuver.repairable() { options.push(prompts[2].clone()) }
        if self.sections_misc.repairable()     { options.push(prompts[3].clone()) }
        if self.sections_power.repairable()    { options.push(prompts[4].clone()) }
        if self.sections_research.repairable() { options.push(prompts[5].clone()) }

        let chosen: String;
        match tli_menu("Select section category to repair:", options) {
            Ok(v) => { chosen = v; },
            Err(_) => { return; }
        }
        match chosen {
            _ if chosen == prompts[0] => { self.sections_comm.repair(); },
            _ if chosen == prompts[1] => { self.sections_crew.repair(); },
            _ if chosen == prompts[2] => { self.sections_maneuver.repair(); },
            _ if chosen == prompts[3] => { self.sections_misc.repair(); },
            _ if chosen == prompts[4] => { self.sections_power.repair(); },
            _ if chosen == prompts[5] => { self.sections_research.repair(); },
            _ => unreachable!()
        }

        self.update_active_modules();
    }
}

impl PowerDown for Station {
    fn power_down(&mut self) {
        self.sections_comm.power_down();
        self.sections_crew.power_down();
        self.sections_maneuver.power_down();
        self.sections_misc.power_down();
        self.sections_power.power_down();
        self.sections_research.power_down();

        self.update_active_modules();
    }
}
