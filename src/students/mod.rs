use std::{cmp::Ordering, collections::HashMap};

pub mod io;

#[derive(Debug, Default)]
pub struct Studente {
    pub nome: Option<String>,
    pub matricola: u32,
    pub voti: HashMap<String, Voto>,
}

impl Studente {
    fn get_score(&self) -> u64 {
        let mut acc = 0;
        for voto in self.voti.values() {
            if voto.voto != 0 && voto.voto != 1 {
                acc += voto.voto as u64 * voto.materia.cfu as u64;
            }
        }
        acc
    }
}

impl PartialOrd for Studente {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for Studente {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.matricola == other.matricola {
            return Ordering::Equal;
        }
        if self.get_score() >= other.get_score() {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}
impl PartialEq for Studente {
    fn eq(&self, other: &Self) -> bool {
        self.matricola == other.matricola
    }
}
impl Eq for Studente {}

pub struct InvalidStudent;

impl TryFrom<(&str, &str)> for Studente {
    type Error = InvalidStudent;

    fn try_from((value, path): (&str, &str)) -> Result<Self, Self::Error> {
        let (materia, cfu) = path.split_once("_").ok_or(InvalidStudent)?;
        let (matricola, voto) = value.split_once(",").ok_or(InvalidStudent)?;
        let matricola = matricola.parse().or(Err(InvalidStudent))?;
        let voto = voto.parse().or(Err(InvalidStudent))?;
        return Ok(Studente {
            nome: None,
            matricola,
            voti: HashMap::from([(
                materia.to_owned(),
                Voto {
                    materia: Materia {
                        nome: materia.to_owned(),
                        cfu: cfu.parse().or(Err(InvalidStudent))?,
                    },
                    voto,
                },
            )]),
        });
    }
}

#[derive(Debug)]
pub struct Voto {
    materia: Materia,
    voto: u8,
}

impl ToString for Voto {
    fn to_string(&self) -> String {
        format!("{}", self.voto)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Materia {
    nome: String,
    cfu: u8,
}
