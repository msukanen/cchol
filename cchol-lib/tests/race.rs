mod race_tests {
    use cchol_lib::{IsNamed, racial::race::{RACE_DEFAULT, Race}, social::culture::Culture};

    #[test]
    fn we_have_a_winner() {
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
}