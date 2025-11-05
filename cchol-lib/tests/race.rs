mod race_tests {
    use rpgassist::ext::IsNamed;
    use cchol_lib::{racial::race::{RACE_DEFAULT, Race}, social::culture::Culture};

    #[test]
    fn race_data_integrity() {
        let r = *RACE_DEFAULT;
        assert_eq!(r.name(), "human");
    }

    #[test]
    fn shift_nomad_down() {
        let _ = env_logger::try_init();
        let r = Race::from("reptileman");
        let c = Culture::from("nomad");
        let c = r.shift_culture_if_needed(&c);
        assert!(!c.is_nomad());
    }

    #[test]
    fn shift_civilized_up() {
        let _ = env_logger::try_init();
        let r = Race::from("reptileman");
        let c = Culture::from("civilized");
        let c = r.shift_culture_if_needed(c);
        assert!(!c.is_civilized());
    }

    #[test]
    fn culture_cap() {
        let _ = env_logger::try_init();
        let r = Race::from("dwarf");
        let c = Culture::from("decadent");
        // this should peg the Decadent culture down a step
        let c = r.shift_culture_if_needed(c);
        assert!(c.is_civilized())
    }

    mod racial_event_tests {
        use cchol_lib::events::RacialEvent;

        use super::*;

        #[test]
        fn human_has_no_special_events() {
            let r = Race::from("human");
            let e = r.has_racial_events(true);
            assert!(e.is_none());
        }

        #[test]
        fn nonhumans_have_special_events() {
            let nhs: [(&str,RacialEvent);4] = [
                ("elf", RacialEvent::Elf),
                ("dwarf", RacialEvent::Dwarf),
                ("halfling", RacialEvent::Halfling),
                ("orc", RacialEvent::Monster)];
            nhs.iter().for_each(|(r, e)| {
                let r = Race::from(r);
                let evt = r.has_racial_events(false);
                let Some(evt) = evt else {
                    panic!("No racial event for '{}'?!", r.name())
                };
                assert_eq!(*e, evt);
            });
        }
    }
}